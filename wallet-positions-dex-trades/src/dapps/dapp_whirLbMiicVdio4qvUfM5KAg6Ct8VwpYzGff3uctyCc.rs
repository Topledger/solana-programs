const SWAP_DISCRIMINATOR: u64 = 14449647541112719096;
const SWAP_V2_DISCRIMINATOR: u64 = 7070309578724672555;
const TWO_HOP_SWAP_DISCRIMINATOR: u64 = 16635068063392030915;
const TWO_HOP_SWAP_V2_DISCRIMINATOR: u64 = 8485347938364657594;

pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    match discriminator {
        SWAP_DISCRIMINATOR => {
            return true;
        }
        SWAP_V2_DISCRIMINATOR => {
            return true;
        }
        TWO_HOP_SWAP_DISCRIMINATOR => {
            return true;
        }
        TWO_HOP_SWAP_V2_DISCRIMINATOR => {
            return true;
        }
        _ => {}
    }

    false
}
