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
    #[prost(uint64, required, tag="5")]
    pub txn_fee: u64,
    #[prost(string, required, tag="6")]
    pub mint: ::prost::alloc::string::String,
    #[prost(double, required, tag="7")]
    pub amount: f64,
    #[prost(string, required, tag="8")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, required, tag="9")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, required, tag="10")]
    pub seller: ::prost::alloc::string::String,
    #[prost(double, required, tag="11")]
    pub taker_fee: f64,
    #[prost(double, required, tag="12")]
    pub maker_fee: f64,
    #[prost(double, required, tag="13")]
    pub amm_fee: f64,
    #[prost(double, required, tag="14")]
    pub royalty: f64,
    #[prost(string, required, tag="15")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="16")]
    pub instruction_index: u32,
    #[prost(string, required, tag="17")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(string, required, tag="18")]
    pub inner_program: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="19")]
    pub inner_instruxtion_index: u32,
    #[prost(bool, required, tag="20")]
    pub is_inner_instruction: bool,
    #[prost(string, required, tag="21")]
    pub platform: ::prost::alloc::string::String,
    #[prost(string, required, tag="22")]
    pub currency_mint: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
