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
        "CreateNft" => {
            arg.create_nft = Some(instruction.createNft.to_proto_struct());
        }
        "EditData" => {
            arg.edit_data = Some(instruction.editData.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
