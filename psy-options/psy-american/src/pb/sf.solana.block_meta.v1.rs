// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeMarketLayout {
    #[prost(uint64, required, tag="1")]
    pub underlying_amount_per_contract: u64,
    #[prost(uint64, required, tag="2")]
    pub quote_amount_per_contract: u64,
    #[prost(double, required, tag="3")]
    pub expiration_unix_timestamp: f64,
    #[prost(uint32, required, tag="4")]
    pub bump_seed: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMintOptionLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMintOptionV2Layout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseOptionLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbExerciseOptionV2Layout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbClosePostExpirationLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCloseOptionPositionLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbBurnWriterForQuoteLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitSerumMarketLayout {
    #[prost(uint64, required, tag="1")]
    pub market_space: u64,
    #[prost(uint64, required, tag="2")]
    pub vault_signer_nonce: u64,
    #[prost(uint64, required, tag="3")]
    pub coin_lot_size: u64,
    #[prost(uint64, required, tag="4")]
    pub pc_lot_size: u64,
    #[prost(uint64, required, tag="5")]
    pub pc_dust_threshold: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize_market: ::core::option::Option<PbInitializeMarketLayout>,
    #[prost(message, optional, tag="3")]
    pub mint_option: ::core::option::Option<PbMintOptionLayout>,
    #[prost(message, optional, tag="4")]
    pub mint_option_v2: ::core::option::Option<PbMintOptionV2Layout>,
    #[prost(message, optional, tag="5")]
    pub exercise_option: ::core::option::Option<PbExerciseOptionLayout>,
    #[prost(message, optional, tag="6")]
    pub exercise_option_v2: ::core::option::Option<PbExerciseOptionV2Layout>,
    #[prost(message, optional, tag="7")]
    pub close_post_expiration: ::core::option::Option<PbClosePostExpirationLayout>,
    #[prost(message, optional, tag="8")]
    pub close_option_position: ::core::option::Option<PbCloseOptionPositionLayout>,
    #[prost(message, optional, tag="9")]
    pub burn_writer_for_quote: ::core::option::Option<PbBurnWriterForQuoteLayout>,
    #[prost(message, optional, tag="10")]
    pub init_serum_market: ::core::option::Option<PbInitSerumMarketLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub underlying_asset_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub quote_asset_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub option_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub writer_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub quote_asset_pool: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub underlying_asset_pool: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub option_market: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub fee_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub rent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub clock: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub user_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub underlying_asset_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub minted_option_dest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub minted_writer_token_dest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub option_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub exerciser_option_token_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub underlying_asset_dest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub quote_asset_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub writer_token_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub option_token_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub option_token_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub writer_quote_dest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub serum_market: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub dex_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="29")]
    pub pc_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="30")]
    pub request_queue: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
    pub event_queue: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="32")]
    pub bids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="33")]
    pub asks: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="34")]
    pub coin_vault: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="35")]
    pub pc_vault: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="36")]
    pub vault_signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="37")]
    pub market_authority: ::core::option::Option<::prost::alloc::string::String>,
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
