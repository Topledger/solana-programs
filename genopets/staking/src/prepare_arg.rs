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
        "ChangeTotalGeneAllocated" => {
            arg.change_total_gene_allocated =
                Some(instruction.changeTotalGeneAllocated.to_proto_struct());
        }
        "ChangePoolWeight" => {
            arg.change_pool_weight = Some(instruction.changePoolWeight.to_proto_struct());
        }
        "CreateGlobalState" => {
            arg.create_global_state = Some(instruction.createGlobalState.to_proto_struct());
        }
        "CreateStakingPool" => {
            arg.create_staking_pool = Some(instruction.createStakingPool.to_proto_struct());
        }
        "Stake" => {
            arg.stake = Some(instruction.stake.to_proto_struct());
        }
        "ReLockDeposit" => {
            arg.re_lock_deposit = Some(instruction.reLockDeposit.to_proto_struct());
        }
        "Withdraw" => {
            arg.withdraw = Some(instruction.withdraw.to_proto_struct());
        }
        "ClaimRewards" => {
            arg.claim_rewards = Some(instruction.claimRewards.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
