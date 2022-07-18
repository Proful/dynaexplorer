use aws_sdk_dynamodb::model::*;
use aws_smithy_types::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "ArchivalSummary")]
pub struct ArchivalSummaryDef {
    #[serde(default, with = "date_time_def")]
    // #[serde(with = "DateTimeDef")]
    pub archival_date_time: std::option::Option<DateTime>,
    pub archival_reason: std::option::Option<std::string::String>,
    pub archival_backup_arn: std::option::Option<std::string::String>,
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
