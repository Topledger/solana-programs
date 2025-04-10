const BUY_DISCRIMINATOR: u64 = u64::from_le_bytes([102, 6, 61, 18, 1, 218, 235, 234]);
const SELL_DISCRIMINATOR: u64 = u64::from_le_bytes([51, 230, 133, 164, 1, 127, 131, 173]);

pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    match discriminator {
        BUY_DISCRIMINATOR => {
            return true;
        }
        SELL_DISCRIMINATOR => {
            return true;
        }
        _ => {}
    }

    false
}
