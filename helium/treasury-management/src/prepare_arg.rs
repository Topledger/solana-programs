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
        "InitializeTreasuryManagementV0" => {
            arg.initialize_treasury_management_v0 =
                Some(instruction.initializeTreasuryManagementV0.to_proto_struct());
        }
        "UpdateTreasuryManagementV0" => {
            arg.update_treasury_management_v0 =
                Some(instruction.updateTreasuryManagementV0.to_proto_struct());
        }
        "RedeemV0" => {
            arg.redeem_v0 = Some(instruction.redeemV0.to_proto_struct());
        }
        "CorrectTreasuriesV0" => {}
        _ => {}
    }

    return arg;
}
