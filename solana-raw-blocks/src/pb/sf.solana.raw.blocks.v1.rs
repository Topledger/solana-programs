// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    #[prost(string, required, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="2")]
    pub lamports: u64,
    #[prost(uint64, required, tag="3")]
    pub post_balance: u64,
    #[prost(uint64, required, tag="4")]
    pub reward_type: u64,
    #[prost(string, required, tag="5")]
    pub commission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStat {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub previous_block_hash: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="4")]
    pub parent_slot: u64,
    #[prost(message, repeated, tag="5")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
    #[prost(uint64, required, tag="6")]
    pub block_time: u64,
    #[prost(uint64, required, tag="7")]
    pub block_height: u64,
    #[prost(uint64, required, tag="8")]
    pub slot: u64,
}
// @@protoc_insertion_point(module)
