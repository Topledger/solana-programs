// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<BlockStat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStat {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub previous_block_hash: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="4")]
    pub parent_slot: u64,
    #[prost(uint64, required, tag="6")]
    pub block_time: u64,
    #[prost(uint64, required, tag="7")]
    pub block_height: u64,
    #[prost(uint64, required, tag="8")]
    pub block_slot: u64,
    #[prost(uint64, required, tag="9")]
    pub total_transactions: u64,
    #[prost(uint64, required, tag="10")]
    pub total_fee: u64,
    #[prost(uint64, required, tag="11")]
    pub successful_transactions: u64,
    #[prost(uint64, required, tag="12")]
    pub failed_transactions: u64,
    #[prost(uint64, required, tag="13")]
    pub total_vote_transactions: u64,
    #[prost(uint64, required, tag="14")]
    pub total_non_vote_transactions: u64,
    #[prost(uint64, required, tag="15")]
    pub successful_vote_transactions: u64,
    #[prost(uint64, required, tag="16")]
    pub successful_vote_transactions_fee: u64,
    #[prost(uint64, required, tag="17")]
    pub successful_non_vote_transactions: u64,
    #[prost(uint64, required, tag="18")]
    pub successful_non_vote_transactions_fee: u64,
    #[prost(uint64, required, tag="19")]
    pub successful_non_vote_transactions_priority_fee: u64,
    #[prost(uint64, required, tag="20")]
    pub failed_vote_transactions: u64,
    #[prost(uint64, required, tag="21")]
    pub failed_vote_transactions_fee: u64,
    #[prost(uint64, required, tag="22")]
    pub failed_non_vote_transactions: u64,
    #[prost(uint64, required, tag="23")]
    pub failed_non_vote_transactions_fee: u64,
}
// @@protoc_insertion_point(module)
