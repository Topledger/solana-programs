// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(uint32, tag="1")]
    pub max_depth: u32,
    #[prost(uint32, tag="2")]
    pub max_buffer_size: u32,
    #[prost(bool, optional, tag="3")]
    pub public: ::core::option::Option<bool>,
    #[prost(message, optional, tag="4")]
    pub message: ::core::option::Option<Message>,
    #[prost(message, optional, tag="5")]
    pub metadata_args: ::core::option::Option<MetadataArgs>,
    #[prost(int32, repeated, tag="6")]
    pub root: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="7")]
    pub data_hash: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="8")]
    pub creator_hash: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint64, tag="9")]
    pub nonce: u64,
    #[prost(uint32, tag="10")]
    pub index: u32,
    #[prost(string, tag="11")]
    pub collection: ::prost::alloc::string::String,
    #[prost(message, optional, tag="12")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(string, tag="13")]
    pub instruction_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub seller_fee_basis_points: i32,
    #[prost(bool, tag="5")]
    pub primary_sale_happened: bool,
    #[prost(bool, tag="6")]
    pub is_mutable: bool,
    #[prost(int32, optional, tag="7")]
    pub edition_nonce: ::core::option::Option<i32>,
    #[prost(string, tag="8")]
    pub token_standard: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub collection: ::core::option::Option<Collection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uses {
    #[prost(string, tag="1")]
    pub use_method: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub remaining: u64,
    #[prost(uint64, tag="3")]
    pub total: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Creator {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub verified: bool,
    #[prost(int32, tag="3")]
    pub share: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(bool, tag="1")]
    pub verified: bool,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataArgs {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub seller_fee_basis_points: i32,
    #[prost(bool, tag="5")]
    pub primary_sale_happened: bool,
    #[prost(bool, tag="6")]
    pub is_mutable: bool,
    #[prost(int32, optional, tag="7")]
    pub edition_nonce: ::core::option::Option<i32>,
    #[prost(string, tag="8")]
    pub token_standard: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub collection: ::core::option::Option<Collection>,
    #[prost(message, optional, tag="10")]
    pub uses: ::core::option::Option<Uses>,
    #[prost(string, tag="11")]
    pub token_program_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="12")]
    pub creators: ::prost::alloc::vec::Vec<Creator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub seller_fee_basis_points: i32,
    #[prost(bool, tag="5")]
    pub primary_sale_happened: bool,
    #[prost(bool, tag="6")]
    pub is_mutable: bool,
    #[prost(int32, optional, tag="7")]
    pub edition_nonce: ::core::option::Option<i32>,
    #[prost(string, tag="8")]
    pub token_standard: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub collection: ::core::option::Option<Collection>,
    #[prost(message, optional, tag="10")]
    pub uses: ::core::option::Option<Uses>,
    #[prost(string, tag="11")]
    pub token_program_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="12")]
    pub creators: ::prost::alloc::vec::Vec<Creator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BubblegumMeta {
    #[prost(string, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub block_time: i64,
    #[prost(string, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(message, optional, tag="10")]
    pub arg: ::core::option::Option<Arg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<BubblegumMeta>,
}
// @@protoc_insertion_point(module)
