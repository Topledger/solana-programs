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
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="2")]
    pub block_slot: u64,
    #[prost(uint64, required, tag="3")]
    pub block_time: u64,
    #[prost(uint64, required, tag="4")]
    pub fees: u64,
    #[prost(string, required, tag="5")]
    pub signer: ::prost::alloc::string::String,
    #[prost(bool, required, tag="6")]
    pub success: bool,
    #[prost(uint32, required, tag="7")]
    pub required_signatures: u32,
}
// @@protoc_insertion_point(module)
