pub mod honest_node;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    u64,
};

use crate::{block::{Block}, digest::BlockHash, message::{Message}};

pub type NodeIdentifier = u32;
pub type Epoch = u64;

pub trait BaseNode {
    fn on_message_received(&mut self, message: &Message, message_queue: &mut VecDeque<Message>);

    fn at_time(&mut self, epoch: Epoch, message_queue: &mut VecDeque<Message>);
}

type BlockChainWithLength = HashMap<BlockHash, (Block, u32)>;

#[derive(Clone, Debug)]
struct Vote(u32, HashSet<NodeIdentifier>);

impl Vote {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn add_voter(&mut self, voter: NodeIdentifier) {
        self.1.insert(voter);
    }

    pub fn get_votes(&self) -> u32 {
        self.0
    }

    pub fn has_voted(&self, voter: NodeIdentifier) -> bool {
        self.1.contains(&voter)
    }
}