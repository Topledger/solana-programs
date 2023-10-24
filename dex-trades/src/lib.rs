#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use pb::sf::solana::dex::trades::v1::{Output, TradeData};
use substreams::log;
use substreams::store::{StoreGet, StoreGetArray};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, TokenBalance};
use utils::convert_to_date;
use utils::get_mint;
mod trade_instruction;

#[substreams::handlers::map]
fn map_block_before_lookup(block: Block) -> Result<Output, substreams::errors::Error> {
    process_block(block, None)
}

#[substreams::handlers::map]
fn map_block(
    block: Block,
    address_lookup_table_store: StoreGetArray<String>,
) -> Result<Output, substreams::errors::Error> {
    process_block(block, Some(address_lookup_table_store))
}

fn process_block(
    block: Block,
    address_lookup_table_store: Option<StoreGetArray<String>>,
) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<TradeData> = vec![];

    for trx in block.transactions_owned() {
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let pre_balances = meta.pre_balances;
            let post_balances = meta.post_balances;
            let pre_token_balances = meta.pre_token_balances;
            let post_token_balances = meta.post_token_balances;

            let msg = transaction.message.unwrap();
            let accounts = prepare_accounts(&msg, &address_lookup_table_store);

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];
                let trade_data = get_trade_instruction(
                    program,
                    inst.data,
                    &inst.accounts,
                    &accounts,
                    &pre_token_balances,
                    &post_token_balances,
                );
                if trade_data.is_some() {
                    let td = trade_data.unwrap();

                    data.push(TradeData {
                        block_date: convert_to_date(timestamp),
                        tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                        block_slot: slot,
                        block_time: timestamp,
                        signer: accounts.get(0).unwrap().to_string(),
                        pool_address: td.amm,
                        base_mint: get_mint(&td.vault_a, &post_token_balances, &accounts),
                        quote_mint: get_mint(&td.vault_b, &pre_token_balances, &accounts),
                        base_amount: get_amt(
                            &td.vault_a,
                            &pre_token_balances,
                            &post_token_balances,
                            &accounts,
                        ),
                        quote_amount: get_amt(
                            &td.vault_b,
                            &pre_token_balances,
                            &post_token_balances,
                            &accounts,
                        ),
                        base_vault: td.vault_a,
                        quote_vault: td.vault_b,
                        is_inner_instruction: false,
                        instruction_index: idx as u32,
                        instruction_type: td.name,
                        inner_instruxtion_index: 0,
                        outer_program: td.dapp_address,
                        inner_program: "".to_string(),
                        txn_fee: meta.fee,
                        signer_sol_change: get_signer_balance_change(&pre_balances, &post_balances),
                    });
                }

                meta.inner_instructions
                    .iter()
                    .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction.instructions.iter().enumerate().for_each(
                            |(inner_idx, inner_inst)| {
                                let inner_program = &accounts[inner_inst.program_id_index as usize];
                                let trade_data = get_trade_instruction(
                                    inner_program,
                                    inner_inst.data.clone(),
                                    &inner_inst.accounts,
                                    &accounts,
                                    &pre_token_balances,
                                    &post_token_balances,
                                );

                                if trade_data.is_some() {
                                    let td = trade_data.unwrap();

                                    data.push(TradeData {
                                        block_date: convert_to_date(timestamp),
                                        tx_id: bs58::encode(&transaction.signatures[0])
                                            .into_string(),
                                        block_slot: slot,
                                        block_time: timestamp,
                                        signer: accounts.get(0).unwrap().to_string(),
                                        pool_address: td.amm,
                                        base_mint: get_mint(
                                            &td.vault_a,
                                            &pre_token_balances,
                                            &accounts,
                                        ),
                                        quote_mint: get_mint(
                                            &td.vault_b,
                                            &pre_token_balances,
                                            &accounts,
                                        ),
                                        base_amount: get_amt(
                                            &td.vault_a,
                                            &pre_token_balances,
                                            &post_token_balances,
                                            &accounts,
                                        ),
                                        quote_amount: get_amt(
                                            &td.vault_b,
                                            &pre_token_balances,
                                            &post_token_balances,
                                            &accounts,
                                        ),
                                        base_vault: td.vault_a,
                                        quote_vault: td.vault_b,
                                        is_inner_instruction: true,
                                        instruction_index: idx as u32,
                                        instruction_type: td.name,
                                        inner_instruxtion_index: inner_idx as u32,
                                        outer_program: program.to_string(),
                                        inner_program: td.dapp_address,
                                        txn_fee: meta.fee,
                                        signer_sol_change: get_signer_balance_change(
                                            &pre_balances,
                                            &post_balances,
                                        ),
                                    });
                                }
                            },
                        )
                    });
            }
        }
    }

    log::info!("{:#?}", slot);
    return Ok(Output { data });
}

fn prepare_accounts(
    msg: &substreams_solana::pb::sf::solana::r#type::v1::Message,
    address_lookup_table_store: &Option<StoreGetArray<String>>,
) -> Vec<String> {
    let mut accounts = vec![];
    let mut writable_accounts = vec![];
    let mut readable_accounts = vec![];

    msg.clone()
        .account_keys
        .into_iter()
        .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));

    if address_lookup_table_store.is_some() {
        msg.clone()
            .address_table_lookups
            .into_iter()
            .for_each(|addr| {
                let acc = bs58::encode(&addr.account_key).into_string();
                match address_lookup_table_store
                    .as_ref()
                    .unwrap()
                    .get_last(format!("table:{}", acc))
                {
                    None => {}
                    Some(accs) => {
                        addr.writable_indexes.into_iter().for_each(|idx| {
                            writable_accounts.push(accs[idx as usize].clone());
                        });
                        addr.readonly_indexes.into_iter().for_each(|idx| {
                            readable_accounts.push(accs[idx as usize].clone());
                        })
                    }
                }
            });

        accounts.append(&mut writable_accounts);
        accounts.append(&mut readable_accounts);
    }

    return accounts;
}

fn get_trade_instruction(
    dapp_address: &String,
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
) -> Option<trade_instruction::TradeInstruction> {
    let account_args = prepare_account_args(account_indices, accounts);

    let mut result = None;
    match dapp_address.as_str() {
        "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR" => {
            result =
                dapps::dapp_CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => {
            result =
                dapps::dapp_Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB" => {
            result =
                dapps::dapp_Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => {
            result =
                dapps::dapp_PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr" => {
            result =
                dapps::dapp_SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX" => {
            result =
                dapps::dapp_srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt" => {
            result =
                dapps::dapp_HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            result =
                dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S" => {
            result =
                dapps::dapp_EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c" => {
            result =
                dapps::dapp_2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => {
            result =
                dapps::dapp_SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
            result =
                dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
            result =
                dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6" => {
            result =
                dapps::dapp_AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4" => {
            result =
                dapps::dapp_CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8" => {
            result =
                dapps::dapp_cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5" => {
            result =
                dapps::dapp_7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" => {
            result =
                dapps::dapp_9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn" => {
            result =
                dapps::dapp_GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => {
            result =
                dapps::dapp_SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ" => {
            result =
                dapps::dapp_SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph" => {
            result =
                dapps::dapp_dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "4ckmDgGdxQoPDLUkDT3vHgSAkzA3QRdNq5ywwY4sUSJn" => {
            result =
                dapps::dapp_4ckmDgGdxQoPDLUkDT3vHgSAkzA3QRdNq5ywwY4sUSJn::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg" => {
            result =
                dapps::dapp_BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o" => {
            result =
                dapps::dapp_EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh" => {
            result =
                dapps::dapp_CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP" => {
            result =
                dapps::dapp_PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag" => {
            result =
                dapps::dapp_D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr" => {
            result =
                dapps::dapp_2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result =
                dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::parse_trade_instruction(
                    instruction_data,
                    account_args,
                    &post_token_balances,
                    accounts,
                );
        }
        "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => {
            result =
                dapps::dapp_27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv::parse_trade_instruction(
                    instruction_data,
                    account_args,
                    &post_token_balances,
                    accounts,
                );
        }
        "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p" => {
            result =
                dapps::dapp_BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X" => {
            result =
                dapps::dapp_FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H" => {
            result =
                dapps::dapp_9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H::parse_trade_instruction(
                    instruction_data,
                    account_args,
                );
        }
        "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky" => {
            result =
                dapps::dapp_MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky::parse_trade_instruction(
                    instruction_data,
                    account_args,
                    &pre_token_balances,
                    &post_token_balances,
                    accounts,
                );
        }
        _ => {}
    }

    return result;
}

fn prepare_account_args(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}

fn get_amt(
    address: &String,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> f64 {
    let index = accounts.iter().position(|r| r == address).unwrap();

    let mut pre_balance: f64 = 0 as f64;
    let mut post_balance: f64 = 0 as f64;

    pre_token_balances
        .iter()
        .filter(|token_balance| token_balance.account_index == index as u32)
        .for_each(|token_balance: &TokenBalance| {
            pre_balance = token_balance.ui_token_amount.clone().unwrap().ui_amount;
        });

    post_token_balances
        .iter()
        .filter(|token_balance| token_balance.account_index == index as u32)
        .for_each(|token_balance: &TokenBalance| {
            post_balance = token_balance.ui_token_amount.clone().unwrap().ui_amount;
        });

    return (post_balance - pre_balance).abs();
}

fn get_signer_balance_change(pre_balances: &Vec<u64>, post_balances: &Vec<u64>) -> i64 {
    return (post_balances[0] - pre_balances[0]) as i64;
}
