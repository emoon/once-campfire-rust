use std::fmt::Write;

use rusqlite::types::{FromSql, ToSql, Value};
use rusqlite::{Connection, Params, Row, Statement, params_from_iter};

use crate::database::Tx;
use crate::error::Result;
use crate::patch::Patch;
use crate::time::Timestamp;

/// `Connection::execute` and `Connection::query_row` through the connection's statement cache,
/// so a query is compiled once per connection rather than on every call (Active Record keeps a
/// prepared-statement cache per connection too).
pub trait CachedStatements {
    fn execute_cached(&self, sql: &str, params: impl Params) -> rusqlite::Result<usize>;

    fn query_row_cached<T>(&self, sql: &str, params: impl Params, map: impl FnOnce(&Row<'_>) -> rusqlite::Result<T>)
    -> rusqlite::Result<T>;
}

impl CachedStatements for Connection {
    fn execute_cached(&self, sql: &str, params: impl Params) -> rusqlite::Result<usize> {
        self.prepare_cached(sql)?.execute(params)
    }

    fn query_row_cached<T>(
        &self,
        sql: &str,
        params: impl Params,
        map: impl FnOnce(&Row<'_>) -> rusqlite::Result<T>,
    ) -> rusqlite::Result<T> {
        self.prepare_cached(sql)?.query_row(params, map)
    }
}

/// The `SET` list of an `UPDATE`. Like Active Record's partial writes, it holds only the attributes
/// that changed, and [`Assignments::write`] touches `updated_at` only when there is one.
#[derive(Debug, Default)]
pub struct Assignments(Vec<(&'static str, Value)>);

impl Assignments {
    /// Assigns `value` to the attribute, and writes it if that changed the attribute.
    pub fn change<T: PartialEq + Clone + Into<Value>>(&mut self, column: &'static str, attribute: &mut T, value: T) {
        if *attribute != value {
            self.set(column, value.clone());
            *attribute = value;
        }
    }

    /// [`Assignments::change`] for a nullable attribute.
    pub fn patch<T: PartialEq + Clone + Into<Value>>(&mut self, column: &'static str, attribute: &mut Option<T>, patch: Patch<T>) {
        match patch {
            Patch::Keep => {}
            Patch::Set(value) => self.change(column, attribute, Some(value)),
            Patch::Clear => self.change(column, attribute, None),
        }
    }

    /// Writes the column whether or not its value changed.
    pub fn set(&mut self, column: &'static str, value: impl Into<Value>) {
        self.0.push((column, value.into()));
    }

    /// `UPDATE "<table>" SET … WHERE "<table>"."id" = ?`, setting `updated_at` to now. Writes
    /// nothing, and leaves `updated_at` alone, when there is nothing to assign.
    pub fn write(self, tx: &Tx<'_>, table: &'static str, id: i64, updated_at: &mut Timestamp) -> Result<()> {
        if self.0.is_empty() {
            return Ok(());
        }
        *updated_at = tx.now();
        // The fixed text is 48 bytes; each column adds its name plus 8 (`"" = ?, `).
        let columns: usize = self.0.iter().map(|(column, _)| column.len() + 8).sum();
        let mut sql = String::with_capacity(48 + 2 * table.len() + columns);
        write!(sql, r#"UPDATE "{table}" SET "#).unwrap();
        for (column, _) in &self.0 {
            write!(sql, r#""{column}" = ?, "#).unwrap();
        }
        write!(sql, r#""updated_at" = ? WHERE "{table}"."id" = ?"#).unwrap();
        let values = self.0.iter().map(|(_, value)| value as &dyn ToSql);
        let params = values.chain([&*updated_at as &dyn ToSql, &id]);
        tx.conn().execute_cached(&sql, params_from_iter(params))?;
        Ok(())
    }
}

/// `?, ?, ?`
pub fn placeholders(n: usize) -> String {
    vec!["?"; n].join(", ")
}

/// A model's column positions in one statement's result, looked up by name once per query (at
/// its first row) and then read by index in `from_row`. rusqlite's `Row::get(&str)` rescans the
/// statement's column names on every call, which was ~6% of the read routes (S-8). The positions
/// can't be constants: `SELECT *` column order depends on the install's migration history.
pub trait Columns: Sized {
    /// Finds each column by its name.
    fn resolve(stmt: &Statement<'_>) -> rusqlite::Result<Self> {
        Self::prefixed(stmt, "")
    }

    /// Finds each column by `prefix` and its name, among a joined table's aliased columns
    /// (`"rooms"."id" AS r_id`).
    fn prefixed(stmt: &Statement<'_>, prefix: &str) -> rusqlite::Result<Self>;
}

/// Declares a model's [`Columns`]: a field per attribute, holding the position of the column with
/// the field's name, or with the name it gives (`room_type = "type"`).
macro_rules! columns {
    ($(#[$attr:meta])* $vis:vis struct $name:ident { $($field:ident $(= $column:literal)?),* $(,)? }) => {
        $(#[$attr])*
        $vis struct $name {
            $($field: usize),*
        }

        impl $crate::sql::Columns for $name {
            fn prefixed(stmt: &rusqlite::Statement<'_>, prefix: &str) -> rusqlite::Result<Self> {
                Ok(Self { $($field: $crate::sql::column_index(stmt, prefix, $crate::sql::columns!(@name $field $($column)?))?),* })
            }
        }
    };
    (@name $field:ident) => { stringify!($field) };
    (@name $field:ident $column:literal) => { $column };
}
pub(crate) use columns;

/// `Statement::column_index` for `prefix` followed by `name`, compared the same way: ASCII case
/// folded, first match.
pub fn column_index(stmt: &Statement<'_>, prefix: &str, name: &str) -> rusqlite::Result<usize> {
    let matches = |column: &str| {
        let (column, len) = (column.as_bytes(), prefix.len());
        column.len() == len + name.len()
            && column[..len].eq_ignore_ascii_case(prefix.as_bytes())
            && column[len..].eq_ignore_ascii_case(name.as_bytes())
    };
    (0..stmt.column_count())
        .find(|&i| stmt.column_name(i).is_ok_and(matches))
        .ok_or_else(|| rusqlite::Error::InvalidColumnName(format!("{prefix}{name}")))
}

/// Every row of `sql`, decoded by `from_row` with the columns found at the first row (after the
/// first step, so a statement SQLite re-prepared after a schema change reports its new columns).
pub fn query_all<T, C: Columns>(
    conn: &Connection,
    sql: &str,
    params: impl Params,
    mut from_row: impl FnMut(&Row<'_>, &C) -> rusqlite::Result<T>,
) -> Result<Vec<T>> {
    let mut stmt = conn.prepare_cached(sql)?;
    let mut columns = None;
    let rows = stmt.query_map(params, |row| {
        let columns = match &columns {
            Some(columns) => columns,
            None => columns.insert(C::resolve(row.as_ref())?),
        };
        from_row(row, columns)
    })?;
    Ok(rows.collect::<rusqlite::Result<Vec<T>>>()?)
}

/// The first row of `sql`, if any, decoded by `from_row`.
pub fn query_one<T, C: Columns>(
    conn: &Connection,
    sql: &str,
    params: impl Params,
    from_row: impl FnOnce(&Row<'_>, &C) -> rusqlite::Result<T>,
) -> Result<Option<T>> {
    use rusqlite::OptionalExtension;
    let mut stmt = conn.prepare_cached(sql)?;
    Ok(stmt.query_row(params, |row| from_row(row, &C::resolve(row.as_ref())?)).optional()?)
}

/// `pluck`: the first column of every row.
pub fn pluck<T: FromSql>(conn: &Connection, sql: &str, params: impl Params) -> Result<Vec<T>> {
    let mut stmt = conn.prepare_cached(sql)?;
    let rows = stmt.query_map(params, |row| row.get(0))?;
    Ok(rows.collect::<rusqlite::Result<Vec<T>>>()?)
}

pub fn count(conn: &Connection, sql: &str, params: impl Params) -> Result<i64> {
    Ok(conn.prepare_cached(sql)?.query_row(params, |r| r.get(0))?)
}

pub fn exists(conn: &Connection, sql: &str, params: impl Params) -> Result<bool> {
    Ok(conn.prepare_cached(sql)?.exists(params)?)
}

/// `SecureRandom.alphanumeric(n)`
pub fn alphanumeric(n: usize) -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    (0..n).map(|_| CHARS[rng.random_range(0..CHARS.len())] as char).collect()
}

/// `SecureRandom.base58(n)`, as `has_secure_token` uses it.
pub fn base58(n: usize) -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut rng = rand::rng();
    (0..n).map(|_| CHARS[rng.random_range(0..CHARS.len())] as char).collect()
}

/// `SecureRandom.uuid` / `Random.uuid`
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_index_finds_a_prefixed_name_ignoring_ascii_case() {
        let conn = Connection::open_in_memory().unwrap();
        let stmt = conn.prepare(r#"SELECT 1 AS id, 2 AS "R_Id", 3 AS r_name"#).unwrap();
        assert_eq!(column_index(&stmt, "", "id").unwrap(), 0);
        assert_eq!(column_index(&stmt, "r_", "id").unwrap(), 1);
        assert_eq!(column_index(&stmt, "r_", "NAME").unwrap(), 2);
        assert!(matches!(column_index(&stmt, "r_", "type"), Err(rusqlite::Error::InvalidColumnName(name)) if name == "r_type"));
    }
}
