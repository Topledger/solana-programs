use crate::instructions::{self, parser::Instruction};
use instructions::parser::parse_instruction;

pub fn prepare_arg(instruction_data: Vec<u8>, tx_id: String) -> String {
    let instruction: Instruction = parse_instruction(instruction_data);
    return instruction.instructionType;
}
