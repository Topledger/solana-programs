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
        "InitUpdateAuthority" => {
            arg.init_update_authority = Some(instruction.initUpdateAuthority.to_proto_struct());
        }
        "InitUpdateWhitelist" => {
            arg.init_update_whitelist = Some(instruction.initUpdateWhitelist.to_proto_struct());
        }
        "InitUpdateMintProof" => {
            arg.init_update_mint_proof = Some(instruction.initUpdateMintProof.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
