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
    #[prost(double, required, tag="6")]
    pub amount: f64,
    #[prost(string, required, tag="7")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(string, required, tag="8")]
    pub stake_pool: ::prost::alloc::string::String,
    #[prost(string, required, tag="9")]
    pub withdraw_authority: ::prost::alloc::string::String,
    #[prost(string, required, tag="10")]
    pub reserve_stake: ::prost::alloc::string::String,
    #[prost(string, required, tag="11")]
    pub validator_stake: ::prost::alloc::string::String,
    #[prost(string, required, tag="12")]
    pub pool_mint: ::prost::alloc::string::String,
    #[prost(string, required, tag="13")]
    pub fee_account: ::prost::alloc::string::String,
    #[prost(double, required, tag="14")]
    pub withdraw_fee: f64,
    #[prost(double, required, tag="15")]
    pub mint_amount: f64,
    #[prost(double, required, tag="16")]
    pub burn_amount: f64,
    #[prost(double, required, tag="17")]
    pub fee_amount: f64,
    #[prost(bool, required, tag="18")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="19")]
    pub instruction_index: u32,
    #[prost(uint32, required, tag="20")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="21")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(string, required, tag="22")]
    pub inner_program: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="23")]
    pub txn_fee: u64,
    #[prost(double, required, tag="24")]
    pub staking_reward: f64,
}
// @@protoc_insertion_point(module)