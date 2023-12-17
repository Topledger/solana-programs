use regex::{Captures, Regex};

extern crate chrono;
use chrono::prelude::*;

#[derive(Clone)]
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

pub fn convert_to_date(ts: i64) -> Result<String, &'static str> {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).ok_or("Invalid timestamp")?;

    let dt: DateTime<Utc> = Utc.from_utc_datetime(&nt);
    Ok(dt.format("%Y-%m-%d").to_string())
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

// testing functions ( not used in substreams )

fn _print_nested_tree(node: &LogContext, depth: usize) {
    let indent = "    ".repeat(depth);
    let failure_info = if let Some(ref message) = node.failure_message {
        format!(", Failure: {}", message)
    } else {
        String::from("")
    };

    println!("{}Program ID: {}, Consumed Units: {}, Compute Units: {}, Depth: {}, ID: {}, Program Data: {}, Program Logs: {:?}{}",
        indent,
        node.program_id,
        node.consumed_units,
        node.compute_units,
        node.depth,
        node.id,
        node.program_data.as_ref().unwrap_or(&String::from("None")),
        node.program_logs,
        failure_info
    );

    for child in &node.children_nodes {
        _print_nested_tree(child, depth + 1);
    }
}

fn _main() {
    // Example log messages
    let log_messages_vec: Vec<&str> = [
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe invoke [1]",
        "Program log: Instruction 6: InitializeParimutuelPositionAccount",
        "Program log: Creating ParimutuelTraderPositionAccount...",
        "Program log: parimutuel_trader_key 9VnAD3WqihcNqmtNSqEXQrkWncxHLk7uZWxpAVmdAAj6",
        "Program log: creating parimutuel_trader_position_seeds...",
        "Program log: derived parimutuel_trader_position_seeds",
        "Program log: Transfer 2088000 lamports to the new account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Allocate space for the account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Assign program to the GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: ParimutuelTraderPositionAccount created.",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe consumed 50822 of 800000 compute units",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe success",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe invoke [1]",
        "Program log: Instruction 5: InitializeTraderAccounts",
        "Program log: Running validations...",
        "Program log: Checking protocol token account is_ata",
        "Program log: Checking settlement token account is_ata",
        "Program log: Validitions succeeded.",
        "Program log: Account already exists.",
        "Program log: Account already exists.",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe consumed 21050 of 749178 compute units",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe success",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe invoke [1]",
        "Program log: Instruction 9: UpdatePosition",
        "Program log: Validating...",
        "Program log: Validations succeeded.",
        "Program log: parimutuel_trader_entry GjogqXbDsBJWNAiy1ZoEPNMxEM27CXmV9XEx2QCWGyH5 GjogqXbDsBJWNAiy1ZoEPNMxEM27CXmV9XEx2QCWGyH5 - sequence: 1672361357144",
        "Program log: creating parimutuel_trader_position_seeds...",
        "Program log: derived parimutuel_trader_entry_seeds",
        "Program log: Transfer 1746960 lamports to the new account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Allocate space for the account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Assign program to the GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Amount: 125000000",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: Transfer",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 662609 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program log: Transferred.",
        "Program log: Size: 125000000",
        "Program log: Entering Long Position",
        "Program log: serializing parimutuel_trader_position_account...",
        "Program log: serializing parimutuel",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe consumed 76880 of 728128 compute units",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe success",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe invoke [1]",
        "Program log: Instruction 9: UpdatePosition",
        "Program log: Validating...",
        "Program log: Validations succeeded.",
        "Program log: parimutuel_trader_entry EZazx9HvegyksCUJuuurvJ4ehsgpdFaD4KCZCGnHHhzk EZazx9HvegyksCUJuuurvJ4ehsgpdFaD4KCZCGnHHhzk - sequence: 1672361357145",
        "Program log: creating parimutuel_trader_position_seeds...",
        "Program log: derived parimutuel_trader_entry_seeds",
        "Program log: Transfer 1746960 lamports to the new account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Allocate space for the account",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Assign program to the GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Amount: 125000000",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: Transfer",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 584253 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program log: Transferred.",
        "Program log: Size: 125000000",
        "Program log: Entering Short Position",
        "Program log: serializing parimutuel_trader_position_account...",
        "Program log: serializing parimutuel",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe consumed 78354 of 651248 compute units",
        "Program GUhB2ohrfqWspztgCrQpAmeVFBWmnWYhPcZuwY52WWRe success"
    ].to_vec();

    let log_messages: Vec<String> = log_messages_vec.iter().map(|s| s.to_string()).collect();

    // Compile the regex

    // Parse the logs
    let parsed_logs = parse_logs(&log_messages);

    for log_context in parsed_logs {
        _print_nested_tree(&log_context, 0);
    }
}
