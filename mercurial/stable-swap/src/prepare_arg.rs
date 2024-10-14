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
        "AddLiquidity" => {
            arg.add_liquidity = Some(instruction.addLiquidity.to_proto_struct());
        }
        "RemoveLiquidity" => {
            arg.remove_liquidity = Some(instruction.removeLiquidity.to_proto_struct());
        }
        "RemoveLiquidityOneToken" => {
            arg.remove_liquidity_one_token =
                Some(instruction.removeLiquidityOneToken.to_proto_struct());
        }
        "Exchange" => {
            arg.exchange = Some(instruction.exchange.to_proto_struct());
        }
        "SetAdminSetting" => {
            arg.set_admin_setting = Some(instruction.setAdminSetting.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
