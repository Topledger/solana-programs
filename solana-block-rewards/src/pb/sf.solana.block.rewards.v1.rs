// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<BlockReward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockReward {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(uint64, required, tag="2")]
    pub block_time: u64,
    #[prost(string, required, tag="3")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(int64, required, tag="5")]
    pub lamports: i64,
    #[prost(uint64, required, tag="6")]
    pub post_balance: u64,
    #[prost(int64, required, tag="7")]
    pub reward_type: i64,
    #[prost(string, required, tag="8")]
    pub commission: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
