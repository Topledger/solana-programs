use regex::{Captures, Regex};
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

extern crate chrono;
use chrono::prelude::*;
use substreams_solana::pb::sf::solana::r#type::v1::{
    CompiledInstruction, Transaction,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogContext {
    pub program_id: String,
    pub depth: usize,
    pub id: usize,
    pub compute_units: usize,
    pub consumed_units: usize,
    pub children: Vec<usize>, // Indices of child nodes in temp_nodes
    pub children_nodes: Vec<LogContext>, // Actual child nodes
    pub program_logs: Vec<String>,
    pub program_data: Option<String>,
    pub failure_message: Option<String>,
}

pub struct LogContextIterator<'a> {
    root_nodes: &'a [LogContext],
    current_root_index: usize,
    current_child_iterator: Option<Box<LogContextIterator<'a>>>,
}

impl<'a> LogContextIterator<'a> {
    pub fn new(nodes: &'a [LogContext]) -> Self {
        LogContextIterator {
            root_nodes: nodes,
            current_root_index: 0,
            current_child_iterator: None,
        }
    }
}

impl<'a> Iterator for LogContextIterator<'a> {
    type Item = &'a LogContext;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut child_iter) = self.current_child_iterator {
            if let Some(child) = child_iter.next() {
                return Some(child);
            }
            // Current child iterator is exhausted.
            self.current_child_iterator = None;
        }

        if self.current_root_index < self.root_nodes.len() {
            let node = &self.root_nodes[self.current_root_index];
            self.current_root_index += 1;

            if !node.children_nodes.is_empty() {
                self.current_child_iterator = Some(Box::new(LogContextIterator::new(&node.children_nodes)));
            }

            Some(node)
        } else {
            None
        }
    }
}

pub fn convert_to_date(ts: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0);
    let dt: DateTime<Utc> = DateTime::from_utc(nt.unwrap(), Utc);
    let res = dt.format("%Y-%m-%d");
    return res.to_string();
}

pub fn parse_logs(logs: &[String]) -> Vec<LogContext> {
    let mut temp_nodes = Vec::new();
    let mut stack = Vec::new();
    let parser_re = Regex::new(
        concat!(
            r"(^Program (?P<program_id>[1-9A-HJ-NP-Za-km-z]{32,}) invoke \[(?P<depth>\d+)\]$)|",
            r"(^Program (?P<success_program_id>[1-9A-HJ-NP-Za-km-z]{32,}) success$)|",
            r"(^Program (?P<failed_program_id>[1-9A-HJ-NP-Za-km-z]{32,}) failed: (?P<failed_message>.+)$)|",
            r"(^Program (?P<consumed_program_id>[1-9A-HJ-NP-Za-km-z]{32,}) consumed (?P<consumed>\d+) of (?P<total_compute_units>\d+) compute units$)|",
            r"(^Program data: (?P<data>.+)$)|",
            r"(^Program log: (?P<log>.+)$)"
        )
    ).expect("Failed to compile regex");

    for log in logs {
        if let Some(captures) = parser_re.captures(log) {
            process_log_entry(&captures, &mut temp_nodes, &mut stack);
        }
    }

    build_final_result(&temp_nodes)
}

fn process_log_entry(
    captures: &Captures,
    temp_nodes: &mut Vec<LogContext>,
    stack: &mut Vec<usize>,
) {
    if let Some(program_id_capture) = captures.name("program_id") {
        let depth = captures["depth"].parse::<usize>().unwrap_or_default();
        let program_id = program_id_capture.as_str().to_string();

        adjust_stack_for_depth(depth, stack);

        let node_index = create_log_context(&program_id, depth, temp_nodes);
        add_node_to_hierarchy(depth, node_index, stack, temp_nodes);
    } else {
        update_existing_log_context(captures, temp_nodes, stack);
    }
}

fn adjust_stack_for_depth(depth: usize, stack: &mut Vec<usize>) {
    while stack.len() >= depth {
        stack.pop();
    }
}

fn create_log_context(program_id: &str, depth: usize, temp_nodes: &mut Vec<LogContext>) -> usize {
    let node = LogContext {
        program_id: program_id.to_string(),
        depth,
        id: temp_nodes.len(),
        consumed_units: 0,
        compute_units: 0,
        children: Vec::new(),
        children_nodes: Vec::new(),
        program_logs: Vec::new(),
        program_data: None,
        failure_message: None,
    };

    temp_nodes.push(node);
    temp_nodes.len() - 1
}

fn add_node_to_hierarchy(
    depth: usize,
    node_index: usize,
    stack: &mut Vec<usize>,
    temp_nodes: &mut Vec<LogContext>,
) {
    if depth > 1 && !stack.is_empty() {
        let parent_index = *stack.last().unwrap();
        temp_nodes[parent_index].children.push(node_index);
    }
    stack.push(node_index);
}

fn update_existing_log_context(
    captures: &Captures,
    temp_nodes: &mut Vec<LogContext>,
    stack: &mut Vec<usize>,
) {
    if let Some(last_index) = stack.last() {
        let context = &mut temp_nodes[*last_index];
        if let Some(consumed_capture) = captures.name("consumed") {
            let computed_capture = captures.name("total_compute_units");
            context.consumed_units += consumed_capture
                .as_str()
                .parse::<usize>()
                .unwrap_or_default();
            context.compute_units += computed_capture
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap_or_default();
        }
        if let Some(log_capture) = captures.name("log") {
            context.program_logs.push(log_capture.as_str().to_string());
        }
        if let Some(data_capture) = captures.name("data") {
            context.program_data = Some(data_capture.as_str().to_string());
        }
        if let Some(failed_message_capture) = captures.name("failed_message") {
            context.failure_message = Some(failed_message_capture.as_str().to_string());
            stack.pop();
        }
        if let Some(success_program_id) = captures.name("success_program_id") {
            if context.program_id == success_program_id.as_str() {
                stack.pop();
            }
        }
    }
}

fn build_final_result(temp_nodes: &[LogContext]) -> Vec<LogContext> {
    let mut result = Vec::new();
    for node in temp_nodes.iter().filter(|node| node.depth == 1) {
        let mut root_node = node.clone();
        build_hierarchy(&mut root_node, temp_nodes);
        result.push(root_node);
    }
    result
}

fn build_hierarchy(parent: &mut LogContext, temp_nodes: &[LogContext]) {
    for &child_index in &parent.children {
        let mut child_node = temp_nodes[child_index].clone();
        build_hierarchy(&mut child_node, temp_nodes);
        parent.children_nodes.push(child_node);
    }
    parent.children.clear();
}

pub fn calculate_byte_size(trx: &Transaction) -> usize {
    let version = trx.message.as_ref().unwrap().versioned;

    if version {
        calculate_versioned_transaction_size(trx)
    } else {
        calculate_legacy_transaction_size(trx)
    }
}

fn calculate_legacy_transaction_size(trx: &Transaction) -> usize {
    let instructions = &trx.message.as_ref().unwrap().instructions;
    let ixs_size = calculate_total_instructions_size(instructions);

    compact_array_size(trx.signatures.len(), 64) // signatures
        + 3 // header
        + compact_array_size(trx.message.as_ref().unwrap().account_keys.len(), 32) // accounts
        + 32 // blockhash
        + compact_header(instructions.len()) // instructions len
        + ixs_size // Instructions
}

pub fn calculate_versioned_transaction_size(trx: &Transaction) -> usize {
    let instructions = &trx.message.as_ref().unwrap().instructions;
    let ixs_size = calculate_total_instructions_size(instructions);

    let account_lookup_tables = &trx.message.as_ref().unwrap().address_table_lookups;
    let alt_size = account_lookup_tables.iter().fold(0, |acc, table| {
        acc + 32 // account_key_size
            + compact_array_size(table.readonly_indexes.len(), 1)
            + compact_array_size(table.writable_indexes.len(), 1)
    });

    compact_array_size(trx.signatures.len(), 64) // signatures
        + 3 // header
        + compact_array_size(trx.message.as_ref().unwrap().account_keys.len(), 32) // accounts
        + 32 // blockhash
        + compact_header(instructions.len()) // instructions
        + ixs_size
        + compact_header(account_lookup_tables.len()) //ALT len size
        + alt_size //ALT
        + 1 // version
}

pub fn calculate_instruction_size(ix: &CompiledInstruction) -> usize {
    let n_indexes = ix.accounts.len();
    let opaque_data = ix.data.len();

    1 // PID index
    + compact_array_size(n_indexes, 1)
    + compact_array_size(opaque_data, 1)
}

// Function to calculate the total size of all instructions
pub fn calculate_total_instructions_size(instructions: &[CompiledInstruction]) -> usize {
    instructions
        .iter()
        .map(|ix| calculate_instruction_size(ix))
        .sum()
}

// Compact array and compact-u16 functions

fn compact_header(n: usize) -> usize {
    const LOW_VALUE: usize = 127;
    const HIGH_VALUE: usize = 16383;

    if n <= LOW_VALUE {
        1
    } else if n <= HIGH_VALUE {
        2
    } else {
        3
    }
}

pub fn compact_array_size(n: usize, size: usize) -> usize {
    compact_header(n) + n * size
}