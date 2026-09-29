use std::fmt::Write;

use rusqlite::types::{ToSql, Value};
use rusqlite::{Connection, Params, Row, params_from_iter};

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

pub fn query_all<T>(conn: &Connection, sql: &str, params: impl Params, map: impl FnMut(&Row<'_>) -> rusqlite::Result<T>) -> Result<Vec<T>> {
    let mut stmt = conn.prepare_cached(sql)?;
    let rows = stmt.query_map(params, map)?;
    Ok(rows.collect::<rusqlite::Result<Vec<T>>>()?)
}

pub fn query_one<T>(
    conn: &Connection,
    sql: &str,
    params: impl Params,
    map: impl FnOnce(&Row<'_>) -> rusqlite::Result<T>,
) -> Result<Option<T>> {
    use rusqlite::OptionalExtension;
    let mut stmt = conn.prepare_cached(sql)?;
    Ok(stmt.query_row(params, map).optional()?)
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
