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
        "StakeWrappedSol" => {
            arg.stake_wrapped_sol = Some(instruction.stakeWrappedSol.to_proto_struct());
        }
        "SwapViaStake" => {
            arg.swap_via_stake = Some(instruction.swapViaStake.to_proto_struct());
        }
        "PrefundWithdrawStake" => {
            arg.prefund_withdraw_stake = Some(instruction.prefundWithdrawStake.to_proto_struct());
        }
        "PrefundSwapViaStakeLayout" => {
            arg.prefund_swap_via_stake = Some(instruction.prefundSwapViaStake.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
