use std::{cell::RefCell, error::Error};

use anyhow::Result as AResult;
use appconfig_derive::DataSource;
use postgres::{Client, NoTls};

pub struct PostgresqlDataSource {
    client: RefCell<Client>,
}

impl PostgresqlDataSource {
    pub fn new(host: &str, user: &str, password: &str, db: &str) -> AResult<Self> {
        let client = Client::configure()
            .host(host)
            .user(user)
            .password(password)
            .dbname(db)
            .connect(NoTls)?;
        let mut client = Self {
            client: RefCell::new(client),
        };
        client.ensure_table()?;
        Ok(client)
    }

    fn ensure_table(&mut self) -> AResult<()> {
        self.client.get_mut().batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS data_source (
                key TEXT NOT NULL PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }
}

impl DataSource for PostgresqlDataSource {
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let stmt = self
            .client
            .borrow_mut()
            .prepare("SELECT value FROM data_source WHERE key = $1")?;
        let rows = self.client.borrow_mut().query(&stmt, &[&key])?;
        Ok(rows.get(0).map(|row| row.get(0)))
    }

    fn set(&mut self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        let stmt = self
            .client
            .get_mut()
            .prepare("INSERT INTO data_source (key, value) VALUES ($1, $2)")?;
        self.client.borrow_mut().execute(&stmt, &[&key, &value])?;
        Ok(())
    }
}
