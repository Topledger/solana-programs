// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<SplTokenMeta>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenMeta {
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
    #[prost(uint32, required, tag="6")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="7")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="8")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="9")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, required, tag="10")]
    pub input_accounts: Accounts,
    #[prost(message, required, tag="11")]
    pub args: Arg,
    #[prost(string, required, tag="12")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(string, optional, tag="13")]
    pub signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="14")]
    pub tx_index: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="15")]
    pub fee: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="16")]
    pub num_signers: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accounts {
    #[prost(string, optional, tag="1")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub rent_sysvar: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub signer_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub delegate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub fund_relocation_sys_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub funding_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub mint_funding_sys_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub destination_owner: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(uint64, optional, tag="1")]
    pub amount: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub authority_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub freeze_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub new_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub new_authority_option: ::core::option::Option<i32>,
    #[prost(string, optional, tag="7")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="8")]
    pub decimals: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub extension_type: ::core::option::Option<i32>,
    #[prost(double, optional, tag="10")]
    pub ui_amount: ::core::option::Option<f64>,
    #[prost(int32, optional, tag="11")]
    pub status: ::core::option::Option<i32>,
}
// @@protoc_insertion_point(module)
