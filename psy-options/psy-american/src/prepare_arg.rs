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
        "InitializeMarket" => {
            arg.initialize_market = Some(instruction.initializeMarket.to_proto_struct());
        }
        "MintOption" => {
            arg.mint_option = Some(instruction.mintOption.to_proto_struct());
        }
        "MintOptionV2" => {
            arg.mint_option_v2 = Some(instruction.mintOptionV2.to_proto_struct());
        }
        "ExerciseOption" => {
            arg.exercise_option = Some(instruction.exerciseOption.to_proto_struct());
        }
        "ExerciseOptionV2" => {
            arg.exercise_option_v2 = Some(instruction.exerciseOptionV2.to_proto_struct());
        }
        "ClosePostExpiration" => {
            arg.close_post_expiration = Some(instruction.closePostExpiration.to_proto_struct());
        }
        "CloseOptionPosition" => {
            arg.close_option_position = Some(instruction.closeOptionPosition.to_proto_struct());
        }
        "BurnWriterForQuote" => {
            arg.burn_writer_for_quote = Some(instruction.burnWriterForQuote.to_proto_struct());
        }
        "InitSerumMarket" => {
            arg.init_serum_market = Some(instruction.initSerumMarket.to_proto_struct());
        }
        _ => {}
    }

    return arg;
}
