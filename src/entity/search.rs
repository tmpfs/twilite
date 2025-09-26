use crate::error::ServerError;
use async_sqlite::Client;
use sql_query_builder as sql;

#[derive(Debug, serde::Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    pub keywords: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecord {
    pub row_id: i32,
    pub title: String,
    pub body: String,
}

impl From<SearchEntity> for SearchRecord {
    fn from(value: SearchEntity) -> Self {
        Self {
            row_id: value.row_id,
            title: value.title,
            body: value.body,
        }
    }
}

fn sanitize_token(t: &str) -> String {
    t.replace('\'', "''")
}

pub struct SearchEntity {
    pub row_id: i32,
    pub title: String,
    pub body: String,
}

impl SearchEntity {
    pub async fn fts_search(
        client: &Client,
        query: SearchQuery,
    ) -> Result<Vec<SearchEntity>, ServerError> {
        let match_expr = Self::build_fts_match(&query.keywords, 20).unwrap_or_default();
        if match_expr.is_empty() {
            return Ok(vec![]);
        }

        let search_query = query
            .keywords
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = sql::Select::new()
            .select("rowid as row_id, page_name as title, page_text as body")
            .from("pages_fts")
            .where_clause("pages_fts MATCH ?1 LIMIT 50");

        let results = client
            .conn(move |conn| {
                let mut stmt = conn.prepare_cached(&query.as_string())?;
                let mut rows = stmt.query([search_query])?;
                let mut results = Vec::new();
                while let Some(row) = rows.next()? {
                    let search_entity = SearchEntity {
                        row_id: row.get("row_id")?,
                        title: row.get("title")?,
                        body: row.get("body")?,
                    };
                    results.push(search_entity);
                }
                Ok(results)
            })
            .await?;
        Ok(results)
    }

    fn build_fts_match(input: &str, max_tokens: usize) -> Option<String> {
        let tokens: Vec<_> = input
            .split_whitespace()
            .map(sanitize_token)
            .filter(|s| !s.is_empty())
            .take(max_tokens)
            .collect();

        if tokens.is_empty() {
            None
        } else {
            // wrap each token in single quotes to force term match and join with OR
            Some(
                tokens
                    .into_iter()
                    // .map(|t| format!("'{}'", t))
                    .collect::<Vec<_>>()
                    .join(" OR "),
            )
        }
    }
}
