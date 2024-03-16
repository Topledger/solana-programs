// @generated
use serde::{Serialize, Deserialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct BlockStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(uint64, required, tag="2")]
    pub block_time: u64,
    #[prost(string, required, tag="3")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="4")]
    pub height: u64,
    #[prost(string, required, tag="5")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="6")]
    pub total_transactions: u64,
    #[prost(uint64, required, tag="7")]
    pub successful_transactions: u64,
    #[prost(uint64, required, tag="8")]
    pub failed_transactions: u64,
    #[prost(uint64, required, tag="9")]
    pub total_vote_transactions: u64,
    #[prost(uint64, required, tag="10")]
    pub total_non_vote_transactions: u64,
    #[prost(uint64, required, tag="11")]
    pub successful_vote_transactions: u64,
    #[prost(uint64, required, tag="14")]
    pub successful_non_vote_transactions: u64,
    #[prost(uint64, required, tag="15")]
    pub failed_non_vote_transactions: u64,
    #[prost(uint64, required, tag="16")]
    pub parent_slot: u64,
    #[prost(string, required, tag="17")]
    pub previous_block_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="18")]
    pub block_rewards: ::prost::alloc::vec::Vec<Reward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
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
// @@protoc_insertion_point(module)
