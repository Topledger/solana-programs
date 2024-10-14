// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdatePriceNoFailOnErrorArgsLayout {
    #[prost(string, required, tag="1")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="2")]
    pub unused1: u32,
    #[prost(sint64, required, tag="3")]
    pub price: i64,
    #[prost(uint64, required, tag="4")]
    pub conf: u64,
    #[prost(uint64, required, tag="5")]
    pub publish_slot: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetMinPublishersArgsLayout {
    #[prost(uint32, required, tag="1")]
    pub min_publishers: u32,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub unused1: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDeletePublisherArgsLayout {
    #[prost(string, required, tag="1")]
    pub publisher: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAddPublisherArgsLayout {
    #[prost(string, required, tag="1")]
    pub publisher: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAddPriceArgsLayout {
    #[prost(int32, required, tag="1")]
    pub exponent: i32,
    #[prost(string, required, tag="2")]
    pub price_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub update_price_args: ::core::option::Option<PbUpdatePriceNoFailOnErrorArgsLayout>,
    #[prost(message, optional, tag="3")]
    pub update_price_no_fail_on_error_args: ::core::option::Option<PbUpdatePriceNoFailOnErrorArgsLayout>,
    #[prost(message, optional, tag="4")]
    pub set_min_publishers_args: ::core::option::Option<PbSetMinPublishersArgsLayout>,
    #[prost(message, optional, tag="5")]
    pub delete_publisher_args: ::core::option::Option<PbDeletePublisherArgsLayout>,
    #[prost(message, optional, tag="6")]
    pub add_publisher_args: ::core::option::Option<PbAddPublisherArgsLayout>,
    #[prost(message, optional, tag="7")]
    pub add_price_args: ::core::option::Option<PbAddPriceArgsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub publisher: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub price: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub funding: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub product: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub mapping: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub next_mapping: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythOracleMeta {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub block_time: i64,
    #[prost(string, required, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, required, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="10")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, required, tag="11")]
    pub args: Arg,
    #[prost(message, required, tag="12")]
    pub input_accounts: InputAccounts,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<PythOracleMeta>,
}
// @@protoc_insertion_point(module)
