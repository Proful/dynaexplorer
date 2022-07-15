#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use aws_sdk_dynamodb::{model::KeyType, Client, Endpoint};
use serde::Serialize;
use serde_json::Value;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_tables,
            list_items,
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

#[derive(Debug, Serialize)]
struct Table {
    name: String,
    primary_key: String,
    sort_key: String,
    item_count: i64,
}

#[tauri::command]
async fn describe_table(table_name: String) -> Table {
    let resp = get_client()
        .await
        .describe_table()
        .table_name(table_name.clone())
        .send()
        .await
        .unwrap();

    // dbg!(&resp);

    let table_desc = resp.table.unwrap();
    let key_schema = table_desc.key_schema.unwrap();

    let mut table = Table {
        name: table_name.clone(),
        primary_key: String::new(),
        sort_key: String::new(),
        item_count: table_desc.item_count,
    };
    for el in key_schema {
        let key_type = el.key_type.unwrap();
        match key_type {
            KeyType::Hash => table.primary_key = el.attribute_name.unwrap(),
            KeyType::Range => table.sort_key = el.attribute_name.unwrap(),
            _ => todo!("unknown key type"),
        }
    }

    dbg!(&table);
    table
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Attribute {
    name: String,
    value: String,
    kind: String,
}

impl Attribute {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            value: "".to_string(),
            kind: "string".to_string(),
        }
    }
    fn from(name: String, value: String, kind: String) -> Self {
        Self { name, value, kind }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Item {
    primary_key: Attribute,
    sort_key: Attribute,
    attributes: Vec<Attribute>,
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
        // let mut records = Vec::new();
        let mut row = Item {
            primary_key: Attribute::new(),
            sort_key: Attribute::new(),
            attributes: Vec::new(),
        };
        for (key, value) in item {
            let v = if value.is_s() {
                let data = value.as_s().unwrap().to_string();
                let x: Result<Value, _> = serde_json::from_str(&data);

                if let Ok(_) = x {
                    [data, "json".to_string()]
                } else {
                    [data, "string".to_string()]
                }
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
                [s, "json".to_string()]
            } else if value.is_m() {
                let mut m: String = String::new();
                m.push_str("{");
                for (k, v) in value.as_m().unwrap() {
                    m.push_str(&format!("\"{}\":{}", k, v.as_s().unwrap()));
                }
                m.push_str("}");
                [m, "json".to_string()]
            } else {
                ["".to_string(), "string".to_string()]
            };

            match key.as_str() {
                "PK" => row.primary_key = Attribute::from(key, v[0].clone(), v[1].clone()),
                "SK" => row.sort_key = Attribute::from(key, v[0].clone(), v[1].clone()),
                _ => row
                    .attributes
                    .push(Attribute::from(key, v[0].clone(), v[1].clone())),
            }
        }
        rows.push(row);
    }
    rows
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
    //#[tokio::test]
    async fn _test_describe_table() {
        let table_name = "sketchnotes_dev_v1".to_string();
        let resp = super::describe_table(table_name.clone()).await;
        assert_eq!(resp.name, table_name);
        assert_eq!(resp.primary_key, "PK".to_string());
        assert_eq!(resp.sort_key, "SK".to_string());
    }

    #[tokio::test]
    async fn test_list_items() {
        let table_name = "sketchnotes_dev_v1".to_string();
        let resp = super::list_items(table_name.clone()).await;
        dbg!(&resp);
    }
}
