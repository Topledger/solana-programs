// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPubkeyLayout {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCreateLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, required, tag="2")]
    pub space: u32,
    #[prost(uint64, optional, tag="3")]
    pub expires_at: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, required, tag="2")]
    pub offset: u32,
    #[prost(uint64, repeated, packed="false", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbTransferLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, required, tag="2")]
    pub name_account_bump: u32,
    #[prost(string, required, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDeleteLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, required, tag="2")]
    pub name_account_bump: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbResizeLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, required, tag="2")]
    pub new_size: u32,
    #[prost(uint32, required, tag="3")]
    pub name_account_bump: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExtendLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, required, tag="2")]
    pub expires_at: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbImmutableOwnerLayout {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub hashed_name: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub create: ::core::option::Option<PbCreateLayout>,
    #[prost(message, optional, tag="3")]
    pub update: ::core::option::Option<PbUpdateLayout>,
    #[prost(message, optional, tag="4")]
    pub transfer: ::core::option::Option<PbTransferLayout>,
    #[prost(message, optional, tag="5")]
    pub delete: ::core::option::Option<PbDeleteLayout>,
    #[prost(message, optional, tag="6")]
    pub resize: ::core::option::Option<PbResizeLayout>,
    #[prost(message, optional, tag="7")]
    pub extend: ::core::option::Option<PbExtendLayout>,
    #[prost(message, optional, tag="8")]
    pub immutable_owner: ::core::option::Option<PbImmutableOwnerLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub payer_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub name_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub name_class_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub name_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub parent_name_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub parent_name_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub refund_target: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
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
