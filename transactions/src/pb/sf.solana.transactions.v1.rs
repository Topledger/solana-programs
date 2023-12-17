// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TransactionStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStats {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="3")]
    pub block_time: i64,
    #[prost(uint32, required, tag="4")]
    pub index: u32,
    #[prost(uint64, required, tag="5")]
    pub fee: u64,
    #[prost(message, optional, tag="6")]
    pub error: ::core::option::Option<Error>,
    #[prost(uint32, required, tag="7")]
    pub required_signatures: u32,
    #[prost(uint32, required, tag="8")]
    pub readonly_signed_accounts: u32,
    #[prost(uint32, required, tag="9")]
    pub readonly_unsigned_accounts: u32,
    #[prost(string, required, tag="10")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, required, tag="11")]
    pub success: bool,
    #[prost(message, repeated, tag="14")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(string, repeated, tag="15")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="16")]
    pub log_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, packed="false", tag="18")]
    pub pre_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, packed="false", tag="19")]
    pub post_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="20")]
    pub pre_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(message, repeated, tag="21")]
    pub post_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(string, repeated, tag="22")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="23")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, required, tag="24")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="25")]
    pub executing_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, required, tag="26")]
    pub base_fee: u32,
    #[prost(uint32, required, tag="27")]
    pub priority_fee: u32,
    #[prost(uint64, required, tag="28")]
    pub compute_units_consumed: u64,
    #[prost(uint64, required, tag="29")]
    pub compute_units_allocated: u64,
    #[prost(uint32, required, tag="30")]
    pub byte_size: u32,
    #[prost(uint32, required, tag="31")]
    pub trx_accounts_size: u32,
    #[prost(uint32, required, tag="32")]
    pub writeable_alt_accounts_size: u32,
    #[prost(uint32, required, tag="33")]
    pub readable_alt_accounts_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, required, tag="1")]
    pub program: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, required, tag="1")]
    pub executing_account: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub account_arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="3")]
    pub data: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub inner_instructions: ::prost::alloc::vec::Vec<InnerInstruction>,
    #[prost(uint32, required, tag="5")]
    pub bytes: u32,
    #[prost(uint32, required, tag="6")]
    pub account_bytes: u32,
    #[prost(uint32, required, tag="7")]
    pub data_bytes: u32,
    #[prost(string, repeated, tag="8")]
    pub program_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstruction {
    #[prost(string, required, tag="1")]
    pub executing_account: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub account_arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub program_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalance {
    #[prost(string, required, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub amount: i64,
    #[prost(string, required, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub program_id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
