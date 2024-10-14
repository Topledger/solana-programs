use crate::{
    instructions::{self, parser::Instruction},
    pb::sf::solana::block_meta::v1::Arg,
};
use instructions::parser::parse_instruction;

pub fn prepare_arg(instruction_data: Vec<u8>, tx_id: String) -> Arg {
    let mut arg: Arg = Arg::default();
    let instruction: Instruction = parse_instruction(instruction_data);

    arg.instruction_type = instruction.instructionType;

    match arg.instruction_type.as_str() {
        "Initialize" => {
            arg.initialize = Some(instruction.initialize.to_proto_struct());
        }
        "Swap" => {
            arg.swap = Some(instruction.swap.to_proto_struct());
        }
        "Deposit" => {
            arg.deposit = Some(instruction.deposit.to_proto_struct());
        }
        "Withdraw" => {
            arg.withdraw = Some(instruction.withdraw.to_proto_struct());
        }
        "WithdrawOne" => {
            arg.withdraw_one = Some(instruction.withdrawOne.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
