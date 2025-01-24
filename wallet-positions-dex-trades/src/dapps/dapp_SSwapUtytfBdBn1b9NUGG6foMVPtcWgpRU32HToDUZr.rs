pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    match discriminator {
        1 => {
            return true;
        }
        _ => {}
    }

    false
}
