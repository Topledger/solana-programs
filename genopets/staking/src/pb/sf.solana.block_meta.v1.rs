// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbChangeTotalGeneAllocatedLayout {
    #[prost(uint64, required, tag="1")]
    pub new_total_gene_allocated: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbChangePoolWeightLayout {
    #[prost(uint32, required, tag="1")]
    pub new_pool_weight: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCreateGlobalStateLayout {
    #[prost(uint64, required, tag="1")]
    pub start_time: u64,
    #[prost(uint64, required, tag="2")]
    pub epoch_time: u64,
    #[prost(uint64, required, tag="3")]
    pub end_time: u64,
    #[prost(double, required, tag="4")]
    pub decay_factor_per_epoch: f64,
    #[prost(uint64, required, tag="5")]
    pub total_gene_allocated: u64,
    #[prost(uint64, optional, tag="6")]
    pub time_factor: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbCreateStakingPoolLayout {
    #[prost(uint64, required, tag="1")]
    pub earliest_unlock_date: u64,
    #[prost(uint32, required, tag="2")]
    pub pool_weight: u32,
    #[prost(uint32, required, tag="3")]
    pub weight_per_token: u32,
    #[prost(bool, required, tag="4")]
    pub governance_eligible: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStakeLayout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
    #[prost(uint32, required, tag="2")]
    pub lock_for_months: u32,
    #[prost(bool, required, tag="3")]
    pub as_sgene: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbReLockDepositLayout {
    #[prost(uint32, required, tag="1")]
    pub new_lock_for_months: u32,
    #[prost(bool, required, tag="2")]
    pub as_sgene: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbWithdrawLayout {
    #[prost(bool, required, tag="1")]
    pub as_sgene: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbClaimRewardsLayout {
    #[prost(bool, required, tag="1")]
    pub as_sgene: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(string, required, tag="1")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub change_total_gene_allocated: ::core::option::Option<PbChangeTotalGeneAllocatedLayout>,
    #[prost(message, optional, tag="3")]
    pub change_pool_weight: ::core::option::Option<PbChangePoolWeightLayout>,
    #[prost(message, optional, tag="4")]
    pub create_global_state: ::core::option::Option<PbCreateGlobalStateLayout>,
    #[prost(message, optional, tag="5")]
    pub create_staking_pool: ::core::option::Option<PbCreateStakingPoolLayout>,
    #[prost(message, optional, tag="6")]
    pub stake: ::core::option::Option<PbStakeLayout>,
    #[prost(message, optional, tag="7")]
    pub re_lock_deposit: ::core::option::Option<PbReLockDepositLayout>,
    #[prost(message, optional, tag="8")]
    pub withdraw: ::core::option::Option<PbWithdrawLayout>,
    #[prost(message, optional, tag="9")]
    pub claim_rewards: ::core::option::Option<PbClaimRewardsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccounts {
    #[prost(string, optional, tag="1")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub new_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub stake_master: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub staking_pool: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub sgene_minter: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub gene_rewarder: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub mint_sgene: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub gene_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub ata_gene_rewarder: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub associated_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub rent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub pool_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub staker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub ata_user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub ata_user_sgene: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub ata_vault: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub user_deposit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub user_re_deposit: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub deposit: ::core::option::Option<::prost::alloc::string::String>,
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
