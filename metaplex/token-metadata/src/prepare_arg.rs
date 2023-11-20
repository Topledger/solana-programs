use crate::{
    instructions::{self, parser::Instruction},
    pb::sf::solana::block_meta::v1::Arg,
};
use instructions::parser::parse_instruction;

pub fn prepare_arg(
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> Arg {
    let mut arg: Arg = Arg::default();
    let instruction: Instruction = parse_instruction(instruction_data);

    return arg;
}
