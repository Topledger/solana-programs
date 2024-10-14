// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCreateNftLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbEditDataLayout {
    #[prost(uint32, required, tag="1")]
    pub offset: u32,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub create_nft: ::core::option::Option<PbCreateNftLayout>,
    #[prost(message, optional, tag="3")]
    pub edit_data: ::core::option::Option<PbEditDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub name_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub central_state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub spl_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub rent_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub fee_payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub collection_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub metadata_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub central_state_nft_ata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub metadata_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub spl_name_service_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub ata_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub nft_destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub nft_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub name_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub edition_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub collection_metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub metadata_signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub nft_source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub nft_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub nft: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub token_destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub token_source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub nft_account: ::core::option::Option<::prost::alloc::string::String>,
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
