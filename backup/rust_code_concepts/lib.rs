// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod storage;
//use crate::storage::DuckDBDataManagerImpl;

//mod swiss_ephem;
//use swiss_ephem::{Date, Planet};
//use swiss_ephem::calculate_planet_position;

//use tauri::{Builder, Manager, Result};
use std::collections::HashMap;
use tauri::{generate_context, generate_handler};
//use serde::Deserialize; // using JSON / YAML for translations

// https://crates.io/crates/rust-i18n

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn translation(name: &str) -> String {
    let translations = {
        let mut translations = HashMap::new();
        translations.insert("en", "Using the English language");
        translations.insert("es", "Utilizando el idioma inglés");
        translations.insert("fr", "Utilisant la langue anglaise");
        translations.insert("de", "Verwenden Sie die englische Sprache");
        translations.insert("cz", "Používám český jazyk");
        translations
    };

    match translations.get(name) {
        Some(translation) => translation.to_string(),
        None => {
            println!("Translation not found for name: {}", name);
            String::from("Unknown language")
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        //.manage(DuckDBDataManagerImpl::new().unwrap()) // Initialize your data manager and pass it to Tauri's manager
        .invoke_handler(generate_handler![translation])
        .run(generate_context!())
        .expect("error while running tauri application");
}
