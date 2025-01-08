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
    #[prost(string, required, tag="3")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(string, required, tag="6")]
    pub associated_account: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
