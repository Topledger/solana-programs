// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeBudgetMeta {
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
    #[prost(uint32, optional, tag="11")]
    pub bytes: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub units: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="13")]
    pub micro_lamports: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<ComputeBudgetMeta>,
}
// @@protoc_insertion_point(module)
