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
    #[prost(string, required, tag="6")]
    pub pool: ::prost::alloc::string::String,
    #[prost(string, required, tag="7")]
    pub mint_a: ::prost::alloc::string::String,
    #[prost(string, required, tag="8")]
    pub mint_b: ::prost::alloc::string::String,
    #[prost(string, required, tag="11")]
    pub account_a: ::prost::alloc::string::String,
    #[prost(string, required, tag="12")]
    pub account_b: ::prost::alloc::string::String,
    #[prost(double, required, tag="15")]
    pub token_a_amount: f64,
    #[prost(double, required, tag="16")]
    pub token_b_amount: f64,
    #[prost(string, required, tag="19")]
    pub lp_wallet: ::prost::alloc::string::String,
    #[prost(string, required, tag="20")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(bool, required, tag="21")]
    pub is_inner_instruction: bool,
    #[prost(string, required, tag="22")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="23")]
    pub instruction_index: u32,
    #[prost(string, required, tag="24")]
    pub inner_program: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="25")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="26")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, required, tag="27")]
    pub liquidity_index: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
