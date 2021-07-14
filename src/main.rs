use core::time;
use std::{collections::{VecDeque}, env, thread};

use message::Message;
use node::{BaseNode, NodeIdentifier};

use log::{info, warn};
use simple_logger::SimpleLogger;

use block::INIT_EPOCH;

use crate::node::honest_node::HonestNode;

pub mod message;
pub mod node;
pub mod block;
pub mod digest;

fn run_network<N: BaseNode>(mut nodes_map: Vec<N>) {
    let mut current_epoch: u64 = INIT_EPOCH + 1;
    let mut message_queue = VecDeque::<Message>::new();

    loop {
        for node in &mut nodes_map {
            node.at_time(current_epoch, &mut message_queue);
        }

        // In normal environment this is probably a bad idea - we're sure that nodes don't echo infinitely their messages
        while message_queue.len() > 0 {
            if let Some(message) = message_queue.pop_front() {
                if let Some(node) = nodes_map.get_mut(message.to as usize) {
                    node.on_message_received(&message, &mut message_queue);
                } else {
                    warn!(target: "network", "Missed message to node with id {:?} from node {:?}", message.to, message.from)
                }
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
        let nodes: Vec<HonestNode> = id_range.map(|id| HonestNode::new(id, node_count)).collect();

        run_network(nodes);
    }
    println!("Please provide number of nodes to run the Streamlet");
}
