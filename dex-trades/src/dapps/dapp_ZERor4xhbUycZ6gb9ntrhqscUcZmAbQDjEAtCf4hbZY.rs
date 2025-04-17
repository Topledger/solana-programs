use crate::trade_instruction::TradeInstruction;

// Set the discriminator for the Swap instruction to 6.
const SWAP_DISCRIMINATOR: u8 = 6; // Updated discriminator value

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, _rest) = bytes_stream.split_at(1); // Read only the first byte
    if disc_bytes.is_empty() {
        return None;
    }
    let discriminator: u8 = disc_bytes[0]; // Use the first byte directly

    let mut result = None;

    match discriminator {
        SWAP_DISCRIMINATOR => { // This will now check for 6
            // TODO: Verify account indices for amm, vault_a, vault_b, fee_account
            result = Some(TradeInstruction {
                dapp_address: String::from("ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY"),
                name: String::from("Swap"),
                // Apply fix for unwrap_or_default
                amm: accounts.get(0).map(|s| s.to_string()).unwrap_or_default(), // Placeholder index 0
                vault_a: accounts.get(2).map(|s| s.to_string()).unwrap_or_default(), // Placeholder index 1
                vault_b: accounts.get(4).map(|s| s.to_string()).unwrap_or_default(), // Placeholder index 2
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
} 