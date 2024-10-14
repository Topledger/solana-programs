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
        "InitializeLazyTransactionsV0" => {
            arg.initialize_lazy_transactions_v0 =
                Some(instruction.initializeLazyTransactionsV0.to_proto_struct());
        }
        "ExecuteTransactionV0" => {
            arg.execute_transaction_v0 = Some(instruction.executeTransactionV0.to_proto_struct());
        }
        "CloseMarkerV0" => {
            arg.close_marker_v0 = Some(instruction.closeMarkerV0.to_proto_struct());
        }
        "CloseCanopyV0" => {}
        "UpdateLazyTransactionsV0" => {
            arg.update_lazy_transactions_v0 =
                Some(instruction.updateLazyTransactionsV0.to_proto_struct());
        }
        "SetCanopyV0" => {
            arg.set_canopy_v0 = Some(instruction.setCanopyV0.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
