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
        "InitializeFraktMarket" => {}
        "AddWhitelistToMarket" => {
            arg.add_whitelist_to_market = Some(instruction.addWhitelistToMarket.to_proto_struct());
        }
        "RemoveWhitelistFromMarket" => {}
        "ActivateFraktMarket" => {}
        "InitializeOracle" => {
            arg.initialize_oracle = Some(instruction.initializeOracle.to_proto_struct());
        }
        "SetOracleAuthority" => {
            arg.set_oracle_authority = Some(instruction.setOracleAuthority.to_proto_struct());
        }
        "SetOracleFloor" => {
            arg.set_oracle_floor = Some(instruction.setOracleFloor.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
