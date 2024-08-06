// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeLazyTransactionsArgsV0Layout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub root: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="4")]
    pub max_depth: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCompiledInstructionLayout {
    #[prost(uint32, required, tag="1")]
    pub program_id_index: u32,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetCanopyArgsV0Layout {
    #[prost(uint32, required, tag="1")]
    pub offset: u32,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub bytes: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExecuteTransactionArgsV0Layout {
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<PbCompiledInstructionLayout>,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub signer_seeds: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag="3")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCloseMarkerArgsV0Layout {
    #[prost(uint32, required, tag="1")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateLazyTransactionsArgsV0Layout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub root: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, optional, tag="2")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeLazyTransactionsV0Layout {
    #[prost(message, required, tag="1")]
    pub initialize_lazy_transactions_v0_args: PbInitializeLazyTransactionsArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExecuteTransactionV0Layout {
    #[prost(message, required, tag="1")]
    pub execute_transaction_v0_args: PbExecuteTransactionArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCloseMarkerV0Layout {
    #[prost(message, required, tag="1")]
    pub close_marker_v0_args: PbCloseMarkerArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateLazyTransactionsV0Layout {
    #[prost(message, required, tag="1")]
    pub update_lazy_transactions_v0_args: PbUpdateLazyTransactionsArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetCanopyV0Layout {
    #[prost(message, required, tag="1")]
    pub set_canopy_v0_args: PbSetCanopyArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize_lazy_transactions_v0: ::core::option::Option<PbInitializeLazyTransactionsV0Layout>,
    #[prost(message, optional, tag="3")]
    pub execute_transaction_v0: ::core::option::Option<PbExecuteTransactionV0Layout>,
    #[prost(message, optional, tag="4")]
    pub close_marker_v0: ::core::option::Option<PbCloseMarkerV0Layout>,
    #[prost(message, optional, tag="5")]
    pub update_lazy_transactions_v0: ::core::option::Option<PbUpdateLazyTransactionsV0Layout>,
    #[prost(message, optional, tag="6")]
    pub set_canopy_v0: ::core::option::Option<PbSetCanopyV0Layout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub lazy_transactions: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub canopy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub lazy_signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub block: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub refund: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
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
