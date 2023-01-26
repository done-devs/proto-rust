#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub successful: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub count: i64,
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<::prost_wkt_types::Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub favorite: bool,
    #[prost(bool, tag = "5")]
    pub today: bool,
    #[prost(bool, tag = "6")]
    pub is_reminder_on: bool,
    #[prost(enumeration = "Status", tag = "7")]
    pub status: i32,
    #[prost(enumeration = "Priority", tag = "8")]
    pub priority: i32,
    #[prost(message, repeated, tag = "9")]
    pub sub_tasks: ::prost::alloc::vec::Vec<SubTask>,
    #[prost(string, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub notes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub completion_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "13")]
    pub deletion_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub due_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub reminder_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "16")]
    pub reminder_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, optional, tag = "17")]
    pub recurrence: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "18")]
    pub created_date_time: i64,
    #[prost(int64, tag = "19")]
    pub last_modified_date_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubTask {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub completed: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub icon: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}
impl Priority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Normal => "NORMAL",
            Priority::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "NORMAL" => Some(Self::Normal),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    NotStarted = 0,
    Completed = 1,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::NotStarted => "NOT_STARTED",
            Status::Completed => "COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOT_STARTED" => Some(Self::NotStarted),
            "COMPLETED" => Some(Self::Completed),
            _ => None,
        }
    }
}
