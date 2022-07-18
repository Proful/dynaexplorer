#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client, Endpoint};
use model::*;
use tokio_stream::StreamExt;

mod model;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_tables,
            list_items,
            get_item,
            describe_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn list_tables() -> Vec<String> {
    let mut tables = Vec::new();

    let resp = get_client()
        .await
        .list_tables()
        .limit(10)
        .send()
        .await
        .unwrap();

    if let Some(table_names) = resp.table_names {
        for table_name in table_names {
            tables.push(table_name.to_string());
        }
    }
    tables
}

#[tauri::command]
async fn describe_table(table_name: String) -> Table {
    println!("describe_table: {}", table_name);

    let resp = get_client()
        .await
        .describe_table()
        .table_name(table_name.clone())
        .send()
        .await
        .unwrap();

    let table: Table = resp.table.unwrap().into();

    table
}

#[tauri::command]
async fn list_items(table_name: String) -> Vec<Item> {
    let mut rows = Vec::new();

    let items: Result<Vec<_>, _> = get_client()
        .await
        .scan()
        .table_name(table_name)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    for item in items.unwrap() {
        rows.push(transform_item(item));
    }
    rows
}

fn transform_item(item: HashMap<String, AttributeValue>) -> Item {
    let mut row = Item {
        partion_key: Attribute::new(),
        sort_key: Attribute::new(),
        attributes: Vec::new(),
    };
    for (key, value) in item {
        let v = if value.is_s() {
            value.as_s().unwrap().to_string()
        } else if value.is_ss() {
            let x = value.as_ss().unwrap();
            let mut s = String::new();
            s.push_str("[");
            for v in x {
                s.push_str(&format!("\"{}\",", v));
            }
            if s.ends_with(",") {
                s.pop();
            }
            s.push_str("]");
            s
        } else if value.is_m() {
            let mut m: String = String::new();
            m.push_str("{");
            for (k, v) in value.as_m().unwrap() {
                m.push_str(&format!("\"{}\":{}", k, v.as_s().unwrap()));
            }
            m.push_str("}");
            m
        } else {
            "".to_string()
        };

        match key.as_str() {
            "PK" => row.partion_key = Attribute::from(&key, &v),
            "SK" => row.sort_key = Attribute::from(&key, &v),
            _ => row.attributes.push(Attribute::from(&key, &v)),
        }
    }
    row
}

#[tauri::command]
async fn get_item(table_name: String, partion_key: Attribute, sort_key: Attribute) -> Option<Item> {
    println!("get_item: {}", table_name);
    dbg!(&partion_key, &sort_key);

    let resp = get_client()
        .await
        .get_item()
        .table_name(table_name)
        .key(partion_key.name, AttributeValue::S(partion_key.value))
        .key(sort_key.name, AttributeValue::S(sort_key.value))
        .send()
        .await
        .unwrap();

    if let Some(item) = resp.item {
        Some(transform_item(item))
    } else {
        None
    }
}

async fn get_client() -> Client {
    let shared_config = aws_config::from_env()
        .endpoint_resolver(Endpoint::immutable(
            "http://localhost:8000".parse().expect("valid URI"),
        ))
        .load()
        .await;

    Client::new(&shared_config)
}

#[cfg(test)]
mod tests_main {
    use crate::model::*;

    #[tokio::test]
    async fn test_describe_table() {
        let table_name = "sketchnotes_dev_v1".to_string();
        let _resp = crate::describe_table(table_name.clone()).await;
        // assert_eq!(resp.name, table_name);
        // assert_eq!(resp.partion_key_name, "PK".to_string());
        // assert_eq!(resp.sort_key_name, "SK".to_string());
    }

    #[tokio::test]
    async fn test_list_items() {
        let table_name = "sketchnotes_dev_v1".to_string();
        let resp = crate::list_items(table_name.clone()).await;
        dbg!(&resp);
    }

    #[tokio::test]
    async fn test_get_item() {
        let table_name = "sketchnotes_dev_v1".to_string();
        let partion_key = Attribute::from("PK", "EMAIL#butu25@gmail.com");
        let sort_key = Attribute::from("SK", "SHAPES#ALL");
        let resp = crate::get_item(table_name.clone(), partion_key, sort_key).await;
        dbg!(&resp);
    }
}
