use serde::Deserialize;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Deserialize)]
pub struct TransactionStat {
    pub block_slot: u32,
    pub block_date: Option<String>,
    pub block_time: u64,
    pub index: Option<u32>,
    pub required_signatures: u32,
    pub readonly_signed_accounts: u32,
    pub readonly_unsigned_accounts: u32,
    pub id: String,
    pub fees: u64,
    pub account_keys: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    pub post_token_balances: Option<Vec<TokenBalance>>,
    pub signatures: Vec<String>,
    pub signer: String,
    pub version: String,
    pub executing_accounts: Vec<String>,
    pub logs_truncated: bool,
    pub log_messages: Vec<String>,
    pub compute_units_consumed: Option<u64>,
    pub return_data: Option<ReturnData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Deserialize)]
pub struct Instruction {
    pub executing_account: String,
    pub account_arguments: Vec<String>,
    pub data: String,
    pub inner_instructions: Option<Vec<InnerInstruction>>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Deserialize)]
pub struct InnerInstruction {
    pub executing_account: String,
    pub account_arguments: Vec<String>,
    pub data: String,
    pub stack_height: Option<u32>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Deserialize)]
pub struct TokenBalance {
    pub account: String,
    pub amount: f64,
    pub mint: String,
    pub owner: String,
    pub program: String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Deserialize)]
pub struct ReturnData {
    pub program_id: String,
    pub data: String,
}
