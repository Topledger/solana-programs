// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbEscrowNftLayout {
    #[prost(uint64, required, tag="1")]
    pub unit_price: u64,
    #[prost(uint64, required, tag="2")]
    pub fee: u64,
    #[prost(uint32, required, tag="3")]
    pub bump: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub escrow_nft: ::core::option::Option<PbEscrowNftLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub helio_signature_wallet: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub owner_nft_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub escrow_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub escrow_nft_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub nft_metadata_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub escrow_pda: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub sysvar_instructions: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub nft_master_edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub owner_token_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub destination_token_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub auth_rules_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub auth_rules: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub metaplex_metadata_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub sender_nft_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub sender_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub recipient: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub recipient_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub helio_fee_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub dao_fee_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub helio_fee_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub dao_fee_account: ::core::option::Option<::prost::alloc::string::String>,
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
