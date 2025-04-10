#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use pb::sf::solana::transactions::v1::InnerInstruction;
use pb::sf::solana::transactions::v1::TokenBalance;
use pb::sf::solana::transactions::v1::TransactionStat;
use serde::Serialize;
use substreams_solana::pb::sf::solana::r#type::v1::InnerInstructions;
use utils::get_amt;
use utils::get_mint;
mod trade_instruction;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Default)]
pub struct TradeData {
    pub block_slot: u64,
    pub block_date: String,
    pub block_time: i64,
    pub tx_id: String,
    pub signer: String,
    pub pool_address: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub base_amount: f64,
    pub quote_amount: f64,
    pub is_inner_instruction: bool,
    pub instruction_index: u32,
    pub instruction_type: String,
    pub inner_instruction_index: u32,
    pub outer_program: String,
    pub inner_program: String,
    pub txn_fee_lamports: u64,
    pub signer_lamports_change: i64,
    pub error: String,
}

#[wasm_bindgen]
pub fn parse(transaction: &JsValue) -> JsValue {
    let data = parse_transaction(transaction);
    serde_wasm_bindgen::to_value(&data).unwrap()
}

fn parse_transaction(transaction_js: &JsValue) -> Vec<TradeData> {
    let mut result: Vec<TradeData> = vec![];

    let transaction_stat: TransactionStat = match transaction_js.into_serde() {
        Ok(tx) => tx,
        Err(err) => {
            let mut err_trade_data = TradeData::default();
            err_trade_data.error = format!("Error deserializing: {err}");
            result.push(err_trade_data);
            return result;
        }
    };

    let accounts = transaction_stat.account_keys.clone();
    let pre_balances = transaction_stat.pre_balances.clone();
    let post_balances = transaction_stat.post_balances.clone();
    let pre_token_balances = transaction_stat
        .pre_token_balances
        .clone()
        .unwrap_or_default();
    let post_token_balances = transaction_stat
        .post_token_balances
        .clone()
        .unwrap_or_default();

    for (idx, inst) in transaction_stat
        .instructions
        .clone()
        .into_iter()
        .enumerate()
    {
        let inner_instructions = inst.inner_instructions.unwrap_or_default();
        let program = inst.executing_account;

        let trade_instruction = get_trade_instruction(
            &program,
            bs58::decode(inst.data.clone()).into_vec().unwrap(),
            inst.account_arguments,
            accounts.clone(),
            &pre_token_balances,
            &post_token_balances,
            &"".to_string(),
            false,
            &inner_instructions,
            0 as u32,
        );

        if trade_instruction.is_some() {
            let ti = trade_instruction.unwrap();
            let mut trade_data = default_trade_data(&transaction_stat);
            trade_data.pool_address = ti.amm;
            trade_data.base_mint = get_mint(
                &ti.vault_a,
                &post_token_balances,
                &accounts,
                ti.dapp_address.clone(),
            );
            trade_data.quote_mint =
                get_mint(&ti.vault_b, &post_token_balances, &accounts, "".to_string());
            trade_data.base_vault = ti.vault_a.clone();
            trade_data.quote_vault = ti.vault_b.clone();
            trade_data.base_amount = get_amt(
                &ti.vault_a.clone(),
                0 as u32,
                &inner_instructions,
                &accounts,
                &post_token_balances,
                ti.dapp_address.clone(),
                pre_balances.clone(),
                post_balances.clone(),
            );

            trade_data.quote_amount = get_amt(
                &ti.vault_b.clone(),
                0 as u32,
                &inner_instructions,
                &accounts,
                &post_token_balances,
                "".to_string(),
                pre_balances.clone(),
                post_balances.clone(),
            );
            trade_data.is_inner_instruction = false;
            trade_data.instruction_index = idx as u32;
            trade_data.instruction_type = ti.name.clone();
            trade_data.inner_instruction_index = 0;
            trade_data.outer_program = ti.dapp_address.clone();
            trade_data.inner_program = "".to_string();
            result.push(trade_data);

            if ti.second_swap_amm.clone().unwrap_or_default() != "" {
                let mut trade_data = default_trade_data(&transaction_stat);
                trade_data.pool_address = ti.second_swap_amm.unwrap();
                trade_data.base_mint = get_mint(
                    &ti.second_swap_vault_a.clone().unwrap(),
                    &post_token_balances,
                    &accounts,
                    "".to_string(),
                );
                trade_data.quote_mint = get_mint(
                    &ti.second_swap_vault_b.clone().unwrap(),
                    &post_token_balances,
                    &accounts,
                    "".to_string(),
                );
                trade_data.base_vault = ti.second_swap_vault_a.clone().unwrap();
                trade_data.quote_vault = ti.second_swap_vault_b.clone().unwrap();
                trade_data.base_amount = get_amt(
                    &ti.second_swap_vault_a.clone().unwrap(),
                    0 as u32,
                    &inner_instructions,
                    &accounts,
                    &post_token_balances,
                    "".to_string(),
                    pre_balances.clone(),
                    post_balances.clone(),
                );
                trade_data.quote_amount = get_amt(
                    &ti.second_swap_vault_b.clone().unwrap(),
                    0 as u32,
                    &inner_instructions,
                    &accounts,
                    &post_token_balances,
                    "".to_string(),
                    pre_balances.clone(),
                    post_balances.clone(),
                );
                trade_data.is_inner_instruction = false;
                trade_data.instruction_index = idx as u32;
                trade_data.instruction_type = ti.name.clone();
                trade_data.inner_instruction_index = 0;
                trade_data.outer_program = ti.dapp_address.clone();
                trade_data.inner_program = "".to_string();
                result.push(trade_data);
            }
        }

        inner_instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = inner_inst.executing_account.clone();
                let inner_trade_instruction = get_trade_instruction(
                    &inner_program,
                    bs58::decode(inner_inst.data.clone()).into_vec().unwrap(),
                    inner_inst.account_arguments.clone(),
                    accounts.clone(),
                    &pre_token_balances,
                    &post_token_balances,
                    &program,
                    true,
                    &inner_instructions,
                    inner_idx as u32,
                );

                if inner_trade_instruction.is_some() {
                    let inner_ti = inner_trade_instruction.unwrap();
                    let mut inner_trade_data = default_trade_data(&transaction_stat);
                    inner_trade_data.pool_address = inner_ti.amm;
                    inner_trade_data.base_mint = get_mint(
                        &inner_ti.vault_a,
                        &post_token_balances,
                        &accounts,
                        inner_ti.dapp_address.clone(),
                    );
                    inner_trade_data.quote_mint = get_mint(
                        &inner_ti.vault_b,
                        &post_token_balances,
                        &accounts,
                        "".to_string(),
                    );
                    inner_trade_data.base_vault = inner_ti.vault_a.clone();
                    inner_trade_data.quote_vault = inner_ti.vault_b.clone();
                    inner_trade_data.base_amount = get_amt(
                        &inner_ti.vault_a.clone(),
                        0 as u32,
                        &inner_instructions,
                        &accounts,
                        &post_token_balances,
                        inner_ti.dapp_address.clone(),
                        pre_balances.clone(),
                        post_balances.clone(),
                    );
                    inner_trade_data.quote_amount = get_amt(
                        &inner_ti.vault_b.clone(),
                        0 as u32,
                        &inner_instructions,
                        &accounts,
                        &post_token_balances,
                        "".to_string(),
                        pre_balances.clone(),
                        post_balances.clone(),
                    );
                    inner_trade_data.is_inner_instruction = true;
                    inner_trade_data.instruction_index = idx as u32;
                    inner_trade_data.instruction_type = inner_ti.name.clone();
                    inner_trade_data.inner_instruction_index = inner_idx as u32;
                    inner_trade_data.outer_program = program.to_string();
                    inner_trade_data.inner_program = inner_ti.dapp_address.clone();
                    result.push(inner_trade_data);

                    if inner_ti.second_swap_amm.clone().unwrap_or_default() != "" {
                        let mut inner_trade_data = default_trade_data(&transaction_stat);
                        inner_trade_data.pool_address = inner_ti.second_swap_amm.unwrap();
                        inner_trade_data.base_mint = get_mint(
                            &inner_ti.second_swap_vault_a.clone().unwrap(),
                            &post_token_balances,
                            &accounts,
                            "".to_string(),
                        );
                        inner_trade_data.quote_mint = get_mint(
                            &inner_ti.second_swap_vault_b.clone().unwrap(),
                            &post_token_balances,
                            &accounts,
                            "".to_string(),
                        );
                        inner_trade_data.base_vault = inner_ti.second_swap_vault_a.clone().unwrap();
                        inner_trade_data.quote_vault =
                            inner_ti.second_swap_vault_b.clone().unwrap();
                        inner_trade_data.base_amount = get_amt(
                            &inner_ti.second_swap_vault_a.clone().unwrap(),
                            inner_idx as u32,
                            &inner_instructions,
                            &accounts,
                            &post_token_balances,
                            "".to_string(),
                            pre_balances.clone(),
                            post_balances.clone(),
                        );
                        inner_trade_data.quote_amount = get_amt(
                            &inner_ti.second_swap_vault_b.clone().unwrap(),
                            inner_idx as u32,
                            &inner_instructions,
                            &accounts,
                            &post_token_balances,
                            "".to_string(),
                            pre_balances.clone(),
                            post_balances.clone(),
                        );
                        inner_trade_data.is_inner_instruction = true;
                        inner_trade_data.instruction_index = idx as u32;
                        inner_trade_data.instruction_type = inner_ti.name.clone();
                        inner_trade_data.inner_instruction_index = inner_idx as u32;
                        inner_trade_data.outer_program = inner_ti.dapp_address.clone();
                        inner_trade_data.inner_program = inner_ti.dapp_address.clone();
                        result.push(inner_trade_data);
                    }
                }
            });
    }
    result
}

fn get_trade_instruction(
    dapp_address: &String,
    instruction_data: Vec<u8>,
    input_accounts: Vec<String>,
    all_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    outer_program: &String,
    is_inner: bool,
    inner_instructions: &Vec<InnerInstruction>,
    input_inner_idx: u32,
) -> Option<trade_instruction::TradeInstruction> {
    let mut result = None;
    match dapp_address.as_str() {
        "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR" => {
            result =
                dapps::dapp_CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => {
            result =
                dapps::dapp_Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB" => {
            result =
                dapps::dapp_Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => {
            result =
                dapps::dapp_PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr" => {
            result =
                dapps::dapp_SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX" => {
            let jupiter_dapps = vec![
                "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
                "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
                "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
                "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
                "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
                "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
                "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
            ];

            if is_inner & jupiter_dapps.contains(outer_program) {
                result =
                dapps::dapp_srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
            }
        }
        "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt" => {
            result =
                dapps::dapp_HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            result =
                dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S" => {
            result =
                dapps::dapp_EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c" => {
            result =
                dapps::dapp_2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => {
            result =
                dapps::dapp_SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
            result =
                dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
            result =
                dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6" => {
            result =
                dapps::dapp_AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4" => {
            result =
                dapps::dapp_CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8" => {
            result =
                dapps::dapp_cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5" => {
            result =
                dapps::dapp_7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" => {
            let jupiter_dapps = vec![
                "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
                "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
                "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
                "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
                "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
                "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
                "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
            ];

            if is_inner & jupiter_dapps.contains(outer_program) {
                result =
                dapps::dapp_9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
            }
        }
        "GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn" => {
            result =
                dapps::dapp_GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    inner_instructions,
                    &all_accounts,
                );
        }
        "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => {
            result =
                dapps::dapp_SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ" => {
            result =
                dapps::dapp_SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph" => {
            result =
                dapps::dapp_dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh" => {
            result =
                dapps::dapp_CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP" => {
            result =
                dapps::dapp_PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag" => {
            result =
                dapps::dapp_D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr" => {
            result =
                dapps::dapp_2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result =
                dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    &post_token_balances,
                    &all_accounts,
                );
        }
        "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => {
            result =
                dapps::dapp_27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    &post_token_balances,
                    &all_accounts,
                );
        }
        "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p" => {
            result =
                dapps::dapp_BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X" => {
            result =
                dapps::dapp_FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H" => {
            result =
                dapps::dapp_9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => {
            result =
                dapps::dapp_DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319" => {
            result =
                dapps::dapp_6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo" => {
            result =
                dapps::dapp_LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => {
            result =
                dapps::dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb" => {
            let jupiter_dapps = vec![
                "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
                "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
                "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
                "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
                "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
                "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
                "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
            ];

            if is_inner & jupiter_dapps.contains(outer_program) {
                result =
                dapps::dapp_opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
            }
        }
        "DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm" => {
            result =
                dapps::dapp_DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt" => {
            result =
                dapps::dapp_H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y" => {
            result =
                dapps::dapp_obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe" => {
            result =
                dapps::dapp_SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ" => {
            result =
                dapps::dapp_swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => {
            result =
                dapps::dapp_6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        _ => {}
    }

    return result;
}

fn get_signer_balance_change(pre_balances: &Vec<u64>, post_balances: &Vec<u64>) -> i64 {
    if let (Some(&pre_signer), Some(&post_signer)) = (pre_balances.get(0), post_balances.get(0)) {
        (post_signer as i64) - (pre_signer as i64)
    } else {
        0
    }
}

fn default_trade_data(transaction_stat: &TransactionStat) -> TradeData {
    let mut trade_data = TradeData::default();
    let _transaction_stat = transaction_stat.clone();
    trade_data.tx_id = _transaction_stat.id.clone();
    trade_data.block_slot = _transaction_stat.block_slot as u64;
    trade_data.block_date = _transaction_stat.block_date.clone().unwrap_or_default();
    trade_data.block_time = _transaction_stat.block_time as i64;
    trade_data.signer = _transaction_stat.signer.clone();
    trade_data.txn_fee_lamports = _transaction_stat.fees;
    trade_data.signer_lamports_change = get_signer_balance_change(
        &_transaction_stat.pre_balances,
        &_transaction_stat.post_balances,
    );
    trade_data.error = "".to_string();
    trade_data
}
