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
        "Create" => {
            arg.create = Some(instruction.create.to_proto_struct());
        }
        "Update" => {
            arg.update = Some(instruction.update.to_proto_struct());
        }
        "Transfer" => {
            arg.transfer = Some(instruction.transfer.to_proto_struct());
        }
        "Delete" => {
            arg.delete = Some(instruction.delete.to_proto_struct());
        }
        "Resize" => {
            arg.resize = Some(instruction.resize.to_proto_struct());
        }
        "Extend" => {
            arg.extend = Some(instruction.extend.to_proto_struct());
        }
        "ImmutableOwner" => {
            arg.immutable_owner = Some(instruction.immutableOwner.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
