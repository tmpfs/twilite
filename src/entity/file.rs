use crate::error::ServerError;
use async_sqlite::{Client, Error::Rusqlite, rusqlite};
use sql_query_builder as sql;
use uuid::Uuid;

pub struct FileEntity {
    pub file_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub file_uuid: Uuid,
    pub file_name: String,
    pub file_size: usize,
    pub content_type: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse {
    file_uuid: Uuid,
    file_name: String,
    file_size: usize,
    content_type: String,
    updated_at: String,
}

impl From<FileEntity> for FileResponse {
    fn from(value: FileEntity) -> Self {
        Self {
            file_uuid: value.file_uuid,
            file_name: value.file_name,
            file_size: value.file_size,
            content_type: value.content_type,
            updated_at: value.updated_at,
        }
    }
}

impl FileEntity {
    pub async fn find_all_by_page_id(
        client: &Client,
        page_id: i32,
    ) -> Result<Vec<FileEntity>, ServerError> {
        let sql = sql::Select::new()
            .select(
                "f.file_id, f.created_at, f.updated_at, f.file_uuid, f.file_name, f.file_size, f.content_type",
            )
            .from("files f")
            .inner_join("page_files pf ON f.file_id = pf.file_id")
            .where_clause("pf.page_id = ?");
        let files = client
            .conn(move |conn| {
                let mut stmt = conn.prepare_cached(&sql.as_string())?;
                let mut rows = stmt.query([page_id])?;
                let mut files = Vec::new();
                while let Some(row) = rows.next()? {
                    let file_uuid: String = row.get("file_uuid")?;
                    let file_uuid = file_uuid.parse().unwrap();
                    let file_entity = FileEntity {
                        file_id: row.get("file_id")?,
                        created_at: row.get("created_at")?,
                        updated_at: row.get("updated_at")?,
                        file_uuid,
                        file_name: row.get("file_name")?,
                        file_size: row.get("file_size")?,
                        content_type: row.get("content_type")?,
                    };
                    files.push(file_entity);
                }
                Ok(files)
            })
            .await?;
        Ok(files)
    }

    pub async fn find_buffer_by_uuid(
        client: &Client,
        file_uuid: Uuid,
    ) -> Result<(usize, String, Vec<u8>), ServerError> {
        let query = sql::Select::new()
            .select("file_size, content_type, file_content")
            .from("files")
            .where_clause("file_uuid = ?1");

        let content: Result<(usize, String, Vec<u8>), async_sqlite::Error> = client
            .conn(move |conn| {
                let mut stmt = conn.prepare_cached(&query.as_string())?;
                stmt.query_row([file_uuid.to_string()], |row| {
                    Ok((
                        row.get("file_size")?,
                        row.get("content_type")?,
                        row.get("file_content")?,
                    ))
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
