extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

extern crate chrono;
use chrono::prelude::*;

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
    pub program_data: Vec<String>,
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
                self.current_child_iterator =
                    Some(Box::new(LogContextIterator::new(&node.children_nodes)));
            }

            Some(node)
        } else {
            None
        }
    }
}

pub fn convert_to_date(ts: i64) -> Result<String, &'static str> {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).ok_or("Invalid timestamp")?;

    let dt: DateTime<Utc> = Utc.from_utc_datetime(&nt);
    Ok(dt.format("%Y-%m-%d").to_string())
}
