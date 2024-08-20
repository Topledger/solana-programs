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
        "ReclaimMintAuthority" => {
            arg.reclaim_mint_authority = Some(instruction.reclaimMintAuthority.to_proto_struct());
        }
        "Stake" => {
            arg.stake = Some(instruction.stake.to_proto_struct());
        }
        "Unstake" => {
            arg.unstake = Some(instruction.unstake.to_proto_struct());
        }
        "EmitPrice" => {}
        _ => {}
    }

    return arg;
}
