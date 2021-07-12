use std::{collections::VecDeque, u64};

use log::{debug, info};

use crate::message::Message;

pub type NodeIdentifier = u32;
pub type Epoch = u64;

pub trait BaseNode {
    fn on_message_received(&self, message: Message) -> VecDeque<Message>;

    fn at_time(&self, epoch: Epoch) -> VecDeque<Message>;
}

pub struct Node {
    id: NodeIdentifier,
    node_count: u32
}

impl Node {
    pub fn new(id: NodeIdentifier, node_count: u32) -> Self {
        Node { id, node_count }
    }

    fn elect_leader(&self, epoch: Epoch) -> NodeIdentifier {
        (epoch as u32) % self.node_count
    }

    fn name(&self) -> String {
        format!("Node {:?}", self.id)
    }
}

impl BaseNode for Node {
    fn on_message_received(&self, message: Message) -> VecDeque<Message> {
        debug!(target: &self.name(), "Received a message");
        VecDeque::new()
    }

    fn at_time(&self, epoch: Epoch) -> VecDeque<Message> {
        debug!(target: &self.name(), "Epoch {:?}", epoch);
        VecDeque::new()
    }
}
