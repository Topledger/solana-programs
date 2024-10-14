// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbFields {
    #[prost(string, required, tag="1")]
    pub k: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCurveLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub fields: ::core::option::Option<PbFields>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWindowedCircuitBreakerConfigV0Layout {
    #[prost(uint64, required, tag="1")]
    pub window_size_seconds: u64,
    #[prost(string, required, tag="2")]
    pub threshold_type: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="3")]
    pub threshold: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeTreasuryManagementArgsV0Layout {
    #[prost(string, required, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, required, tag="2")]
    pub curve: PbCurveLayout,
    #[prost(int64, required, tag="3")]
    pub freeze_unix_time: i64,
    #[prost(message, required, tag="4")]
    pub window_config: PbWindowedCircuitBreakerConfigV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateTreasuryManagementArgsV0Layout {
    #[prost(string, required, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, required, tag="2")]
    pub curve: PbCurveLayout,
    #[prost(int64, required, tag="3")]
    pub freeze_unix_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRedeemArgsV0Layout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
    #[prost(uint64, required, tag="2")]
    pub expected_output_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeTreasuryManagementV0Layout {
    #[prost(message, required, tag="1")]
    pub initialize_treasury_management_v0_args: PbInitializeTreasuryManagementArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateTreasuryManagementV0Layout {
    #[prost(message, required, tag="1")]
    pub update_treasury_management_v0_args: PbUpdateTreasuryManagementArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbRedeemV0Layout {
    #[prost(message, required, tag="1")]
    pub redeem_v0_args: PbRedeemArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize_treasury_management_v0: ::core::option::Option<PbInitializeTreasuryManagementV0Layout>,
    #[prost(message, optional, tag="3")]
    pub update_treasury_management_v0: ::core::option::Option<PbUpdateTreasuryManagementV0Layout>,
    #[prost(message, optional, tag="4")]
    pub redeem_v0: ::core::option::Option<PbRedeemV0Layout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub treasury_management: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub treasury_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub supply_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub circuit_breaker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub treasury: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub circuit_breaker_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub from: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub admin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub dest_treasury: ::core::option::Option<::prost::alloc::string::String>,
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
