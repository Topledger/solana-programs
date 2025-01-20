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
use utils::{convert_to_date, get_amt, get_outer_executing_accounts};
use utils::{get_mint, get_trader_account, get_trader_account_v2};
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

                        if (trader_token_balance_changes.len() == 1) {
                            let trader_token_balance_change =
                                trader_token_balance_changes.get(0).unwrap();
                            let amt = trader_token_balance_change.amount;
                            if (amt >= 0.0) {
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

                        if (trader_token_balance_changes.len() == 2) {
                            for trader_token_balance_change in trader_token_balance_changes.iter() {
                                if (trader_token_balance_change.amount >= 0.0) {
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
                            tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                            trader: trader.clone(),
                            trader_sol_change: get_trader_balance_change(
                                &trader,
                                &pre_balances,
                                &post_balances,
                                &accounts,
                            ),
                            txn_fee_lamports: 1.0 as u64,
                            buy_mint: buy_mint,
                            sell_mint: sell_mint,
                            buy_amount: buy_amount,
                            sell_amount: sell_amount,
                        });
                    }

                    // meta.inner_instructions
                    //     .iter()
                    //     .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    //     .for_each(|inner_instruction| {
                    //         inner_instruction.instructions.iter().enumerate().for_each(
                    //             |(inner_idx, inner_inst)| {
                    //                 let inner_program =
                    //                     &accounts[inner_inst.program_id_index as usize];
                    //                 let inner_trade_data = get_trade_instruction(
                    //                     inner_program,
                    //                     inner_inst.data.clone(),
                    //                     &inner_inst.accounts,
                    //                     &accounts,
                    //                     &pre_token_balances,
                    //                     &post_token_balances,
                    //                     &program.to_string(),
                    //                     true,
                    //                     &inner_instructions,
                    //                     inner_idx as u32,
                    //                 );

                    //                 if inner_trade_data.is_some() {
                    //                     let inner_td = inner_trade_data.unwrap();

                    //                     let inner_td_name = inner_td.name;
                    //                     let inner_td_dapp_address = inner_td.dapp_address;
                    //                     let trader = get_trader_account(
                    //                         &inner_td.vault_a,
                    //                         &inner_td.vault_b,
                    //                         inner_idx as u32,
                    //                         &inner_instructions,
                    //                         &accounts,
                    //                         &post_token_balances,
                    //                         "".to_string(),
                    //                         pre_balances.clone(),
                    //                         post_balances.clone(),
                    //                     );

                    //                     data.push(TradeData {
                    //                         block_date: convert_to_date(timestamp),
                    //                         tx_id: bs58::encode(&transaction.signatures[0])
                    //                             .into_string(),
                    //                         tx_index: tx_index as i64,
                    //                         block_slot: slot,
                    //                         block_time: timestamp,
                    //                         signer: accounts.get(0).unwrap().to_string(),
                    //                         pool_address: inner_td.amm,
                    //                         base_mint: get_mint(
                    //                             &inner_td.vault_a,
                    //                             &post_token_balances,
                    //                             &accounts,
                    //                             inner_td_dapp_address.clone(),
                    //                         ),
                    //                         quote_mint: get_mint(
                    //                             &inner_td.vault_b,
                    //                             &post_token_balances,
                    //                             &accounts,
                    //                             "".to_string(),
                    //                         ),
                    //                         base_amount: get_amt(
                    //                             &inner_td.vault_a,
                    //                             inner_idx as u32,
                    //                             &inner_instructions,
                    //                             &accounts,
                    //                             &post_token_balances,
                    //                             inner_td_dapp_address.clone(),
                    //                             pre_balances.clone(),
                    //                             post_balances.clone(),
                    //                         ),
                    //                         quote_amount: get_amt(
                    //                             &inner_td.vault_b,
                    //                             inner_idx as u32,
                    //                             &inner_instructions,
                    //                             &accounts,
                    //                             &post_token_balances,
                    //                             "".to_string(),
                    //                             pre_balances.clone(),
                    //                             post_balances.clone(),
                    //                         ),
                    //                         trader: trader.clone(),
                    //                         base_vault: inner_td.vault_a,
                    //                         quote_vault: inner_td.vault_b,
                    //                         is_inner_instruction: true,
                    //                         instruction_index: idx as u32,
                    //                         instruction_type: inner_td_name.clone(),
                    //                         inner_instruxtion_index: inner_idx as u32,
                    //                         outer_program: program.to_string(),
                    //                         inner_program: inner_td_dapp_address.clone(),
                    //                         txn_fee_lamports: meta.fee,
                    //                         signer_lamports_change: get_signer_balance_change(
                    //                             &pre_balances,
                    //                             &post_balances,
                    //                         ),
                    //                         outer_executing_accounts: get_outer_executing_accounts(
                    //                             &msg.instructions,
                    //                             &accounts,
                    //                         ),
                    //                         trader_lamports_change: get_trader_balance_change(
                    //                             accounts.get(0).unwrap().to_string(),
                    //                             trader.clone(),
                    //                             &pre_balances,
                    //                             &post_balances,
                    //                             &accounts,
                    //                         ),
                    //                         trader_token_balance_changes:
                    //                             get_trader_token_balance_changes(
                    //                                 &accounts.get(0).unwrap().to_string(),
                    //                                 &trader,
                    //                                 &pre_token_balances,
                    //                                 &post_token_balances,
                    //                             ),
                    //                     });
                    //                 }
                    //             },
                    //         )
                    //     });
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
        // "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => {
        //     result =
        //         dapps::dapp_Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB" => {
        //     result =
        //         dapps::dapp_Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => {
        //     result =
        //         dapps::dapp_PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr" => {
        //     result =
        //         dapps::dapp_SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX" => {
        //     let jupiter_dapps = vec![
        //         "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
        //         "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
        //         "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
        //         "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
        //         "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
        //         "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
        //         "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
        //     ];

        //     if is_inner & jupiter_dapps.contains(outer_program) {
        //         result =
        //         dapps::dapp_srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        //     }
        // }
        // "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt" => {
        //     result =
        //         dapps::dapp_HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
        //     result =
        //         dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S" => {
        //     result =
        //         dapps::dapp_EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c" => {
        //     result =
        //         dapps::dapp_2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => {
        //     result =
        //         dapps::dapp_SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
        //     result =
        //         dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
        //     result =
        //         dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6" => {
        //     result =
        //         dapps::dapp_AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4" => {
        //     result =
        //         dapps::dapp_CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8" => {
        //     result =
        //         dapps::dapp_cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5" => {
        //     result =
        //         dapps::dapp_7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" => {
        //     let jupiter_dapps = vec![
        //         "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
        //         "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
        //         "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
        //         "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
        //         "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
        //         "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
        //         "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
        //     ];

        //     if is_inner & jupiter_dapps.contains(outer_program) {
        //         result =
        //         dapps::dapp_9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        //     }
        // }
        // "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => {
        //     result =
        //         dapps::dapp_SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ" => {
        //     result =
        //         dapps::dapp_SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph" => {
        //     result =
        //         dapps::dapp_dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh" => {
        //     result =
        //         dapps::dapp_CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP" => {
        //     result =
        //         dapps::dapp_PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag" => {
        //     result =
        //         dapps::dapp_D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr" => {
        //     result =
        //         dapps::dapp_2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result = dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::is_trade_instruction(
                instruction_data,
            );
        }
        // "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => {
        //     result =
        //         dapps::dapp_27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //             &post_token_balances,
        //             accounts,
        //         );
        // }
        // "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p" => {
        //     result =
        //         dapps::dapp_BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X" => {
        //     result =
        //         dapps::dapp_FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H" => {
        //     result =
        //         dapps::dapp_9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => {
        //     result =
        //         dapps::dapp_DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319" => {
        //     result =
        //         dapps::dapp_6MLxLqiXaaSUpkgMnWDTuejNZEz3kE7k2woyHGVFw319::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo" => {
        //     result =
        //         dapps::dapp_LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => {
        //     result =
        //         dapps::dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb" => {
        //     let jupiter_dapps = vec![
        //         "JUP2jxvXaqu7NQY1GmNF4m1vodw12LVXYxbFL2uJvfo".to_string(),
        //         "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph".to_string(),
        //         "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB".to_string(),
        //         "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow".to_string(),
        //         "JUP5pEAZeHdHrLxh5UCwAbpjGwYKKoquCpda2hfP4u8".to_string(),
        //         "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(),
        //         "JUP6i4ozu5ydDCnLiMogSckDPpbtr7BJ4FtzYWkb5Rk".to_string(),
        //     ];

        //     if is_inner & jupiter_dapps.contains(outer_program) {
        //         result =
        //         dapps::dapp_opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        //     }
        // }
        // "DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm" => {
        //     result =
        //         dapps::dapp_DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt" => {
        //     result =
        //         dapps::dapp_H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y" => {
        //     result =
        //         dapps::dapp_obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe" => {
        //     result =
        //         dapps::dapp_SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ" => {
        //     result =
        //         dapps::dapp_swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
        // "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => {
        //     result =
        //         dapps::dapp_6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P::parse_trade_instruction(
        //             instruction_data,
        //             input_accounts,
        //         );
        // }
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
    let mut result: f64 = 0.0;
    let unwapped_index = accounts.iter().position(|r| r == address);
    if unwapped_index.is_some() {
        let index = unwapped_index.unwrap();
        result = (post_balances[index] - pre_balances[index]) as f64 / 1e9;
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
        result.push(TraderTokenBalanceChange {
            mint: key.to_string(),
            amount: value[0] - value[1],
        });
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
