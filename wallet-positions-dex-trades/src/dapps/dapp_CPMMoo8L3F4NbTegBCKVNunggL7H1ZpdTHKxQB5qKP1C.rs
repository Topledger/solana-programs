const SWAP_BASE_INPUT_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([143, 190, 90, 218, 196, 30, 51, 222]);
const SWAP_BASE_OUTPUT_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([55, 217, 98, 86, 163, 74, 180, 173]);

pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    match discriminator {
        SWAP_BASE_INPUT_PARTNER_DISCRIMINATOR => {
            return true;
        }
        SWAP_BASE_OUTPUT_PARTNER_DISCRIMINATOR => {
            return true;
        }
        _ => {}
    }

    false
}
