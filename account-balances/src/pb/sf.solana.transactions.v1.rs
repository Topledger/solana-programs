// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TransactionStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="3")]
    pub fee: u64,
    #[prost(string, required, tag="4")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, required, tag="5")]
    pub success: bool,
    #[prost(message, repeated, tag="6")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(string, required, tag="7")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="9")]
    pub base_fee: u32,
    #[prost(uint32, required, tag="10")]
    pub priority_fee: u32,
    #[prost(uint64, required, tag="11")]
    pub compute_units_consumed: u64,
    #[prost(uint64, required, tag="12")]
    pub compute_units_allocated: u64,
    #[prost(uint32, required, tag="13")]
    pub byte_size: u32,
    #[prost(uint32, required, tag="14")]
    pub trx_accounts_size: u32,
    #[prost(uint32, required, tag="15")]
    pub writable_alt_accounts_size: u32,
    #[prost(uint32, required, tag="16")]
    pub readable_alt_accounts_size: u32,
    #[prost(bool, required, tag="17")]
    pub logs_truncated: bool,
    #[prost(uint32, required, tag="18")]
    pub signatures_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, required, tag="1")]
    pub executing_account: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="5")]
    pub bytes: u32,
    #[prost(uint32, required, tag="6")]
    pub account_bytes: u32,
    #[prost(uint32, required, tag="7")]
    pub data_bytes: u32,
}
// @@protoc_insertion_point(module)
