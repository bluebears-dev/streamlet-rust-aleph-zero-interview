use std::{
    collections::{HashMap, VecDeque},
    u64,
};

use log::debug;

use crate::{block::{Block, BlockHash}, message::{Message, MessageContent, ProposalMessage}};

pub type NodeIdentifier = u32;
pub type Epoch = u64;

pub trait BaseNode {
    fn on_message_received(&mut self, message: &Message, message_queue: &mut VecDeque<Message>);

    fn at_time(&self, epoch: Epoch, message_queue: &mut VecDeque<Message>);
}

type BlockChainWithLength = HashMap<BlockHash, (Block, u32)>;
pub struct Node {
    id: NodeIdentifier,
    node_count: u32,

    longest_chain_key: BlockHash,
    finalized_blocks: Vec<BlockHash>,
    blocks: BlockChainWithLength,
}

impl Node {
    pub fn new(id: NodeIdentifier, node_count: u32) -> Self {
        let mut finalized_blocks = Vec::new();
        let mut blocks = HashMap::new();
        let genesis_block = Block::genesis_block();
        let block_key = genesis_block.digest();

        finalized_blocks.push(block_key.to_owned());
        blocks.insert(block_key.to_owned(), (genesis_block, 1));

        Node {
            id,
            node_count,
            finalized_blocks,
            blocks,
            longest_chain_key: block_key,
        }
    }

    fn elect_leader(&self, epoch: Epoch) -> NodeIdentifier {
        (epoch as u32) % self.node_count
    }

    fn name(&self) -> String {
        format!("Node {:?}", self.id)
    }

    fn broadcast_block(&self, block: Block, message_queue: &mut VecDeque<Message>) {
        for target_id in 0..self.node_count {
            if target_id != self.id {
                message_queue.push_back(ProposalMessage::new(self.id, target_id, block.clone()))
            }
        }
    }
}

impl BaseNode for Node {
    fn on_message_received(&mut self, message: &Message, message_queue: &mut VecDeque<Message>) {
        debug!(target: &self.name(), "Received a message");
        match message.content {
            MessageContent::Proposal(ProposalMessage { block }) => {
                let (_, length) = self.blocks[&block.parent_digest];
                self.blocks
                    .insert(block.digest(), (block.clone(), length + 1));
            }
            MessageContent::Vote(_) => todo!(),
        }
    }

    fn at_time(&self, epoch: Epoch, message_queue: &mut VecDeque<Message>) {
        let leader_id = self.elect_leader(epoch);
        if leader_id == self.id {
            debug!(target: &self.name(), "Is leader in {:?} epoch", epoch);
            let (longest_chain_head, _length) = &self.blocks[&self.longest_chain_key];

            let block = Block::new(epoch, longest_chain_head.digest());
            self.broadcast_block(block, message_queue);
        }
    }
}
