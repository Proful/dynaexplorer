use aws_sdk_dynamodb::model::TableDescription;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDefinition {
    pub attribute_name: String,
    pub attribute_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeySchema {
    pub attribute_name: String,
    pub key_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionedThroughput {
    pub last_increase_date_time: String,
    pub last_decrease_date_time: String,
    pub number_of_decreases_today: i64,
    pub read_capacity_units: i64,
    pub write_capacity_units: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub attribute_definitions: Vec<AttributeDefinition>,
    pub table_name: String,
    pub key_schema: Vec<KeySchema>,
    pub table_status: String,
    pub creation_date_time: String,
    pub provisioned_throughput: ProvisionedThroughput,
    pub table_size_bytes: i64,
    pub item_count: i64,
    pub table_arn: String,
}

impl From<TableDescription> for Table {
    fn from(table_desc: TableDescription) -> Self {
        let attribute_definitions: Vec<AttributeDefinition> = table_desc
            .attribute_definitions
            .unwrap()
            .into_iter()
            .map(|attr_def| AttributeDefinition {
                attribute_name: attr_def.attribute_name.unwrap_or_default(),
                attribute_type: attr_def.attribute_type.unwrap().as_str().to_string(),
            })
            .collect();

        let table_name = table_desc.table_name.unwrap_or_default();

        let key_schema: Vec<KeySchema> = table_desc
            .key_schema
            .unwrap()
            .into_iter()
            .map(|key_schema| KeySchema {
                attribute_name: key_schema.attribute_name.unwrap_or_default(),
                key_type: key_schema.key_type.unwrap().as_str().to_string(),
            })
            .collect();

        let table_status = table_desc.table_status.unwrap().as_str().to_string();

        let creation_date_time = table_desc
            .creation_date_time
            .unwrap()
            .fmt(aws_smithy_types::date_time::Format::DateTime)
            .unwrap();

        let pr_th = table_desc.provisioned_throughput.unwrap();
        let provisioned_throughput = ProvisionedThroughput {
            last_increase_date_time: pr_th
                .last_increase_date_time
                .unwrap()
                .fmt(aws_smithy_types::date_time::Format::DateTime)
                .unwrap(),
            last_decrease_date_time: pr_th
                .last_decrease_date_time
                .unwrap()
                .fmt(aws_smithy_types::date_time::Format::DateTime)
                .unwrap(),
            number_of_decreases_today: pr_th.number_of_decreases_today.unwrap(),
            read_capacity_units: pr_th.read_capacity_units.unwrap(),
            write_capacity_units: pr_th.write_capacity_units.unwrap(),
        };
        let table_size_bytes = table_desc.table_size_bytes;
        let item_count = table_desc.item_count;
        let table_arn = table_desc.table_arn.unwrap_or_default();

        Table {
            attribute_definitions,
            table_name,
            key_schema,
            table_status,
            creation_date_time,
            provisioned_throughput,
            table_size_bytes,
            item_count,
            table_arn,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            value: "".to_string(),
        }
    }
    pub fn from(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub partion_key: Attribute,
    pub sort_key: Attribute,
    pub attributes: Vec<Attribute>,
}
