use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
    pub validity: i64,
    pub expiresAt: Option<String>,
    pub roles: Option<serde_json::Value>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct VsphereCredentials {
    pub username: String,
    pub password: String,
    pub authSource: String,
}

impl VsphereCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            authSource: "nb.com".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct VsphereResourceId {
    pub resource_list: Vec<Identifier>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
pub struct Identifier {
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
pub struct VsphereNameField {
    pub name: Vec<String>,
}

impl VsphereNameField {
    pub fn new(name: &str) -> Self {
        Self {
            name: vec![name.to_owned()],
        }
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct VsphereMetrics {
    pub values: Vec<VsphereMetricsValue>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct VsphereMetricsValue {
    pub resource_id: String,
    #[serde(rename = "stat-list")]
    pub stat_list: StatList,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct StatList {
    pub stat: Vec<Stat>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub timestamps: Vec<i64>,
    pub stat_key: StatKey,
    pub roll_up_type: String,
    pub interval_unit: IntervalUnit,
    pub data: Vec<f64>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct StatKey {
    pub key: String,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct IntervalUnit {
    pub quantifier: i64,
    pub interval_type: String,
}

#[derive(Serialize, Deserialize, Debug, InputObject, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VsphereMetricsInput {
    pub interval_type: Option<IntervalType>,
    pub roll_up_type: Option<RollupType>,
    pub resource_id: String,
    pub begin: Option<i64>,
    pub end: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VsphereMetricsQueryBody {
    pub interval_type: String,
    pub interval_quantifier: i32,
    pub roll_up_type: String,
    pub resource_id: Vec<String>,
    pub stat_key: Vec<String>,
    pub begin: Option<i64>,
    pub end: Option<i64>,
}

impl VsphereMetricsQueryBody {
    pub fn new(input: VsphereMetricsInput, identifier: String) -> Self {
        Self {
            begin: input.begin,
            end: input.end,
            interval_quantifier: 1,
            resource_id: vec![identifier],
            stat_key: vec![
                "diskspace|workload",
                "mem|workload",
                "cpu|usage_average",
                "disk|read_average",
                "disk|write_average",
                "net|usage_average",
                "net|received_average",
                "virtualDisk|peak_vDisk_iops",
                "virtualDisk|peak_vDisk_readLatency",
                "virtualDisk|peak_vDisk_writeLatency",
                "net|transmitted_average",
                "OnlineCapacityAnalytics|diskspace|capacityRemaining",
            ]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
            interval_type: {
                if let Some(interval) = input.interval_type {
                    interval.to_string()
                } else {
                    "HOURS".to_string()
                }
            },
            roll_up_type: {
                if let Some(roll_up_type) = input.roll_up_type {
                    roll_up_type.to_string()
                } else {
                    "AVG".to_string()
                }
            },
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum RollupType {
    Sum,
    Avg,
    Min,
    Max,
    None,
    Latest,
    Count,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum IntervalType {
    Hours,
    Minutes,
    Days,
    Weeks,
    Months,
    Years,
}

impl RollupType {
    pub fn to_string(&self) -> String {
        match *self {
            RollupType::Sum => "SUM".to_string(),
            RollupType::Avg => "AVG".to_string(),
            RollupType::Min => "MIN".to_string(),
            RollupType::Max => "MAX".to_string(),
            RollupType::None => "NONE".to_string(),
            RollupType::Latest => "LATEST".to_string(),
            RollupType::Count => "COUNT".to_string(),
        }
    }
}

impl IntervalType {
    pub fn to_string(&self) -> String {
        match *self {
            IntervalType::Hours => "HOURS".to_string(),
            IntervalType::Minutes => "MINUTES".to_string(),
            IntervalType::Days => "DAYS".to_string(),
            IntervalType::Weeks => "WEEKS".to_string(),
            IntervalType::Months => "MONTHS".to_string(),
            IntervalType::Years => "YEARS".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VsphereSeverPropertiesInput {
    pub property_keys: Vec<String>,
    pub resource_ids: Vec<String>,
}

impl VsphereSeverPropertiesInput {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_ids: vec![resource_id],
            property_keys: vec![
                "config|hardware|diskSpace".to_string(),
                "config|hardware|numCpu".to_string(),
                "config|memoryAllocation|shares|shares".to_string(),
                "cpu|speed".to_string(),
                "net:4000|ip_address".to_string(),
                "net:4000|mac_address".to_string(),
                "summary|parentDatacenter".to_string(),
                "config|guestFullName".to_string(),
                "config|name".to_string(),
                "summary|datastore".to_string(),
                "summary|UUID".to_string(),
            ],
        }
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct VsphereSeverProperties {
    pub values: Vec<VsphereSeverPropertiesValue>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct VsphereSeverPropertiesValue {
    pub resource_id: String,
    #[serde(rename = "property-contents")]
    pub property_contents: PropertyContents,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct PropertyContents {
    #[serde(rename = "property-content")]
    pub property_content: Vec<PropertyContent>,
}

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct PropertyContent {
    pub stat_key: String,
    pub timestamps: Vec<i64>,
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    pub data: Vec<f64>,
}
