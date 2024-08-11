use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// Define your table structure (replace with your actual schema)
#[derive(Queryable)]
struct Language {
    id: i32,
    col: String,
    cz: String,
    en: String,
    fr: String,
    de: String,
    es: String,
}

fn main() {
    // Establish a connection to the SQLite database file
    let connection = SqliteConnection::establish("settings.db").expect("Couldn't connect to database");

    // Query for existing data
    let results = language::table().load::<Language>(&connection).expect("Failed to query translation table");

    // Print the results
    for item in results {
        println!("ID: {}, Name: {}", item.id, item.cz);
    }
}
