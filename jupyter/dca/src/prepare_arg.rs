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
        "OpenDca" => {
            arg.open_dca = Some(instruction.openDca.to_proto_struct());
        }
        "Withdraw" => {
            arg.withdraw = Some(instruction.withdraw.to_proto_struct());
        }
        "Deposit" => {
            arg.deposit = Some(instruction.deposit.to_proto_struct());
        }
        "WithdrawFees" => {
            arg.withdraw_fees = Some(instruction.withdrawFees.to_proto_struct());
        }
        "FulfillFlashFill" => {
            arg.fulfill_flash_fill = Some(instruction.fulfillFlashFill.to_proto_struct());
        }
        "OpenDcaV2" => {
            arg.open_dca_v2 = Some(instruction.openDcaV2.to_proto_struct());
        }
        "FulfillDlmmFill" => {
            arg.fulfill_dlmm_fill = Some(instruction.fulfillDlmmFill.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
