extern crate duckdb;
use duckdb::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct DuckDBDataManagerImpl {
    conn: Arc<Mutex<Connection>>,
}

impl DuckDBDataManagerImpl {
    pub fn new() -> Result<Self> {
        // Connect to DuckDB database
        let connection_string = String::from("settings.db");
        let conn= Connection::open(connection_string)?;
        let conn = Arc::new(Mutex::new(conn));

        Ok(Self { conn })
    }

    pub fn load_translation(&self, key: &str) -> Result<Option<String>> {
        // Load translations from DuckDB database
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            &format!("SELECT * FROM {}_translation;", key),
            [],
            |row| row.get(0),
        );

        match result {
            Ok(translation) => Ok(Some(translation)),
            Err(_) => Ok(None),
        }
    }

    pub fn load_config(&self, key: &str) -> Result<Option<String>> {
        // Load config from DuckDB database
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            &format!("SELECT value FROM config WHERE key = '{}'", key),
            [],
            |row| row.get(0),
        );

        match result {
            Ok(translation) => Ok(Some(translation)),
            Err(_) => Ok(None),
        }
    }
}