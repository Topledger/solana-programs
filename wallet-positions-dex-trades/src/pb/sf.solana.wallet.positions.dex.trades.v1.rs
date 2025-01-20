// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<WalletPositionDexTradeData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletPositionDexTradeData {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub block_time: i64,
    #[prost(string, required, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub trader: ::prost::alloc::string::String,
    #[prost(double, required, tag="5")]
    pub trader_sol_change: f64,
    #[prost(string, required, tag="6")]
    pub buy_mint: ::prost::alloc::string::String,
    #[prost(string, required, tag="7")]
    pub sell_mint: ::prost::alloc::string::String,
    #[prost(double, required, tag="8")]
    pub buy_amount: f64,
    #[prost(double, required, tag="9")]
    pub sell_amount: f64,
    #[prost(uint64, required, tag="10")]
    pub txn_fee_lamports: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderTokenBalanceChange {
    #[prost(string, required, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(double, required, tag="2")]
    pub amount: f64,
}
// @@protoc_insertion_point(module)
