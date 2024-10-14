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
        "InitializeLazyDistributorV0" => {
            arg.initialize_lazy_distributor_v0 =
                Some(instruction.initializeLazyDistributorV0.to_proto_struct());
        }
        "InitializeCompressionRecipientV0" => {
            arg.initialize_compression_recipient_v0 = Some(
                instruction
                    .initializeCompressionRecipientV0
                    .to_proto_struct(),
            );
        }
        "SetCurrentRewardsV0" => {
            arg.set_current_rewards_v0 = Some(instruction.setCurrentRewardsV0.to_proto_struct());
        }
        "DistributeCompressionRewardsV0" => {
            arg.distribute_compression_rewards_v0 =
                Some(instruction.distributeCompressionRewardsV0.to_proto_struct());
        }
        "UpdateLazyDistributorV0" => {
            arg.update_lazy_distributor_v0 =
                Some(instruction.updateLazyDistributorV0Layout.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
