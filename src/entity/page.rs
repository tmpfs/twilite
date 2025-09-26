use crate::{
    error::ServerError,
    helpers::{html_to_text, sanitize_html},
};
use async_sqlite::{Client, Error::Rusqlite, rusqlite};
use axum::body::Bytes;
use sql_query_builder as sql;
use time::{UtcDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

/// Upload for a page.
pub struct PageUpload(pub String, pub String, pub Bytes);

pub struct PageEntity {
    pub page_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub page_uuid: Uuid,
    pub page_name: String,
    pub page_content: String,
    pub page_text: String,
}

impl PageEntity {
    pub async fn add(
        client: &Client,
        page_name: String,
        page_content: String,
        uploads: Vec<PageUpload>,
    ) -> Result<(), ServerError> {
        let query = sql::Insert::new()
            .insert_into(
                "pages (created_at, updated_at, page_uuid, page_name, page_content, page_text)",
            )
            .values("(?1, ?2, ?3, ?4, ?5, ?6)");

        let now = UtcDateTime::now();
        let created_at = now.format(&Rfc3339)?;
        let updated_at = now.format(&Rfc3339)?;
        let page_uuid = Uuid::new_v4();
        let page_content = sanitize_html(&page_content);
        let page_text = html_to_text(&page_content);
        match client
            .conn_mut(move |conn| {
                let tx = conn.transaction()?;
                tx.execute(
                    &query.as_string(),
                    (
                        created_at.clone(),
                        updated_at.clone(),
                        page_uuid.to_string(),
                        page_name,
                        page_content,
                        page_text,
                    ),
                )?;

                let page_id = tx.last_insert_rowid();

                for upload in uploads {
                    let file_uuid = Uuid::new_v4();

                    let query = sql::Insert::new()
                        .insert_into(
                            "files (created_at, updated_at, file_uuid, file_name, content_type, file_content)",
                        )
                        .values("(?1, ?2, ?3, ?4, ?5, ?6)");

                    tx.execute(
                        &query.as_string(),
                        (
                            created_at.clone(),
                            updated_at.clone(),
                            file_uuid.to_string(),
                            upload.0,
                            upload.1,
                            upload.2.to_vec(),
                        ),
                    )?;
                }

                tx.commit()?;
                Ok(())
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match e {
                Rusqlite(rusqlite::Error::SqliteFailure(err, _)) => {
                    if err.code == rusqlite::ErrorCode::ConstraintViolation {
                        Err(ServerError::Conflict)
                    } else {
                        Err(e.into())
                    }
                }
                _ => Err(e.into()),
            },
        }
    }

    pub async fn edit(
        client: &Client,
        page_uuid: Uuid,
        page_name: String,
        page_content: String,
    ) -> Result<(), ServerError> {
        let query = sql::Update::new()
            .update("pages")
            .set("updated_at = ?1, page_name = ?2, page_content = ?3")
            .where_clause("page_uuid = ?4");

        let now = UtcDateTime::now();
        let updated_at = now.format(&Rfc3339)?;
        let page_content = sanitize_html(&page_content);
        client
            .conn(move |conn| {
                let mut stmt = conn.prepare_cached(&query.as_string())?;
                stmt.execute((updated_at, page_name, page_content, page_uuid.to_string()))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    pub async fn find_by_name(client: &Client, page_name: String) -> Result<Self, ServerError> {
        let query = sql::Select::new()
            .select(
                "page_id, created_at, updated_at, page_uuid, page_name, page_content, page_text",
            )
            .from("pages")
            .where_clause("page_name = ?1");

        let content: Result<PageEntity, async_sqlite::Error> = client
            .conn(move |conn| {
                let mut stmt = conn.prepare_cached(&query.as_string())?;
                stmt.query_row([page_name], |row| {
                    let page_uuid = row.get::<_, String>("page_uuid")?;
                    let page_uuid = page_uuid.parse().unwrap();
                    Ok(PageEntity {
                        page_id: row.get("page_id")?,
                        created_at: row.get("created_at")?,
                        updated_at: row.get("updated_at")?,
                        page_uuid,
                        page_name: row.get("page_name")?,
                        page_content: row.get("page_content")?,
                        page_text: row.get("page_text")?,
                    })
                })
            })
            .await;

        match content {
            Ok(entity) => Ok(entity),
            Err(Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Err(ServerError::NotFound),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    page_uuid: Uuid,
    page_name: String,
    page_content: String,
    updated_at: String,
}

impl From<PageEntity> for PageResponse {
    fn from(value: PageEntity) -> Self {
        Self {
            page_uuid: value.page_uuid,
            page_name: value.page_name,
            page_content: value.page_content,
            updated_at: value.updated_at,
        }
    }
}
