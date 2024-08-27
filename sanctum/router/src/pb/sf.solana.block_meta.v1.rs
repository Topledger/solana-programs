// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwapViaStakeArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
    #[prost(uint32, required, tag="2")]
    pub bridge_stake_seed: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStakeWrappedSolLayout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSwapViaStakeLayout {
    #[prost(message, required, tag="1")]
    pub args: PbSwapViaStakeArgsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPrefundWithdrawStakeLayout {
    #[prost(message, required, tag="1")]
    pub args: PbSwapViaStakeArgsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPrefundSwapViaStakeLayout {
    #[prost(message, required, tag="1")]
    pub args: PbSwapViaStakeArgsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub stake_wrapped_sol: ::core::option::Option<PbStakeWrappedSolLayout>,
    #[prost(message, optional, tag="3")]
    pub swap_via_stake: ::core::option::Option<PbSwapViaStakeLayout>,
    #[prost(message, optional, tag="4")]
    pub prefund_withdraw_stake: ::core::option::Option<PbPrefundWithdrawStakeLayout>,
    #[prost(message, optional, tag="5")]
    pub prefund_swap_via_stake: ::core::option::Option<PbPrefundSwapViaStakeLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub wsol_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub dest_token_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub wsol_bridge_in: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub sol_bridge_out: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub dest_token_fee_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub dest_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub wsol_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub src_token_from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub bridge_stake: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub src_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub fee_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub admin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub close_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub withdraw_to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub stake_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub prefunder: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub slumdog_stake: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub unstakeit_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub unstake_pool: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub pool_sol_reserves: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub unstake_fee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub slumdog_stake_acc_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub unstake_protocol_fee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="29")]
    pub unstake_protocol_fee_dest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="30")]
    pub clock: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
    pub stake_program: ::core::option::Option<::prost::alloc::string::String>,
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
