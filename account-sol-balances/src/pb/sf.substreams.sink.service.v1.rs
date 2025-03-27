// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployRequest {
    #[prost(message, optional, tag="1")]
    pub substreams_package: ::core::option::Option<super::super::super::v1::Package>,
    #[prost(bool, tag="2")]
    pub development_mode: bool,
    #[prost(message, repeated, tag="3")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub status: i32,
    /// deployment_id is a short name (max 8 characters) that uniquely identifies your deployment
    #[prost(string, tag="2")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub services: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub motd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(message, optional, tag="1")]
    pub substreams_package: ::core::option::Option<super::super::super::v1::Package>,
    #[prost(string, tag="2")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub reset: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub status: i32,
    #[prost(map="string, string", tag="2")]
    pub services: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub motd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    #[prost(string, tag="1")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub status: i32,
    #[prost(map="string, string", tag="2")]
    pub services: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub package_info: ::core::option::Option<PackageInfo>,
    #[prost(message, optional, tag="5")]
    pub progress: ::core::option::Option<SinkProgress>,
    #[prost(string, tag="6")]
    pub motd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkProgress {
    #[prost(uint64, tag="1")]
    pub last_processed_block: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub output_module_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub output_module_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag="1")]
    pub deployments: ::prost::alloc::vec::Vec<DeploymentWithStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentWithStatus {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="DeploymentStatus", tag="2")]
    pub status: i32,
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub package_info: ::core::option::Option<PackageInfo>,
    #[prost(message, optional, tag="5")]
    pub progress: ::core::option::Option<SinkProgress>,
    #[prost(string, tag="6")]
    pub motd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRequest {
    #[prost(string, tag="1")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub previous_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseRequest {
    #[prost(string, tag="1")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub previous_status: i32,
    #[prost(enumeration="DeploymentStatus", tag="2")]
    pub new_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    #[prost(string, tag="1")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub previous_status: i32,
    #[prost(enumeration="DeploymentStatus", tag="2")]
    pub new_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeRequest {
    #[prost(string, tag="1")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeResponse {
    #[prost(enumeration="DeploymentStatus", tag="1")]
    pub previous_status: i32,
    #[prost(enumeration="DeploymentStatus", tag="2")]
    pub new_status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentStatus {
    Unknown = 0,
    Running = 1,
    Failing = 2,
    Paused = 3,
    Stopped = 4,
    Starting = 5,
    Pausing = 6,
    Stopping = 7,
    Removing = 8,
    Resuming = 9,
}
impl DeploymentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeploymentStatus::Unknown => "UNKNOWN",
            DeploymentStatus::Running => "RUNNING",
            DeploymentStatus::Failing => "FAILING",
            DeploymentStatus::Paused => "PAUSED",
            DeploymentStatus::Stopped => "STOPPED",
            DeploymentStatus::Starting => "STARTING",
            DeploymentStatus::Pausing => "PAUSING",
            DeploymentStatus::Stopping => "STOPPING",
            DeploymentStatus::Removing => "REMOVING",
            DeploymentStatus::Resuming => "RESUMING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "RUNNING" => Some(Self::Running),
            "FAILING" => Some(Self::Failing),
            "PAUSED" => Some(Self::Paused),
            "STOPPED" => Some(Self::Stopped),
            "STARTING" => Some(Self::Starting),
            "PAUSING" => Some(Self::Pausing),
            "STOPPING" => Some(Self::Stopping),
            "REMOVING" => Some(Self::Removing),
            "RESUMING" => Some(Self::Resuming),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
