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
    #[prost(string, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub block_time: i64,
    #[prost(string, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(message, optional, tag="10")]
    pub arg: ::core::option::Option<Arg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub authority_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub freeze_authority: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub freeze_authority_option: i32,
    #[prost(string, tag="6")]
    pub mint_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub new_authority: ::prost::alloc::string::String,
    #[prost(int32, tag="8")]
    pub new_authority_option: i32,
    #[prost(string, tag="9")]
    pub owner: ::prost::alloc::string::String,
    #[prost(int32, tag="10")]
    pub decimals: i32,
    #[prost(int32, tag="11")]
    pub extension_type: i32,
    #[prost(string, tag="12")]
    pub ui_amount: ::prost::alloc::string::String,
    #[prost(int32, tag="13")]
    pub status: i32,
}
// @@protoc_insertion_point(module)
