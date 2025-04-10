// @generated
/// This Block is backwards compatible with solana.storage.ConfirmedBlock.ConfirmedBlock from
/// the Solana Labs repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(string, tag="1")]
    pub previous_blockhash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub parent_slot: u64,
    #[prost(message, repeated, tag="4")]
    pub transactions: ::prost::alloc::vec::Vec<ConfirmedTransaction>,
    #[prost(message, repeated, tag="5")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
    #[prost(message, optional, tag="6")]
    pub block_time: ::core::option::Option<UnixTimestamp>,
    #[prost(message, optional, tag="7")]
    pub block_height: ::core::option::Option<BlockHeight>,
    /// StreamingFast additions
    #[prost(uint64, tag="20")]
    pub slot: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmedTransaction {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="2")]
    pub meta: ::core::option::Option<TransactionStatusMeta>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="2")]
    pub message: ::core::option::Option<Message>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<MessageHeader>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub recent_blockhash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub instructions: ::prost::alloc::vec::Vec<CompiledInstruction>,
    #[prost(bool, tag="5")]
    pub versioned: bool,
    #[prost(message, repeated, tag="6")]
    pub address_table_lookups: ::prost::alloc::vec::Vec<MessageAddressTableLookup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHeader {
    #[prost(uint32, tag="1")]
    pub num_required_signatures: u32,
    #[prost(uint32, tag="2")]
    pub num_readonly_signed_accounts: u32,
    #[prost(uint32, tag="3")]
    pub num_readonly_unsigned_accounts: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAddressTableLookup {
    #[prost(bytes="vec", tag="1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub writable_indexes: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub readonly_indexes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStatusMeta {
    #[prost(message, optional, tag="1")]
    pub err: ::core::option::Option<TransactionError>,
    #[prost(uint64, tag="2")]
    pub fee: u64,
    #[prost(uint64, repeated, tag="3")]
    pub pre_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="4")]
    pub post_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="5")]
    pub inner_instructions: ::prost::alloc::vec::Vec<InnerInstructions>,
    #[prost(bool, tag="10")]
    pub inner_instructions_none: bool,
    #[prost(string, repeated, tag="6")]
    pub log_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="11")]
    pub log_messages_none: bool,
    #[prost(message, repeated, tag="7")]
    pub pre_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(message, repeated, tag="8")]
    pub post_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(message, repeated, tag="9")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
    #[prost(bytes="vec", repeated, tag="12")]
    pub loaded_writable_addresses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="13")]
    pub loaded_readonly_addresses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="14")]
    pub return_data: ::core::option::Option<ReturnData>,
    #[prost(bool, tag="15")]
    pub return_data_none: bool,
    /// Sum of compute units consumed by all instructions.
    /// Available since Solana v1.10.35 / v1.11.6.
    /// Set to `None` for txs executed on earlier versions.
    #[prost(uint64, optional, tag="16")]
    pub compute_units_consumed: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionError {
    #[prost(bytes="vec", tag="1")]
    pub err: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstructions {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(message, repeated, tag="2")]
    pub instructions: ::prost::alloc::vec::Vec<InnerInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstruction {
    #[prost(uint32, tag="1")]
    pub program_id_index: u32,
    #[prost(bytes="vec", tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Invocation stack height of an inner instruction.
    /// Available since Solana v1.14.6
    /// Set to `None` for txs executed on earlier versions.
    #[prost(uint32, optional, tag="4")]
    pub stack_height: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiledInstruction {
    #[prost(uint32, tag="1")]
    pub program_id_index: u32,
    #[prost(bytes="vec", tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalance {
    #[prost(uint32, tag="1")]
    pub account_index: u32,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub ui_token_amount: ::core::option::Option<UiTokenAmount>,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub program_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiTokenAmount {
    #[prost(double, tag="1")]
    pub ui_amount: f64,
    #[prost(uint32, tag="2")]
    pub decimals: u32,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ui_amount_string: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnData {
    #[prost(bytes="vec", tag="1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub lamports: i64,
    #[prost(uint64, tag="3")]
    pub post_balance: u64,
    #[prost(enumeration="RewardType", tag="4")]
    pub reward_type: i32,
    #[prost(string, tag="5")]
    pub commission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rewards {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnixTimestamp {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeight {
    #[prost(uint64, tag="1")]
    pub block_height: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RewardType {
    Unspecified = 0,
    Fee = 1,
    Rent = 2,
    Staking = 3,
    Voting = 4,
}
impl RewardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RewardType::Unspecified => "Unspecified",
            RewardType::Fee => "Fee",
            RewardType::Rent => "Rent",
            RewardType::Staking => "Staking",
            RewardType::Voting => "Voting",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Fee" => Some(Self::Fee),
            "Rent" => Some(Self::Rent),
            "Staking" => Some(Self::Staking),
            "Voting" => Some(Self::Voting),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
