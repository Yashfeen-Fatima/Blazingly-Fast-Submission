use std::hash::{DefaultHasher, Hash, Hasher};

use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone, Serialize)]
pub struct Urls {
    pub long_url: String,
    short_code: String,
}

impl Urls {
    pub fn new<S>(long_url: S) -> Self
    where
        S: Into<String>,
    {
        let long_url = long_url.into();
        let mut s = DefaultHasher::new();
        long_url.hash(&mut s);
        let short_code = format!("{:x}", s.finish())[0..5].to_string();

        Self {
            long_url,
            short_code,
        }
    }

    pub async fn add_to_db(&mut self, pool: &Pool<Postgres>) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO urls ( long_url, short_code ) VALUES ( $1, $2 );
            "#,
            self.long_url,
            self.short_code
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_from_db(&mut self, pool: &Pool<Postgres>) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM urls WHERE short_code = $1;
            "#,
            self.short_code
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
