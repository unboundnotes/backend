use std::error::Error;

use anyhow::Result as AResult;
use appconfig_derive::DataSource;
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};

pub struct PostgresqlDataSource {
    client: Client,
}

impl PostgresqlDataSource {
    pub async fn new(url: &str) -> AResult<Self> {
        let (client, conn) = tokio_postgres::connect(url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        let mut client = Self { client };
        client.ensure_table().await?;
        Ok(client)
    }

    async fn ensure_table(&mut self) -> AResult<()> {
        self.client
            .batch_execute(
                r#"
            CREATE TABLE IF NOT EXISTS data_source (
                key TEXT NOT NULL PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DataSource for PostgresqlDataSource {
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let stmt = self
            .client
            .prepare("SELECT value FROM data_source WHERE key = $1")
            .await?;
        let rows = self.client.query(&stmt, &[&key]).await?;
        Ok(rows.get(0).map(|row| row.get(0)))
    }

    async fn set(&mut self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        let stmt = self
            .client
            .prepare(
                r#"INSERT INTO data_source(key, value)
            VALUES ($1, $2)
            ON CONFLICT (key) DO UPDATE SET value = $2;"#,
            )
            .await?;
        self.client.execute(&stmt, &[&key, &value]).await?;
        Ok(())
    }
}
