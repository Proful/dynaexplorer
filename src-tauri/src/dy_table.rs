use aws_sdk_dynamodb::model::*;
use aws_smithy_types::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "TableDescription")]
pub struct TableDescriptionDef {
    pub attribute_definitions: std::option::Option<std::vec::Vec<AttributeDefinition>>,
    pub table_name: std::option::Option<std::string::String>,
    pub key_schema: std::option::Option<std::vec::Vec<KeySchemaElement>>,
    pub table_status: std::option::Option<TableStatus>,
    pub creation_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub provisioned_throughput: std::option::Option<ProvisionedThroughputDescription>,
    pub table_size_bytes: i64,
    pub item_count: i64,
    pub table_arn: std::option::Option<std::string::String>,
    pub table_id: std::option::Option<std::string::String>,
    pub billing_mode_summary: std::option::Option<BillingModeSummary>,
    pub local_secondary_indexes: std::option::Option<std::vec::Vec<LocalSecondaryIndexDescription>>,
    pub global_secondary_indexes:
        std::option::Option<std::vec::Vec<GlobalSecondaryIndexDescription>>,
    pub stream_specification: std::option::Option<StreamSpecification>,
    pub latest_stream_label: std::option::Option<std::string::String>,
    pub latest_stream_arn: std::option::Option<std::string::String>,
    pub global_table_version: std::option::Option<std::string::String>,
    pub replicas: std::option::Option<std::vec::Vec<ReplicaDescription>>,
    pub restore_summary: std::option::Option<RestoreSummary>,
    pub sse_description: std::option::Option<SseDescription>,
    pub archival_summary: std::option::Option<ArchivalSummary>,
    pub table_class_summary: std::option::Option<TableClassSummary>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "AttributeDefinition")]
pub struct AttributeDefinitionDef {
    pub attribute_name: std::option::Option<std::string::String>,
    pub attribute_type: std::option::Option<ScalarAttributeType>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ScalarAttributeType")]
pub enum ScalarAttributeTypeDef {
    B,
    N,
    S,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "KeySchemaElement")]
pub struct KeySchemaElementDef {
    pub attribute_name: std::option::Option<std::string::String>,
    pub key_type: std::option::Option<KeyType>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "KeyType")]
pub enum KeyTypeDef {
    Hash,
    Range,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TableStatus")]
pub enum TableStatusDef {
    Active,
    Archived,
    Archiving,
    Creating,
    Deleting,
    InaccessibleEncryptionCredentials,
    Updating,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "aws_smithy_types::DateTime")]
pub struct DateTimeDef {
    #[serde(getter = "aws_smithy_types::DateTime::secs")]
    seconds: i64,
    #[serde(getter = "aws_smithy_types::DateTime::subsec_nanos")]
    subsecond_nanos: u32,
}

impl From<DateTimeDef> for aws_smithy_types::DateTime {
    fn from(def: DateTimeDef) -> aws_smithy_types::DateTime {
        aws_smithy_types::DateTime::from_secs_and_nanos(def.seconds, def.subsecond_nanos)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ProvisionedThroughputDescription")]
pub struct ProvisionedThroughputDescriptionDef {
    pub last_increase_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub last_decrease_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub number_of_decreases_today: std::option::Option<i64>,
    pub read_capacity_units: std::option::Option<i64>,
    pub write_capacity_units: std::option::Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BillingModeSummary")]
pub struct BillingModeSummaryDef {
    pub billing_mode: std::option::Option<BillingMode>,
    pub last_update_to_pay_per_request_date_time: std::option::Option<aws_smithy_types::DateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BillingMode")]
pub enum BillingModeDef {
    PayPerRequest,
    Provisioned,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "LocalSecondaryIndexDescription")]
pub struct LocalSecondaryIndexDescriptionDef {
    pub index_name: std::option::Option<std::string::String>,
    pub key_schema: std::option::Option<std::vec::Vec<KeySchemaElement>>,
    pub projection: std::option::Option<Projection>,
    pub index_size_bytes: i64,
    pub item_count: i64,
    pub index_arn: std::option::Option<std::string::String>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Projection")]
pub struct ProjectionDef {
    pub projection_type: std::option::Option<ProjectionType>,
    pub non_key_attributes: std::option::Option<std::vec::Vec<std::string::String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ProjectionType")]
pub enum ProjectionTypeDef {
    All,
    Include,
    KeysOnly,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GlobalSecondaryIndexDescription")]
pub struct GlobalSecondaryIndexDescriptionDef {
    pub index_name: std::option::Option<std::string::String>,
    pub key_schema: std::option::Option<std::vec::Vec<KeySchemaElement>>,
    pub projection: std::option::Option<Projection>,
    pub index_status: std::option::Option<IndexStatus>,
    pub backfilling: std::option::Option<bool>,
    pub provisioned_throughput: std::option::Option<ProvisionedThroughputDescription>,
    pub index_size_bytes: i64,
    pub item_count: i64,
    pub index_arn: std::option::Option<std::string::String>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "IndexStatus")]
pub enum IndexStatusDef {
    Active,
    Creating,
    Deleting,
    Updating,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "StreamSpecification")]
pub struct StreamSpecificationDef {
    pub stream_enabled: std::option::Option<bool>,
    pub stream_view_type: std::option::Option<StreamViewType>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "StreamViewType")]
pub enum StreamViewTypeDef {
    KeysOnly,
    NewAndOldImages,
    NewImage,
    OldImage,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ReplicaDescription")]
pub struct ReplicaDescriptionDef {
    pub region_name: std::option::Option<std::string::String>,
    pub replica_status: std::option::Option<ReplicaStatus>,
    pub replica_status_description: std::option::Option<std::string::String>,
    pub replica_status_percent_progress: std::option::Option<std::string::String>,
    pub kms_master_key_id: std::option::Option<std::string::String>,
    pub provisioned_throughput_override: std::option::Option<ProvisionedThroughputOverride>,
    pub global_secondary_indexes:
        std::option::Option<std::vec::Vec<ReplicaGlobalSecondaryIndexDescription>>,
    pub replica_inaccessible_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub replica_table_class_summary: std::option::Option<TableClassSummary>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ReplicaStatus")]
pub enum ReplicaStatusDef {
    Active,
    Creating,
    CreationFailed,
    Deleting,
    InaccessibleEncryptionCredentials,
    RegionDisabled,
    Updating,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ProvisionedThroughputOverride")]
pub struct ProvisionedThroughputOverrideDef {
    pub read_capacity_units: std::option::Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ReplicaGlobalSecondaryIndexDescription")]
pub struct ReplicaGlobalSecondaryIndexDescriptionDef {
    pub index_name: std::option::Option<std::string::String>,
    pub provisioned_throughput_override: std::option::Option<ProvisionedThroughputOverride>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TableClassSummary")]
pub struct TableClassSummaryDef {
    pub table_class: std::option::Option<TableClass>,
    pub last_update_date_time: std::option::Option<aws_smithy_types::DateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TableClass")]
pub enum TableClassDef {
    Standard,
    StandardInfrequentAccess,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "RestoreSummary")]
pub struct RestoreSummaryDef {
    pub source_backup_arn: std::option::Option<std::string::String>,
    pub source_table_arn: std::option::Option<std::string::String>,
    pub restore_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub restore_in_progress: std::option::Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SseDescription")]
pub struct SseDescriptionDef {
    pub status: std::option::Option<SseStatus>,
    pub sse_type: std::option::Option<SseType>,
    pub kms_master_key_arn: std::option::Option<std::string::String>,
    pub inaccessible_encryption_date_time: std::option::Option<aws_smithy_types::DateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SseStatus")]
pub enum SseStatusDef {
    Disabled,
    Disabling,
    Enabled,
    Enabling,
    Updating,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SseType")]
pub enum SseTypeDef {
    Aes256,
    Kms,
    Unknown(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ArchivalSummary")]
pub struct ArchivalSummaryDef {
    // #[serde(default, with = "date_time_def")]
    #[serde(with = "DateTimeDef")]
    pub archival_date_time: std::option::Option<DateTime>,
    pub archival_reason: std::option::Option<std::string::String>,
    pub archival_backup_arn: std::option::Option<std::string::String>,
}

mod date_time_def {
    use super::{DateTime, DateTimeDef};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "DateTimeDef")] &'a DateTime);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "DateTimeDef")] DateTime);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}
