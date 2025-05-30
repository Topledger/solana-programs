// This file is @generated by prost-build.
/// Placeholder Meta message - Adapt from DLMM or define as needed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    #[prost(string, tag = "1")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_slot: u64,
    #[prost(int64, tag = "3")]
    pub block_time: i64,
    #[prost(string, tag = "4")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "5")]
    pub instruction_index: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub is_inner_instruction: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub inner_instruction_index: ::core::option::Option<u32>,
    /// Signer pubkey
    #[prost(string, optional, tag = "8")]
    pub signer: ::core::option::Option<::prost::alloc::string::String>,
    /// Outer program ID if inner instruction
    #[prost(string, optional, tag = "9")]
    pub outer_program: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the instruction
    #[prost(string, tag = "10")]
    pub instruction_type: ::prost::alloc::string::String,
    /// Map of roles to account pubkeys
    #[prost(map = "string, string", tag = "11")]
    pub input_accounts: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Parsed instruction arguments
    #[prost(message, optional, tag = "12")]
    pub args: ::core::option::Option<InstructionArgs>,
}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbInitializeLayout {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbEnableVaultLayout {
    /// Corresponds to u8, using u32 for Protobuf compatibility
    #[prost(uint32, optional, tag = "1")]
    pub enabled: ::core::option::Option<u32>,
}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbSetOperatorLayout {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbStrategyBumpsLayout {
    /// u8 -> u32
    #[prost(uint32, optional, tag = "1")]
    pub strategy_index: ::core::option::Option<u32>,
    /// u8 array -> repeated u32
    #[prost(uint32, repeated, tag = "2")]
    pub other_bumps: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbInitializeStrategyLayout {
    #[prost(message, optional, tag = "1")]
    pub bumps: ::core::option::Option<PbStrategyBumpsLayout>,
    #[prost(enumeration = "PbStrategyType", optional, tag = "2")]
    pub strategy_type: ::core::option::Option<i32>,
}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbRemoveStrategyLayout {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbRemoveStrategy2Layout {
    #[prost(uint64, optional, tag = "1")]
    pub max_admin_pay_amount: ::core::option::Option<u64>,
}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbCollectDustLayout {}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbAddStrategyLayout {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbDepositStrategyLayout {
    #[prost(uint64, optional, tag = "1")]
    pub amount: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdrawStrategyLayout {
    #[prost(uint64, optional, tag = "1")]
    pub amount: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdraw2Layout {
    #[prost(uint64, optional, tag = "1")]
    pub unmint_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub min_out_amount: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbDepositLayout {
    #[prost(uint64, optional, tag = "1")]
    pub token_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub minimum_lp_token_amount: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdrawLayout {
    #[prost(uint64, optional, tag = "1")]
    pub unmint_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub min_out_amount: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdrawDirectlyFromStrategyLayout {
    #[prost(uint64, optional, tag = "1")]
    pub unmint_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub min_out_amount: ::core::option::Option<u64>,
}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbClaimRewardsLayout {}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbPatchVaultFieldInStrategyLayout {}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbWithdrawMangoLayout {}
/// No arguments
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PbIdlWriteLayout {}
/// Wrapper for instruction arguments using oneof
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionArgs {
    #[prost(
        oneof = "instruction_args::InstructionArgs",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18"
    )]
    pub instruction_args: ::core::option::Option<instruction_args::InstructionArgs>,
}
/// Nested message and enum types in `InstructionArgs`.
pub mod instruction_args {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InstructionArgs {
        #[prost(message, tag = "1")]
        Initialize(super::PbInitializeLayout),
        #[prost(message, tag = "2")]
        EnableVault(super::PbEnableVaultLayout),
        #[prost(message, tag = "3")]
        SetOperator(super::PbSetOperatorLayout),
        #[prost(message, tag = "4")]
        InitializeStrategy(super::PbInitializeStrategyLayout),
        #[prost(message, tag = "5")]
        RemoveStrategy(super::PbRemoveStrategyLayout),
        #[prost(message, tag = "6")]
        RemoveStrategy2(super::PbRemoveStrategy2Layout),
        #[prost(message, tag = "7")]
        CollectDust(super::PbCollectDustLayout),
        #[prost(message, tag = "8")]
        AddStrategy(super::PbAddStrategyLayout),
        #[prost(message, tag = "9")]
        DepositStrategy(super::PbDepositStrategyLayout),
        #[prost(message, tag = "10")]
        WithdrawStrategy(super::PbWithdrawStrategyLayout),
        #[prost(message, tag = "11")]
        Withdraw2(super::PbWithdraw2Layout),
        #[prost(message, tag = "12")]
        Deposit(super::PbDepositLayout),
        #[prost(message, tag = "13")]
        Withdraw(super::PbWithdrawLayout),
        #[prost(message, tag = "14")]
        WithdrawDirectlyFromStrategy(super::PbWithdrawDirectlyFromStrategyLayout),
        #[prost(message, tag = "15")]
        ClaimRewards(super::PbClaimRewardsLayout),
        #[prost(message, tag = "16")]
        PatchVaultFieldInStrategy(super::PbPatchVaultFieldInStrategyLayout),
        #[prost(message, tag = "17")]
        WithdrawMango(super::PbWithdrawMangoLayout),
        #[prost(message, tag = "18")]
        IdlWrite(super::PbIdlWriteLayout),
    }
}
/// Wrapper message for map output
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInstructions {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Meta>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PbStrategyType {
    PortFinanceWithoutLm = 0,
    PortFinanceWithLm = 1,
    SolendWithoutLm = 2,
    Mango = 3,
    SolendWithLm = 4,
    ApricotWithoutLm = 5,
    Francium = 6,
    Tulip = 7,
    Vault = 8,
    Drift = 9,
    Frakt = 10,
    Marginfi = 11,
}
impl PbStrategyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::PortFinanceWithoutLm => "PORT_FINANCE_WITHOUT_LM",
            Self::PortFinanceWithLm => "PORT_FINANCE_WITH_LM",
            Self::SolendWithoutLm => "SOLEND_WITHOUT_LM",
            Self::Mango => "MANGO",
            Self::SolendWithLm => "SOLEND_WITH_LM",
            Self::ApricotWithoutLm => "APRICOT_WITHOUT_LM",
            Self::Francium => "FRANCIUM",
            Self::Tulip => "TULIP",
            Self::Vault => "VAULT",
            Self::Drift => "DRIFT",
            Self::Frakt => "FRAKT",
            Self::Marginfi => "MARGINFI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PORT_FINANCE_WITHOUT_LM" => Some(Self::PortFinanceWithoutLm),
            "PORT_FINANCE_WITH_LM" => Some(Self::PortFinanceWithLm),
            "SOLEND_WITHOUT_LM" => Some(Self::SolendWithoutLm),
            "MANGO" => Some(Self::Mango),
            "SOLEND_WITH_LM" => Some(Self::SolendWithLm),
            "APRICOT_WITHOUT_LM" => Some(Self::ApricotWithoutLm),
            "FRANCIUM" => Some(Self::Francium),
            "TULIP" => Some(Self::Tulip),
            "VAULT" => Some(Self::Vault),
            "DRIFT" => Some(Self::Drift),
            "FRAKT" => Some(Self::Frakt),
            "MARGINFI" => Some(Self::Marginfi),
            _ => None,
        }
    }
}
