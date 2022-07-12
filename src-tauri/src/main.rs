#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use tokio_stream::stream_ext::StreamExt;

use aws_sdk_dynamodb::{Client, Endpoint};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_tables, list_items])
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

#[tauri::command]
async fn list_items(table_name: String) -> Vec<String> {
    let mut rows = Vec::new();
    let shared_config = aws_config::from_env()
        .endpoint_resolver(Endpoint::immutable(
            "http://localhost:8000".parse().expect("valid URI"),
        ))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(table_name)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;
    for item in items.unwrap() {
        // println!("   {:?}", item);
        // rows.push(item);
        let mut records = Vec::new();
        for (key, value) in item {
            let v = if value.is_s() {
                value.as_s().unwrap().to_string()
            } else if value.is_ss() {
                value.as_ss().unwrap().join(",").to_string()
            } else if value.is_m() {
                let mut m: String = "".to_string();
                for (k, v) in value.as_m().unwrap() {
                    m.push_str(&format!("{}:{}\n", k, v.as_s().unwrap()));
                }
                m
            } else {
                "".to_string()
            };

            // println!("{}:{}", key, value);
            records.push(format!("{}::{}", key, v));
        }
        rows.push(records.join("~~"));
    }
    rows
}
