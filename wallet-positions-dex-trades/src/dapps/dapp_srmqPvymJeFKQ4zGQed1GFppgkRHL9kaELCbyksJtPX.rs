pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(2);
    let disc_bytes_arr: [u8; 2] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u16 = u16::from_be_bytes(disc_bytes_arr);

    match discriminator {
        10 => return true,
        _ => {}
    }

    false
}
