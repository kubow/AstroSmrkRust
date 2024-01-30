// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod storage;
//use crate::storage::DuckDBDataManagerImpl;

//mod swiss_ephem;
//use swiss_ephem::{Date, Planet};
//use swiss_ephem::calculate_planet_position;

//use tauri::{Builder, Manager, Result};
use tauri::{generate_context, generate_handler};
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn translation(name: &str) -> String {
    let translations = {
        let mut translations = HashMap::new();
        translations.insert("en", "Using the English language");
        translations.insert("es", "Utilizando el idioma inglés");
        translations.insert("fr", "Utilisant la langue anglaise");
        translations.insert("de", "Verwenden Sie die englische Sprache");
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


fn main() {
    tauri::Builder::default()
        //.manage(DuckDBDataManagerImpl::new().unwrap()) // Initialize your data manager and pass it to Tauri's manager
        .invoke_handler(generate_handler![translation])
        .run(generate_context!())
        .expect("error while running tauri application");
}