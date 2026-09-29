//! `reference/app/models/webhook.rb`: the row and the payload. Delivery (HTTP, replies)
//! lives in the app.

use rails_compat::json;
use rusqlite::{Connection, Row, params};
use serde::Serialize;

use crate::database::Tx;
use crate::error::Result;
use crate::models::{Message, Room, User};
use crate::rich_text::RichText;
use crate::sql::{CachedStatements, query_one};
use crate::time::Timestamp;

/// `Webhook::ENDPOINT_TIMEOUT`
pub const ENDPOINT_TIMEOUT_SECONDS: u64 = 7;

#[derive(Debug, Clone, PartialEq)]
pub struct Webhook {
    pub id: i64,
    pub user_id: i64,
    pub url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Webhook {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            user_id: row.get("user_id")?,
            url: row.get("url")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    pub fn find_by_user(conn: &Connection, user_id: i64) -> Result<Option<Self>> {
        query_one(
            conn,
            r#"SELECT "webhooks".* FROM "webhooks" WHERE "webhooks"."user_id" = ? LIMIT 1"#,
            [user_id],
            Self::from_row,
        )
    }

    /// `create_webhook!(url:)`
    pub fn create(tx: &Tx<'_>, user_id: i64, url: Option<&str>) -> Result<Self> {
        let now = tx.now();
        let id: i64 = tx.conn().query_row_cached(
            r#"INSERT INTO "webhooks" ("created_at", "updated_at", "url", "user_id") VALUES (?, ?, ?, ?) RETURNING "id""#,
            params![now, now, url, user_id],
            |r| r.get(0),
        )?;
        Ok(Self {
            id,
            user_id,
            url: url.map(Into::into),
            created_at: now,
            updated_at: now,
        })
    }

    /// `webhook.update!(url:)`
    pub fn update_url(&mut self, tx: &Tx<'_>, url: &str) -> Result<()> {
        if self.url.as_deref() == Some(url) {
            return Ok(());
        }
        let now = tx.now();
        tx.conn().execute_cached(
            r#"UPDATE "webhooks" SET "updated_at" = ?, "url" = ? WHERE "webhooks"."id" = ?"#,
            params![now, url, self.id],
        )?;
        self.url = Some(url.into());
        self.updated_at = now;
        Ok(())
    }

    pub fn destroy(&self, tx: &Tx<'_>) -> Result<()> {
        tx.conn()
            .execute_cached(r#"DELETE FROM "webhooks" WHERE "webhooks"."id" = ?"#, [self.id])?;
        Ok(())
    }

    /// The JSON body `deliver` posts. `room_bot_messages_path` and `message_path` come from
    /// the route helpers (`room_bot_messages_path(room, bot_key)`, `room_at_message_path(room, message)`).
    pub fn payload(
        &self,
        conn: &Connection,
        rich_text: &dyn RichText,
        message: &Message,
        room_bot_messages_path: &str,
        message_path: &str,
    ) -> Result<String> {
        let creator = message.creator(conn)?;
        let room = Room::find(conn, message.room_id)?;
        let recipient = User::find(conn, self.user_id)?;
        let html = message.body_html(conn)?;
        let plain = without_recipient_mentions(&message.plain_text_body(conn, rich_text)?, &recipient);

        let payload = Payload {
            user: PayloadUser {
                id: creator.id,
                name: &creator.name,
            },
            room: PayloadRoom {
                id: room.id,
                name: room.name.as_deref(),
                path: room_bot_messages_path,
            },
            message: PayloadMessage {
                id: message.id,
                body: PayloadBody {
                    html: html.as_deref(),
                    plain: &plain,
                },
                path: message_path,
            },
        };
        Ok(json::encode(&payload).into_string())
    }
}

// `Webhook#payload`'s hash, fields in its order.
#[derive(Serialize)]
struct Payload<'a> {
    user: PayloadUser<'a>,
    room: PayloadRoom<'a>,
    message: PayloadMessage<'a>,
}

#[derive(Serialize)]
struct PayloadUser<'a> {
    id: i64,
    name: &'a str,
}

#[derive(Serialize)]
struct PayloadRoom<'a> {
    id: i64,
    name: Option<&'a str>,
    path: &'a str,
}

#[derive(Serialize)]
struct PayloadMessage<'a> {
    id: i64,
    body: PayloadBody<'a>,
    path: &'a str,
}

#[derive(Serialize)]
struct PayloadBody<'a> {
    html: Option<&'a str>,
    plain: &'a str,
}

/// Removes `@Recipient` mentions and leading/trailing (Unicode) whitespace.
fn without_recipient_mentions(body: &str, recipient: &User) -> String {
    body.replace(&recipient.attachable_plain_text_representation(), "")
        .trim_matches(char::is_whitespace)
        .to_string()
}
