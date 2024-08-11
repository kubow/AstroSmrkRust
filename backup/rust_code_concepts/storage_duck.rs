//extern crate duckdb;
//currently uninstalled
use duckdb::{Connection, Result};

pub fn load_translation(key: &str) -> Result<()> {
    // Load translations from DuckDB database
    let connection_string = String::from("settings.db");
    let conn = Connection::open(connection_string)?;
    match conn.execute_batch(&format!("SELECT * FROM {}_translation;", key)) {
        Ok(translation) => Ok(translation),
        Err(err) => Err(err.into()),
    }
}