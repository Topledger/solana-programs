// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAdminSettingsLayout {
    #[prost(bool, required, tag="1")]
    pub swap_enabled: bool,
    #[prost(bool, required, tag="2")]
    pub add_liquidity_enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeLayout {
    #[prost(uint32, required, tag="1")]
    pub nonce: u32,
    #[prost(uint64, required, tag="2")]
    pub amplification_coefficient: u64,
    #[prost(uint64, required, tag="3")]
    pub fee_numerator: u64,
    #[prost(uint32, required, tag="4")]
    pub n_coins: u32,
    #[prost(message, required, tag="5")]
    pub admin_settings: PbAdminSettingsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbAddLiquidityLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub deposit_amounts: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, required, tag="2")]
    pub min_mint_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRemoveLiquidityLayout {
    #[prost(uint64, required, tag="1")]
    pub unmint_amount: u64,
    #[prost(uint64, repeated, packed="false", tag="2")]
    pub minimum_amounts: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRemoveLiquidityOneTokenLayout {
    #[prost(uint64, required, tag="1")]
    pub unmint_amount: u64,
    #[prost(uint64, required, tag="2")]
    pub minimum_out_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExchangeLayout {
    #[prost(uint64, required, tag="1")]
    pub in_amount: u64,
    #[prost(uint64, required, tag="2")]
    pub minimum_out_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetAdminSettingLayout {
    #[prost(message, required, tag="1")]
    pub admin_setting: PbAdminSettingsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize: ::core::option::Option<PbInitializeLayout>,
    #[prost(message, optional, tag="3")]
    pub add_liquidity: ::core::option::Option<PbAddLiquidityLayout>,
    #[prost(message, optional, tag="4")]
    pub remove_liquidity: ::core::option::Option<PbRemoveLiquidityLayout>,
    #[prost(message, optional, tag="5")]
    pub remove_liquidity_one_token: ::core::option::Option<PbRemoveLiquidityOneTokenLayout>,
    #[prost(message, optional, tag="6")]
    pub exchange: ::core::option::Option<PbExchangeLayout>,
    #[prost(message, optional, tag="7")]
    pub set_admin_setting: ::core::option::Option<PbSetAdminSettingLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub swap_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub token_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub token_mints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub pool_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub admin_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub token_program_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub user_transfer_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub source_token_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub user_lp_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub source_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub destination_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub admin_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub admin_nft_owner: ::core::option::Option<::prost::alloc::string::String>,
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
