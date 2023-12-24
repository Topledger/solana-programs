// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<AccountStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub mint: ::prost::alloc::string::String,
    #[prost(double, required, tag="5")]
    pub pre_balance: f64,
    #[prost(double, required, tag="6")]
    pub post_balance: f64,
    #[prost(double, required, tag="7")]
    pub balance_change: f64,
}
// @@protoc_insertion_point(module)
