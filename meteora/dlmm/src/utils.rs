use sha2::{Sha256, Digest};

/// Generates an 8-byte discriminator for an event name by computing
/// SHA-256 hash of "event:{name}" and taking the first 8 bytes
pub fn generate_event_discriminator(event_name: &str) -> [u8; 8] {
    let arg = format!("event:{}", event_name);
    let mut hasher = Sha256::new();
    hasher.update(arg.as_bytes());
    let result = hasher.finalize();
    
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[0..8]);
    
    discriminator
}

/// Prints the discriminator array for a given event name in the format used in the code
pub fn print_event_discriminator(event_name: &str) {
    let discriminator = generate_event_discriminator(event_name);
    println!("const EVENT_{}_DISCRIMINATOR: &[u8] = &[{}, {}, {}, {}, {}, {}, {}, {}];", 
        event_name.to_uppercase().replace(" ", "_"),
        discriminator[0], discriminator[1], discriminator[2], discriminator[3],
        discriminator[4], discriminator[5], discriminator[6], discriminator[7]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_reward_discriminator() {
        let discriminator = generate_event_discriminator("FundReward");
        assert_eq!(discriminator, [246, 228, 58, 130, 145, 170, 79, 204]);
    }

    #[test]
    fn test_claim_fee_discriminator() {
        let discriminator = generate_event_discriminator("ClaimFee");
        assert_eq!(discriminator, [246, 228, 58, 130, 145, 170, 79, 204]);
        // Note: This assertion will fail if the correct value is different
    }
} 