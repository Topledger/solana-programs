use crate::trade_instruction::TradeInstruction;

// TODO: Update these discriminators with the correct values for pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
const BUY_DISCRIMINATOR: u64 = u64::from_le_bytes([102, 6, 61, 18, 1, 218, 235, 234]); // Copied from 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
const SELL_DISCRIMINATOR: u64 = u64::from_le_bytes([51, 230, 133, 164, 1, 127, 131, 173]); // Copied from 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, _rest) = bytes_stream.split_at(8);
    if disc_bytes.len() < 8 { // Check length before unwrapping
        return None;
    }
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap(); // Safe unwrap after check
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        BUY_DISCRIMINATOR => {
            // TODO: Verify instruction name ("Buy"?), account indices, and FEE ACCOUNT INDEX (using 5) for pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
            result = Some(TradeInstruction {
                dapp_address: String::from("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
                name: String::from("Buy"), // Assuming name is "Buy"
                amm: accounts.get(0).map(|s| s.to_string()).unwrap_or_default(), // Your index 0
                vault_a: accounts.get(7).map(|s| s.to_string()).unwrap_or_default(), // Your index 7
                vault_b: accounts.get(8).map(|s| s.to_string()).unwrap_or_default(), // Your index 8
                fee_account: Some(accounts.get(10).map(|s| s.to_string()).unwrap_or_default()), // Added fee_account with placeholder index 5
                ..Default::default()
            });
        }
        SELL_DISCRIMINATOR => {
            // TODO: Verify instruction name ("Sell"?), account indices, and FEE ACCOUNT INDEX (using 5) for pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
            result = Some(TradeInstruction {
                dapp_address: String::from("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"),
                name: String::from("Sell"), // Assuming name is "Sell"
                amm: accounts.get(0).map(|s| s.to_string()).unwrap_or_default(), // Your index 0
                vault_a: accounts.get(7).map(|s| s.to_string()).unwrap_or_default(), // Your index 7
                vault_b: accounts.get(8).map(|s| s.to_string()).unwrap_or_default(), // Your index 8
                fee_account: Some(accounts.get(10).map(|s| s.to_string()).unwrap_or_default()), // Added fee_account with placeholder index 5
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
} 