const SWAP_FUND_TOKENS_DISCRIMINATOR: u64 =
    u64::from_le_bytes([112, 246, 21, 136, 172, 62, 27, 20]);

pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    match discriminator {
        SWAP_FUND_TOKENS_DISCRIMINATOR => {
            return true;
        }
        _ => {}
    }

    false
}
