use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u64 = u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);
const SWAP_EXACT_OUT_DISCRIMINATOR: u64 = u64::from_le_bytes([250, 73, 101, 33, 38, 207, 75, 184]);
const SWAP_WITH_PRICE_IMPACT_DISCRIMINATOR: u64 =
    u64::from_le_bytes([56, 173, 230, 208, 173, 228, 156, 205]);

// New V2 instruction discriminators
const SWAP2_DISCRIMINATOR: u64 = u64::from_le_bytes([65, 75, 63, 76, 235, 91, 91, 136]);
const SWAP_EXACT_OUT2_DISCRIMINATOR: u64 = u64::from_le_bytes([43, 215, 247, 132, 137, 60, 243, 81]);
const SWAP_WITH_PRICE_IMPACT2_DISCRIMINATOR: u64 =
    u64::from_le_bytes([74, 98, 192, 214, 177, 51, 75, 51]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("Swap"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_EXACT_OUT_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("SwapExactOut"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_WITH_PRICE_IMPACT_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("SwapWithPriceImpact"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("Swap2"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_EXACT_OUT2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("SwapExactOut2"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_WITH_PRICE_IMPACT2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("SwapWithPriceImpact2"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
