#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use aws_sdk_dynamodb::{Client, Endpoint};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_tables])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn list_tables() -> Vec<String> {
    let mut tables = Vec::new();
    let shared_config = aws_config::from_env()
        .endpoint_resolver(Endpoint::immutable(
            "http://localhost:8000".parse().expect("valid URI"),
        ))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let resp = req.send().await.unwrap();
    if let Some(table_names) = resp.table_names {
        for table_name in table_names {
            tables.push(table_name.to_string());
        }
    }
    tables
}
