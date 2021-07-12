use core::time;
use std::{collections::{HashMap, VecDeque}, env, thread};

use message::Message;
use node::{BaseNode, NodeIdentifier};

use log::info;
use simple_logger::SimpleLogger;

use crate::node::Node;

pub mod message;
pub mod node;
pub mod block;

fn run_network<N: BaseNode>(nodes_map: &HashMap<NodeIdentifier, N>) {
    let mut current_epoch: u64 = 0;
    let mut message_queue = VecDeque::<Message>::new();

    loop {
        for node in nodes_map.values() {
            let mut messages = node.at_time(current_epoch);
            message_queue.append(&mut messages);
        }

        let messages_to_process_count = message_queue.len();

        for _ in 0..messages_to_process_count {
            if let Some(message) = message_queue.pop_front() {
                let mut messages = nodes_map[&message.to].on_message_received(message);
                message_queue.append(&mut messages);
            }
        }

        current_epoch += 1;
        // Artificially wait to be able to examine logs
        let duration_second = time::Duration::from_secs(1);
        thread::sleep(duration_second);
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if let Some(node_count_arg) = args.get(1) {
        let node_count: NodeIdentifier = node_count_arg.parse().unwrap();
        info!(target: "main", "Creating '{:?}' nodes", node_count);
        let id_range = 0..node_count;
        let nodes_map: HashMap<NodeIdentifier, Node> = id_range.map(|id| (id, Node::new(id, node_count))).collect();

        run_network(&nodes_map);
    }
    println!("Please provide number of nodes to run the Streamlet");
}