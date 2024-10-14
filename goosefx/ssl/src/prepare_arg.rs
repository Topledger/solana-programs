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
        "CreateLiquidityAccount" => {}
        "Deposit" => {
            arg.deposit = Some(instruction.deposit.to_proto_struct());
        }
        "Withdraw" => {
            arg.withdraw = Some(instruction.withdraw.to_proto_struct());
        }
        "MintPt" => {
            arg.mint_pt = Some(instruction.mintPt.to_proto_struct());
        }
        "BurnPt" => {
            arg.burn_pt = Some(instruction.burnPt.to_proto_struct());
        }
        "Swap" => {
            arg.swap = Some(instruction.swap.to_proto_struct());
        }
        "CrankLiability" => {}
        _ => {}
    }

    return arg;
}
