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
        "InitRoot" => {
            arg.init_root = Some(instruction.initRoot.to_proto_struct());
        }
        "SetOperator" => {
            arg.set_operator = Some(instruction.setOperator.to_proto_struct());
        }
        "SetAdmin" => {
            arg.set_admin = Some(instruction.setAdmin.to_proto_struct());
        }
        "SetAlternateStaker" => {
            arg.set_alternate_staker = Some(instruction.setAlternateStaker.to_proto_struct());
        }
        "Split" => {
            arg.split = Some(instruction.split.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
