// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawParamsLayout {
    #[prost(uint64, required, tag="1")]
    pub withdraw_amount: u64,
    #[prost(string, required, tag="2")]
    pub withdrawal: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbOpenDcaLayout {
    #[prost(uint64, required, tag="1")]
    pub application_idx: u64,
    #[prost(uint64, required, tag="2")]
    pub in_amount: u64,
    #[prost(uint64, required, tag="3")]
    pub in_amount_per_cycle: u64,
    #[prost(double, required, tag="4")]
    pub cycle_frequency: f64,
    #[prost(uint64, optional, tag="5")]
    pub min_price: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub max_price: ::core::option::Option<u64>,
    #[prost(double, optional, tag="7")]
    pub start_at: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="8")]
    pub close_wsol_in_ata: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawLayout {
    #[prost(message, required, tag="1")]
    pub withdraw_params: PbWithdrawParamsLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDepositLayout {
    #[prost(uint64, required, tag="1")]
    pub deposit_in: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawFeesLayout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbFulfillFlashFillLayout {
    #[prost(uint64, required, tag="1")]
    pub repay_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbOpenDcaV2Layout {
    #[prost(uint64, required, tag="1")]
    pub application_idx: u64,
    #[prost(uint64, required, tag="2")]
    pub in_amount: u64,
    #[prost(uint64, required, tag="3")]
    pub in_amount_per_cycle: u64,
    #[prost(double, required, tag="4")]
    pub cycle_frequency: f64,
    #[prost(uint64, optional, tag="5")]
    pub min_out_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub max_out_amount: ::core::option::Option<u64>,
    #[prost(double, optional, tag="7")]
    pub start_at: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbFulfillDlmmFillLayout {
    #[prost(uint64, required, tag="1")]
    pub repay_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub open_dca: ::core::option::Option<PbOpenDcaLayout>,
    #[prost(message, optional, tag="3")]
    pub withdraw: ::core::option::Option<PbWithdrawLayout>,
    #[prost(message, optional, tag="4")]
    pub deposit: ::core::option::Option<PbDepositLayout>,
    #[prost(message, optional, tag="5")]
    pub withdraw_fees: ::core::option::Option<PbWithdrawFeesLayout>,
    #[prost(message, optional, tag="6")]
    pub fulfill_flash_fill: ::core::option::Option<PbFulfillFlashFillLayout>,
    #[prost(message, optional, tag="7")]
    pub open_dca_v2: ::core::option::Option<PbOpenDcaV2Layout>,
    #[prost(message, optional, tag="8")]
    pub fulfill_dlmm_fill: ::core::option::Option<PbFulfillDlmmFillLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub dca: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub input_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub output_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub user_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub in_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub out_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub event_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub user_in_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub user_out_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub dca_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub admin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub fee_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub program_fee_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub admin_fee_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub keeper: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub keeper_in_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub instructions_sysvar: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub fee_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub dca_out_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub intermediate_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub init_user_out_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
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
