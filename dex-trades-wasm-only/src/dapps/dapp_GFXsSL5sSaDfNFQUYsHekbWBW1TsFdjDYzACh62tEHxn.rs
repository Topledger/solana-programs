use crate::{
    pb::sf::solana::transactions::v1::InnerInstruction, trade_instruction::TradeInstruction,
};

const SWAP_DISCRIMINATOR: u64 = u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);

fn parse_vault_a(
    user_ata_in: String,
    input_accounts: &Vec<String>,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstruction>,
) -> String {
    let mut result: String = String::new();

    inner_instructions.iter().for_each(|inner_instruction| {
        let inner_program = inner_instruction.executing_account.clone();
        if inner_program
            .as_str()
            .eq("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        {
            let b58_data = bs58::decode(inner_instruction.data.clone())
                .into_vec()
                .unwrap();
            let (discriminator_bytes, rest) = b58_data.split_at(1);
            let discriminator: u8 = u8::from(discriminator_bytes[0]);
            match discriminator {
                3 => {
                    let _source = input_accounts.get(0).unwrap().to_string();
                    let _destination = input_accounts.get(1).unwrap().to_string();

                    if user_ata_in.as_str().eq(_source.as_str()) {
                        if result.is_empty() {
                            result = _destination;
                        }
                    }
                }
                _ => {}
            }
        }
    });
    result
}

fn parse_vault_b(
    user_ata_out: String,
    input_accounts: &Vec<String>,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstruction>,
) -> String {
    let mut result: String = String::new();

    inner_instructions.iter().for_each(|inner_instruction| {
        let inner_program = inner_instruction.executing_account.clone();
        if inner_program
            .as_str()
            .eq("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        {
            let b58_data = bs58::decode(inner_instruction.data.clone())
                .into_vec()
                .unwrap();
            let (discriminator_bytes, rest) = b58_data.split_at(1);
            let discriminator: u8 = u8::from(discriminator_bytes[0]);
            match discriminator {
                3 => {
                    let _source = input_accounts.get(0).unwrap().to_string();
                    let _destination = input_accounts.get(1).unwrap().to_string();

                    if user_ata_out.as_str().eq(_destination.as_str()) {
                        if result.is_empty() {
                            result = _source;
                        }
                    }
                }
                _ => {}
            }
        }
    });
    result
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    inner_instructions: &Vec<InnerInstruction>,
    accounts: &Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_DISCRIMINATOR => {
            let user_ata_in = input_accounts.get(5).unwrap().to_string();
            let user_ata_out = input_accounts.get(6).unwrap().to_string();

            let vault_a: String =
                parse_vault_a(user_ata_in, &input_accounts, &accounts, inner_instructions);
            let vault_b: String =
                parse_vault_b(user_ata_out, &input_accounts, &accounts, inner_instructions);

            result = Some(TradeInstruction {
                dapp_address: String::from("GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn"),
                name: String::from("Swap"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: vault_a,
                vault_b: vault_b,
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
