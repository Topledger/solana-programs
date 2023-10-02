// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fees {
    #[prost(uint64, tag="1")]
    pub trade_fee_numerator: u64,
    #[prost(uint64, tag="2")]
    pub trade_fee_denominator: u64,
    #[prost(uint64, tag="3")]
    pub owner_trade_fee_numerator: u64,
    #[prost(uint64, tag="4")]
    pub owner_trade_fee_denominator: u64,
    #[prost(uint64, tag="5")]
    pub owner_withdraw_fee_numerator: u64,
    #[prost(uint64, tag="6")]
    pub owner_withdraw_fee_denominator: u64,
    #[prost(uint64, tag="7")]
    pub host_fee_numerator: u64,
    #[prost(uint64, tag="8")]
    pub host_fee_denominator: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapCurve {
    #[prost(string, tag="1")]
    pub curve_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub nonce: u32,
    #[prost(message, optional, tag="3")]
    pub fees: ::core::option::Option<Fees>,
    #[prost(message, optional, tag="4")]
    pub swap_curve: ::core::option::Option<SwapCurve>,
    #[prost(uint64, tag="5")]
    pub amount_in: u64,
    #[prost(uint64, tag="6")]
    pub minimum_amount_out: u64,
    #[prost(uint64, tag="7")]
    pub pool_token_amount: u64,
    #[prost(uint64, tag="8")]
    pub maximum_token_a_amount: u64,
    #[prost(uint64, tag="9")]
    pub maximum_token_b_amount: u64,
    #[prost(uint64, tag="10")]
    pub source_token_amount: u64,
    #[prost(uint64, tag="11")]
    pub minimum_pool_token_amount: u64,
    #[prost(uint64, tag="12")]
    pub destination_token_amount: u64,
    #[prost(uint64, tag="13")]
    pub maximum_pool_token_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenSwapMeta {
    #[prost(string, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub block_time: i64,
    #[prost(string, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(message, optional, tag="10")]
    pub arg: ::core::option::Option<Arg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<SplTokenSwapMeta>,
}
// @@protoc_insertion_point(module)
