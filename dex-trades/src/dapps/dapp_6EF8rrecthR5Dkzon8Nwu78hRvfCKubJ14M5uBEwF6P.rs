use crate::trade_instruction::TradeInstruction;

const BUY_DISCRIMINATOR: u64 = u64::from_le_bytes([102, 6, 61, 18, 1, 218, 235, 234]);
const SELL_DISCRIMINATOR: u64 = u64::from_le_bytes([51, 230, 133, 164, 1, 127, 131, 173]);
const BUY_EXACT_SOL_IN_DISCRIMINATOR: u64 =
    u64::from_le_bytes([56, 252, 116, 8, 158, 223, 205, 95]);
const BUY_V2_DISCRIMINATOR: u64 = u64::from_le_bytes([184, 23, 238, 97, 103, 197, 211, 61]);
const SELL_V2_DISCRIMINATOR: u64 = u64::from_le_bytes([93, 246, 130, 60, 231, 233, 64, 178]);
const BUY_EXACT_QUOTE_IN_V2_DISCRIMINATOR: u64 =
    u64::from_le_bytes([194, 171, 28, 70, 104, 77, 91, 47]);

const PUMP_FUN_PROGRAM: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        BUY_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from(PUMP_FUN_PROGRAM),
                name: String::from("Buy"),
                amm: accounts.get(3).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        SELL_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from(PUMP_FUN_PROGRAM),
                name: String::from("Sell"),
                amm: accounts.get(3).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        BUY_EXACT_SOL_IN_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from(PUMP_FUN_PROGRAM),
                name: String::from("BuyExactSolIn"),
                amm: accounts.get(3).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        BUY_V2_DISCRIMINATOR => {
            result = Some(parse_v2_instruction(accounts, "BuyV2"));
        }
        SELL_V2_DISCRIMINATOR => {
            result = Some(parse_v2_instruction(accounts, "SellV2"));
        }
        BUY_EXACT_QUOTE_IN_V2_DISCRIMINATOR => {
            result = Some(parse_v2_instruction(accounts, "BuyExactQuoteInV2"));
        }
        _ => {}
    }

    return result;
}

fn parse_v2_instruction(accounts: Vec<String>, name: &str) -> TradeInstruction {
    TradeInstruction {
        dapp_address: String::from(PUMP_FUN_PROGRAM),
        name: String::from(name),
        amm: accounts.get(10).unwrap().to_string(),
        vault_a: accounts.get(11).unwrap().to_string(),
        vault_b: accounts.get(12).unwrap().to_string(),
        base_mint: Some(accounts.get(1).unwrap().to_string()),
        quote_mint: Some(accounts.get(2).unwrap().to_string()),
        fee_account: Some(accounts.get(17).unwrap().to_string()),
        ..Default::default()
    }
}
