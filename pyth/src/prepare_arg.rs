use crate::{
    instructions::{self, parser::Instruction},
    pb::sf::solana::block_meta::v1::Arg,
};
use instructions::parser::parse_instruction;

pub fn prepare_arg(instruction_data: Vec<u8>, tx_id: String) -> Arg {
    let mut arg: Arg = Arg::default();
    let mut instruction: Instruction = parse_instruction(instruction_data);

    arg.instruction_type = instruction.instructionType;

    match arg.instruction_type.as_str() {
        "InitMapping" => {}
        "AddMapping" => {}
        "AddProduct" => {}
        "AddPrice" => {
            arg.add_price_args = Some(instruction.addPriceArgs.to_proto_struct());
        }
        "AddPublisher" => {
            arg.add_publisher_args = Some(instruction.addPublisherArgs.to_proto_struct());
        }
        "DeletePublisher" => {
            arg.delete_publisher_args = Some(instruction.deletePublisherArgs.to_proto_struct());
        }
        "UpdatePrice" => {
            arg.update_price_args = Some(instruction.updatePriceArgs.to_proto_struct());
        }
        "AggregatePrice" => {}
        "SetMinPublishers" => {
            arg.set_min_publishers_args = Some(instruction.setMinPublishersArgs.to_proto_struct());
        }
        "UpdatePriceNoFailOnError" => {
            arg.update_price_no_fail_on_error_args =
                Some(instruction.updatePriceNoFailOnErrorArgs.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
