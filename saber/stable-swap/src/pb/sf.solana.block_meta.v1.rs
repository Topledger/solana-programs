// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbFeesLayout {
    #[prost(uint64, required, tag="1")]
    pub admin_trade_fee_numerator: u64,
    #[prost(uint64, required, tag="2")]
    pub admin_trade_fee_denominator: u64,
    #[prost(uint64, required, tag="3")]
    pub admin_withdraw_fee_numerator: u64,
    #[prost(uint64, required, tag="4")]
    pub admin_withdraw_fee_denominator: u64,
    #[prost(uint64, required, tag="5")]
    pub trade_fee_numerator: u64,
    #[prost(uint64, required, tag="6")]
    pub trade_fee_denominator: u64,
    #[prost(uint64, required, tag="7")]
    pub withdraw_fee_numerator: u64,
    #[prost(uint64, required, tag="8")]
    pub withdraw_fee_denominator: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeLayout {
    #[prost(uint32, required, tag="1")]
    pub nonce: u32,
    #[prost(uint64, required, tag="2")]
    pub amp_factor: u64,
    #[prost(message, required, tag="3")]
    pub fees: PbFeesLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwapLayout {
    #[prost(uint64, required, tag="1")]
    pub amount_in: u64,
    #[prost(uint64, required, tag="2")]
    pub minimum_amount_out: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDepositLayout {
    #[prost(uint64, required, tag="1")]
    pub token_a_amount: u64,
    #[prost(uint64, required, tag="2")]
    pub token_b_amount: u64,
    #[prost(uint64, required, tag="3")]
    pub min_mint_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawLayout {
    #[prost(uint64, required, tag="1")]
    pub pool_token_amount: u64,
    #[prost(uint64, required, tag="2")]
    pub minimum_token_a_amount: u64,
    #[prost(uint64, required, tag="3")]
    pub minimum_token_b_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawOneLayout {
    #[prost(uint64, required, tag="1")]
    pub pool_token_amount: u64,
    #[prost(uint64, required, tag="2")]
    pub minimum_token_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize: ::core::option::Option<PbInitializeLayout>,
    #[prost(message, optional, tag="3")]
    pub swap: ::core::option::Option<PbSwapLayout>,
    #[prost(message, optional, tag="4")]
    pub deposit: ::core::option::Option<PbDepositLayout>,
    #[prost(message, optional, tag="5")]
    pub withdraw: ::core::option::Option<PbWithdrawLayout>,
    #[prost(message, optional, tag="6")]
    pub withdraw_one: ::core::option::Option<PbWithdrawOneLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub new_stable_swap: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub admin_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub admin_fee_aadmin_fee_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub admin_fee_badmin_fee_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub token_a_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub token_b_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub pool_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub stable_swap: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub user_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub token_ab_source_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub token_ab_base_account_swap_into: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub token_ab_base_account_swap_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub token_ab_destination_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub token_ab_admin_fee_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub token_program_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub token_a_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub token_b_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub token_a_base_account_deposit_into: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub token_b_base_account_deposit_into: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub pool_mint_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub pool_account_deposit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub source_pool_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub token_a_swap_account_withdraw_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub token_b_swap_account_withdraw_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub token_a_user_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub token_b_user_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub admin_fee_a_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="29")]
    pub admin_fee_b_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="30")]
    pub token_ab_base_swap_account_withdraw_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
    pub token_ab_quote_swap_account_exchange: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="32")]
    pub token_ab_base_user_account_credit: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub block_time: i64,
    #[prost(string, required, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, required, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="10")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, required, tag="11")]
    pub args: Arg,
    #[prost(message, required, tag="12")]
    pub input_accounts: InputAccounts,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Meta>,
}
// @@protoc_insertion_point(module)
