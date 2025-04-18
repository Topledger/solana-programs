#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use std::collections::HashMap;

use pb::sf::solana::wallet::positions::dex::trades::v1::{
    Output, TraderTokenBalanceChange, WalletPositionDexTradeData,
};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::InnerInstructions;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, TokenBalance};
use utils::convert_to_date;
use utils::get_trader_account_v2;
mod trade_instruction;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    process_block(block)
}

fn process_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref();
    let mut data: Vec<WalletPositionDexTradeData> = vec![];
    if timestamp.is_some() {
        let timestamp = timestamp.unwrap().timestamp;
        // (index, trx) in block.transactions.iter().enumerate()
        for (tx_index, trx) in block.transactions_owned().enumerate() {
            let accounts = trx.resolved_accounts_as_strings();
            if let Some(transaction) = trx.transaction {
                let meta = trx.meta.unwrap();
                let pre_balances = meta.pre_balances;
                let post_balances = meta.post_balances;
                let pre_token_balances = meta.pre_token_balances;
                let post_token_balances = meta.post_token_balances;

                let msg = transaction.message.unwrap();

                for (idx, inst) in msg.instructions.clone().into_iter().enumerate() {
                    let inner_instructions: Vec<InnerInstructions> =
                        filter_inner_instructions(&meta.inner_instructions, idx as u32);

                    let program = &accounts[inst.program_id_index as usize];
                    let is_trade_ix = is_trade_instruction(program, inst.data);
                    if is_trade_ix {
                        let trader = get_trader_account_v2(
                            msg.header.clone().unwrap().num_required_signatures,
                            &pre_token_balances,
                            &post_token_balances,
                            &accounts,
                        );

                        let trader_token_balance_changes: Vec<TraderTokenBalanceChange> =
                            get_trader_token_balance_changes(
                                &trader,
                                &pre_token_balances,
                                &post_token_balances,
                            );

                        let mut buy_mint = "".to_string();
                        let mut sell_mint = "".to_string();
                        let mut buy_amount = 0.0;
                        let mut sell_amount = 0.0;

                        if trader_token_balance_changes.len() == 1 {
                            let trader_token_balance_change =
                                trader_token_balance_changes.get(0).unwrap();
                            let amt = trader_token_balance_change.amount;
                            if amt >= 0.0 {
                                buy_mint = trader_token_balance_change.mint.clone();
                                buy_amount = trader_token_balance_change.amount;
                                sell_mint =
                                    "So11111111111111111111111111111111111111112".to_string();
                                sell_amount = get_trader_balance_change(
                                    &trader,
                                    &pre_balances,
                                    &post_balances,
                                    &accounts,
                                );
                            } else {
                                sell_mint = trader_token_balance_change.mint.clone();
                                sell_amount = trader_token_balance_change.amount;
                                buy_mint =
                                    "So11111111111111111111111111111111111111112".to_string();
                                buy_amount = get_trader_balance_change(
                                    &trader,
                                    &pre_balances,
                                    &post_balances,
                                    &accounts,
                                );
                            }
                        }

                        if trader_token_balance_changes.len() == 2 {
                            for trader_token_balance_change in trader_token_balance_changes.iter() {
                                if trader_token_balance_change.amount >= 0.0 {
                                    buy_mint = trader_token_balance_change.mint.clone();
                                    buy_amount = trader_token_balance_change.amount;
                                } else {
                                    sell_mint = trader_token_balance_change.mint.clone();
                                    sell_amount = trader_token_balance_change.amount;
                                }
                            }
                        }

                        data.push(WalletPositionDexTradeData {
                            block_date: convert_to_date(timestamp),
                            block_time: timestamp,
                            block_slot: slot,
                            tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                            signer: accounts.get(0).unwrap().to_string(),
                            trader: trader.clone(),
                            trader_sol_change: get_trader_balance_change(
                                &trader,
                                &pre_balances,
                                &post_balances,
                                &accounts,
                            ),
                            txn_fee_lamports: meta.fee,
                            buy_mint: buy_mint.clone(),
                            sell_mint: sell_mint.clone(),
                            buy_amount: buy_amount,
                            sell_amount: sell_amount,
                            buy_mint_pre_token_balance: get_token_balance(
                                buy_mint.clone(),
                                trader.clone(),
                                &pre_token_balances,
                            ),
                            buy_mint_post_token_balance: get_token_balance(
                                buy_mint.clone(),
                                trader.clone(),
                                &post_token_balances,
                            ),
                            sell_mint_pre_token_balance: get_token_balance(
                                sell_mint.clone(),
                                trader.clone(),
                                &pre_token_balances,
                            ),
                            sell_mint_post_token_balance: get_token_balance(
                                sell_mint.clone(),
                                trader.clone(),
                                &post_token_balances,
                            ),
                            sol_pre_balance: get_sol_balance(
                                trader.clone(),
                                &pre_balances,
                                &accounts,
                            ),
                            sol_post_balance: get_sol_balance(
                                trader.clone(),
                                &post_balances,
                                &accounts,
                            ),
                        });
                    }

                    meta.inner_instructions
                        .iter()
                        .filter(|inner_instruction| inner_instruction.index == idx as u32)
                        .for_each(|inner_instruction| {
                            inner_instruction.instructions.iter().enumerate().for_each(
                                |(inner_idx, inner_inst)| {
                                    let inner_program =
                                        &accounts[inner_inst.program_id_index as usize];

                                    let is_trade_ix = is_trade_instruction(
                                        inner_program,
                                        inner_inst.data.clone(),
                                    );
                                    if is_trade_ix {
                                        let trader = get_trader_account_v2(
                                            msg.header.clone().unwrap().num_required_signatures,
                                            &pre_token_balances,
                                            &post_token_balances,
                                            &accounts,
                                        );

                                        let trader_token_balance_changes: Vec<
                                            TraderTokenBalanceChange,
                                        > = get_trader_token_balance_changes(
                                            &trader,
                                            &pre_token_balances,
                                            &post_token_balances,
                                        );

                                        let mut buy_mint = "".to_string();
                                        let mut sell_mint = "".to_string();
                                        let mut buy_amount = 0.0;
                                        let mut sell_amount = 0.0;

                                        if trader_token_balance_changes.len() == 1 {
                                            let trader_token_balance_change =
                                                trader_token_balance_changes.get(0).unwrap();
                                            let amt = trader_token_balance_change.amount;
                                            if amt >= 0.0 {
                                                buy_mint = trader_token_balance_change.mint.clone();
                                                buy_amount = trader_token_balance_change.amount;
                                                sell_mint =
                                                    "So11111111111111111111111111111111111111112"
                                                        .to_string();
                                                sell_amount = get_trader_balance_change(
                                                    &trader,
                                                    &pre_balances,
                                                    &post_balances,
                                                    &accounts,
                                                );
                                            } else {
                                                sell_mint =
                                                    trader_token_balance_change.mint.clone();
                                                sell_amount = trader_token_balance_change.amount;
                                                buy_mint =
                                                    "So11111111111111111111111111111111111111112"
                                                        .to_string();
                                                buy_amount = get_trader_balance_change(
                                                    &trader,
                                                    &pre_balances,
                                                    &post_balances,
                                                    &accounts,
                                                );
                                            }
                                        }

                                        if trader_token_balance_changes.len() == 2 {
                                            for trader_token_balance_change in
                                                trader_token_balance_changes.iter()
                                            {
                                                if trader_token_balance_change.amount >= 0.0 {
                                                    buy_mint =
                                                        trader_token_balance_change.mint.clone();
                                                    buy_amount = trader_token_balance_change.amount;
                                                } else {
                                                    sell_mint =
                                                        trader_token_balance_change.mint.clone();
                                                    sell_amount =
                                                        trader_token_balance_change.amount;
                                                }
                                            }
                                        }

                                        data.push(WalletPositionDexTradeData {
                                            block_date: convert_to_date(timestamp),
                                            block_time: timestamp,
                                            block_slot: slot,
                                            tx_id: bs58::encode(&transaction.signatures[0])
                                                .into_string(),
                                            signer: accounts.get(0).unwrap().to_string(),
                                            trader: trader.clone(),
                                            trader_sol_change: get_trader_balance_change(
                                                &trader,
                                                &pre_balances,
                                                &post_balances,
                                                &accounts,
                                            ),
                                            txn_fee_lamports: meta.fee,
                                            buy_mint: buy_mint.clone(),
                                            sell_mint: sell_mint.clone(),
                                            buy_amount: buy_amount,
                                            sell_amount: sell_amount,
                                            buy_mint_pre_token_balance: get_token_balance(
                                                buy_mint.clone(),
                                                trader.clone(),
                                                &pre_token_balances,
                                            ),
                                            buy_mint_post_token_balance: get_token_balance(
                                                buy_mint.clone(),
                                                trader.clone(),
                                                &post_token_balances,
                                            ),
                                            sell_mint_pre_token_balance: get_token_balance(
                                                sell_mint.clone(),
                                                trader.clone(),
                                                &pre_token_balances,
                                            ),
                                            sell_mint_post_token_balance: get_token_balance(
                                                sell_mint.clone(),
                                                trader.clone(),
                                                &post_token_balances,
                                            ),
                                            sol_pre_balance: get_sol_balance(
                                                trader.clone(),
                                                &pre_balances,
                                                &accounts,
                                            ),
                                            sol_post_balance: get_sol_balance(
                                                trader.clone(),
                                                &post_balances,
                                                &accounts,
                                            ),
                                        });
                                    }
                                },
                            )
                        });
                }
            }
        }
    }

    log::info!("{:#?}", slot);
    Ok(Output { data })
}

fn is_trade_instruction(dapp_address: &String, instruction_data: Vec<u8>) -> bool {
    let mut result = false;
    match dapp_address.as_str() {
        "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR" => {
            result = dapps::dapp_CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR::is_trade_instruction(
                instruction_data,
            );
        }
        "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => {
            result = dapps::dapp_Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j::is_trade_instruction(
                instruction_data,
            );
        }
        "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB" => {
            result = dapps::dapp_Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB::is_trade_instruction(
                instruction_data,
            );
        }
        "2NZ9rBZtrMdJhwCDYbHjTqAjTQ4bcHxYXFAjsj6NECue" => {
            result = dapps::dapp_2NZ9rBZtrMdJhwCDYbHjTqAjTQ4bcHxYXFAjsj6NECue::is_trade_instruction(
                instruction_data,
            );
        }
        "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => {
            result = dapps::dapp_PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY::is_trade_instruction(
                instruction_data,
            );
        }
        "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr" => {
            result = dapps::dapp_SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr::is_trade_instruction(
                instruction_data,
            );
        }
        "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt" => {
            result = dapps::dapp_HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt::is_trade_instruction(
                instruction_data,
            );
        }
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            result = dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::is_trade_instruction(
                instruction_data,
            );
        }
        "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S" => {
            result = dapps::dapp_EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S::is_trade_instruction(
                instruction_data,
            );
        }
        "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c" => {
            result = dapps::dapp_2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c::is_trade_instruction(
                instruction_data,
            );
        }
        "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => {
            result = dapps::dapp_SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ::is_trade_instruction(
                instruction_data,
            );
        }
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
            result = dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::is_trade_instruction(
                instruction_data,
            );
        }
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
            result = dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::is_trade_instruction(
                instruction_data,
            );
        }
        "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6" => {
            result = dapps::dapp_AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6::is_trade_instruction(
                instruction_data,
            );
        }
        "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4" => {
            result = dapps::dapp_CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4::is_trade_instruction(
                instruction_data,
            );
        }
        "cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8" => {
            result = dapps::dapp_cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8::is_trade_instruction(
                instruction_data,
            );
        }
        "7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5" => {
            result = dapps::dapp_7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5::is_trade_instruction(
                instruction_data,
            );
        }
        "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => {
            result = dapps::dapp_SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1::is_trade_instruction(
                instruction_data,
            );
        }
        "SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ" => {
            result = dapps::dapp_SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ::is_trade_instruction(
                instruction_data,
            );
        }
        "dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph" => {
            result = dapps::dapp_dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph::is_trade_instruction(
                instruction_data,
            );
        }
        "CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh" => {
            result = dapps::dapp_CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh::is_trade_instruction(
                instruction_data,
            );
        }
        "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP" => {
            result = dapps::dapp_PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP::is_trade_instruction(
                instruction_data,
            );
        }
        "D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag" => {
            result = dapps::dapp_D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag::is_trade_instruction(
                instruction_data,
            );
        }
        "2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr" => {
            result = dapps::dapp_2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr::is_trade_instruction(
                instruction_data,
            );
        }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result = dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::is_trade_instruction(
                instruction_data,
            );
        }
        "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => {
            result = dapps::dapp_27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv::is_trade_instruction(
                instruction_data,
            );
        }
        "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p" => {
            result = dapps::dapp_BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p::is_trade_instruction(
                instruction_data,
            );
        }
        "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X" => {
            result = dapps::dapp_FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X::is_trade_instruction(
                instruction_data,
            );
        }
        "9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H" => {
            result = dapps::dapp_9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H::is_trade_instruction(
                instruction_data,
            );
        }
        "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => {
            result = dapps::dapp_DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1::is_trade_instruction(
                instruction_data,
            );
        }
        "6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319" => {
            result = dapps::dapp_6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319::is_trade_instruction(
                instruction_data,
            );
        }
        "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo" => {
            result = dapps::dapp_LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo::is_trade_instruction(
                instruction_data,
            );
        }
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => {
            result = dapps::dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C::is_trade_instruction(
                instruction_data,
            );
        }
        "DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm" => {
            result = dapps::dapp_DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm::is_trade_instruction(
                instruction_data,
            );
        }
        "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt" => {
            result = dapps::dapp_H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt::is_trade_instruction(
                instruction_data,
            );
        }
        "obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y" => {
            result = dapps::dapp_obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y::is_trade_instruction(
                instruction_data,
            );
        }
        "SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe" => {
            result = dapps::dapp_SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe::is_trade_instruction(
                instruction_data,
            );
        }
        "swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ" => {
            result = dapps::dapp_swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ::is_trade_instruction(
                instruction_data,
            );
        }
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => {
            result = dapps::dapp_6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P::is_trade_instruction(
                instruction_data,
            );
        }
        _ => {}
    }

    return result;
}

fn get_signer_balance_change(pre_balances: &Vec<u64>, post_balances: &Vec<u64>) -> i64 {
    return (post_balances[0] - pre_balances[0]) as i64;
}

fn get_trader_balance_change(
    address: &String,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    accounts: &Vec<String>,
) -> f64 {
    let result: f64;
    let unwapped_index = accounts.iter().position(|r| r == address);
    if unwapped_index.is_some() {
        let index = unwapped_index.unwrap();
        result = (post_balances[index] as f64 - pre_balances[index] as f64) / 1e9;
    } else {
        result = 0 as f64;
    }
    return result;
}

fn get_trader_token_balance_changes(
    address_to_use: &String,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
) -> Vec<TraderTokenBalanceChange> {
    let mut result: Vec<TraderTokenBalanceChange> = vec![];
    let mut mint_map: HashMap<String, [f64; 2]> = HashMap::new();

    post_token_balances
        .iter()
        .filter(|token_balance| token_balance.owner == address_to_use.to_string())
        .for_each(|token_balance| {
            mint_map.insert(
                token_balance.mint.clone(),
                [
                    token_balance.ui_token_amount.clone().unwrap().ui_amount,
                    0.0,
                ],
            );
        });
    pre_token_balances
        .iter()
        .filter(|token_balance| token_balance.owner == address_to_use.to_string())
        .for_each(|token_balance| {
            let mint = token_balance.mint.clone();
            if mint_map.get(&mint).is_some() {
                let value = mint_map.get(&mint);
                mint_map.insert(
                    mint,
                    [
                        value.unwrap()[0],
                        token_balance.ui_token_amount.clone().unwrap().ui_amount,
                    ],
                );
            } else {
                mint_map.insert(
                    mint,
                    [
                        0.0,
                        token_balance.ui_token_amount.clone().unwrap().ui_amount,
                    ],
                );
            }
        });
    mint_map.iter().for_each(|(key, value)| {
        let amount = value[0] - value[1];
        if amount != 0.0 {
            result.push(TraderTokenBalanceChange {
                mint: key.to_string(),
                amount: amount,
            });
        }
    });

    result
}

fn prepare_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}

fn filter_inner_instructions(
    meta_inner_instructions: &Vec<InnerInstructions>,
    idx: u32,
) -> Vec<InnerInstructions> {
    let mut inner_instructions: Vec<InnerInstructions> = vec![];
    let mut iterator = meta_inner_instructions.iter();
    while let Some(inner_inst) = iterator.next() {
        if inner_inst.index == idx as u32 {
            inner_instructions.push(inner_inst.clone());
        }
    }
    return inner_instructions;
}

fn get_token_balance(
    mint_address: String,
    owner_address: String,
    token_balances: &Vec<TokenBalance>,
) -> f64 {
    let mut result: f64 = 0.0;
    token_balances
        .iter()
        .filter(|token_balance| {
            token_balance.owner == owner_address.to_string()
                && token_balance.mint == mint_address.to_string()
        })
        .for_each(|token_balance| {
            result = token_balance.ui_token_amount.clone().unwrap().ui_amount;
        });
    result
}

fn get_sol_balance(address: String, balances: &Vec<u64>, accounts: &Vec<String>) -> f64 {
    let mut account_index = 0;
    accounts.iter().enumerate().for_each(|(index, account)| {
        if account.to_string().eq(&address) {
            account_index = index;
        }
    });
    *balances.get(account_index).unwrap() as f64 / 1e9
}