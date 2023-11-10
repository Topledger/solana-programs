// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TradeData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeData {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub block_time: i64,
    #[prost(uint64, required, tag="3")]
    pub block_slot: u64,
    #[prost(string, required, tag="4")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="6")]
    pub txn_fee: u64,
    #[prost(string, required, tag="7")]
    pub merkle_tree: ::prost::alloc::string::String,
    #[prost(string, required, tag="8")]
    pub leaf_id: ::prost::alloc::string::String,
    #[prost(double, required, tag="9")]
    pub amount: f64,
    #[prost(string, required, tag="10")]
    pub currency: ::prost::alloc::string::String,
    #[prost(string, required, tag="11")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, required, tag="12")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, required, tag="13")]
    pub seller: ::prost::alloc::string::String,
    #[prost(double, required, tag="14")]
    pub taker_fee: f64,
    #[prost(double, required, tag="15")]
    pub maker_fee: f64,
    #[prost(double, required, tag="16")]
    pub amm_fee: f64,
    #[prost(double, required, tag="17")]
    pub royalty: f64,
    #[prost(string, required, tag="18")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="19")]
    pub instruction_index: u32,
    #[prost(string, required, tag="20")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(string, required, tag="21")]
    pub platform: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
