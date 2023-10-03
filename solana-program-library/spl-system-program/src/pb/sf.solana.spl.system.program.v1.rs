// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lamports: u64,
    #[prost(uint64, tag="3")]
    pub space: u64,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub seed: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub from_seed: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub from_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplSystemProgramMeta {
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
    pub data: ::prost::alloc::vec::Vec<SplSystemProgramMeta>,
}
// @@protoc_insertion_point(module)
