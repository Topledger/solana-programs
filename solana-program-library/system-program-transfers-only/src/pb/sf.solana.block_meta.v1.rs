// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTransfer {
    #[prost(uint64, required, tag="1")]
    pub lamports: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTransferWithSeed {
    #[prost(uint64, required, tag="1")]
    pub lamports: u64,
    #[prost(string, required, tag="2")]
    pub from_seed: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub from_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transfer: ::core::option::Option<PbTransfer>,
    #[prost(message, optional, tag="3")]
    pub transfer_with_seed: ::core::option::Option<PbTransferWithSeed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub funding_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub recipient_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub base_account: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(string, required, tag="6")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag="10")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(string, required, tag="11")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(message, required, tag="12")]
    pub args: Arg,
    #[prost(message, required, tag="13")]
    pub input_accounts: InputAccounts,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Meta>,
}
// @@protoc_insertion_point(module)
