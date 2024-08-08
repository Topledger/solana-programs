// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbOracleConfigV0Layout {
    #[prost(string, required, tag="1")]
    pub oracle: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub url: ::prost::alloc::string::String,
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
pub struct PbInitializeLazyDistributorArgsV0Layout {
    #[prost(message, repeated, tag="1")]
    pub oracles: ::prost::alloc::vec::Vec<PbOracleConfigV0Layout>,
    #[prost(string, required, tag="2")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, required, tag="3")]
    pub window_config: PbWindowedCircuitBreakerConfigV0Layout,
    #[prost(string, optional, tag="4")]
    pub approver: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeCompressionRecipientArgsV0Layout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub data_hash: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub creator_hash: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag="4")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetCurrentRewardsArgsV0Layout {
    #[prost(uint32, required, tag="1")]
    pub oracle_index: u32,
    #[prost(uint64, required, tag="2")]
    pub current_rewards: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDistributeCompressionRewardsArgsV0Layout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub data_hash: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub creator_hash: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag="4")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateLazyDistributorArgsV0Layout {
    #[prost(message, repeated, tag="1")]
    pub oracles: ::prost::alloc::vec::Vec<PbOracleConfigV0Layout>,
    #[prost(string, optional, tag="2")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub approver: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeLazyDistributorV0Layout {
    #[prost(message, required, tag="1")]
    pub initialize_lazy_distributor_v0_args: PbInitializeLazyDistributorArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeCompressionRecipientV0Layout {
    #[prost(message, required, tag="1")]
    pub initialize_compression_recipient_v0_args: PbInitializeCompressionRecipientArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbSetCurrentRewardsV0Layout {
    #[prost(message, required, tag="1")]
    pub set_current_rewards_v0_args: PbSetCurrentRewardsArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDistributeCompressionRewardsV0Layout {
    #[prost(message, required, tag="1")]
    pub distribute_compression_rewards_v0_args: PbDistributeCompressionRewardsArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbUpdateLazyDistributorV0Layout {
    #[prost(message, required, tag="1")]
    pub update_lazy_distributor_v0_args: PbUpdateLazyDistributorArgsV0Layout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub initialize_lazy_distributor_v0: ::core::option::Option<PbInitializeLazyDistributorV0Layout>,
    #[prost(message, optional, tag="3")]
    pub initialize_compression_recipient_v0: ::core::option::Option<PbInitializeCompressionRecipientV0Layout>,
    #[prost(message, optional, tag="4")]
    pub set_current_rewards_v0: ::core::option::Option<PbSetCurrentRewardsV0Layout>,
    #[prost(message, optional, tag="5")]
    pub distribute_compression_rewards_v0: ::core::option::Option<PbDistributeCompressionRewardsV0Layout>,
    #[prost(message, optional, tag="6")]
    pub update_lazy_distributor_v0: ::core::option::Option<PbUpdateLazyDistributorV0Layout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub lazy_distributor: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub rewards_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub rewards_escrow: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub circuit_breaker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub circuit_breaker_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub recipient: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub target_metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub merkle_tree: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub delegate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub compression_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub oracle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub common_payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub common_lazy_distributor: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub common_recipient: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub common_rewards_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub common_rewards_escrow: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub common_circuit_breaker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub common_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub common_destination_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="26")]
    pub common_associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="27")]
    pub common_circuit_breaker_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="28")]
    pub common_system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="29")]
    pub common_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="30")]
    pub recipient_mint_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
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
