use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use log::{debug, error, info, trace};

use crate::{block::{Block}, digest::{BlockHash, sha512_digest}, message::{Message, MessageContent, ProposalMessage, VoteMessage}};

use super::{BaseNode, BlockChainWithLength, Epoch, NodeIdentifier, Vote};

pub struct HonestNode {
    id: NodeIdentifier,
    node_count: u32,

    longest_notarized_chain_head_digests: Vec<BlockHash>,
    finalized_chain_head_digest: BlockHash,
    blocks: BlockChainWithLength,
    votes: HashMap<BlockHash, Vote>,
    current_leader: NodeIdentifier,
}

impl HonestNode {
    pub fn new(id: NodeIdentifier, node_count: u32) -> Self {
        let mut blocks = HashMap::new();
        let genesis_block = Block::genesis_block();
        let block_key = genesis_block.digest();

        blocks.insert(block_key.to_owned(), (genesis_block, 1));

        HonestNode {
            id,
            node_count,
            blocks,
            longest_notarized_chain_head_digests: vec![block_key],
            finalized_chain_head_digest: block_key,
            votes: HashMap::new(),
            current_leader: 0,
        }
    }

    fn is_leader(&self, epoch: Epoch) -> bool {
        self.calculate_leader_id(epoch) == self.id
    }

    fn calculate_leader_id(&self, epoch: Epoch) -> NodeIdentifier {
        let mut sum: u32 = 0;
        for byte in sha512_digest(&epoch.to_le_bytes()).iter() {
            sum += *byte as u32;
        }
        sum % self.node_count
    }

    fn name(&self) -> String {
        format!("Node {:?}", self.id)
    }

    fn broadcast(&self, message: &Message, message_queue: &mut VecDeque<Message>) {
        for target_id in 0..self.node_count {
            if target_id != self.id {
                let new_message = Message {
                    to: target_id,
                    from: message.from,
                    content: message.content,
                };

                message_queue.push_back(new_message)
            }
        }
    }

    fn can_block_be_notarized(&self, votes: u32) -> bool {
        votes >= self.node_count * 2 / 3
    }

    fn notarize_block(&mut self, block_digest: BlockHash) {
        self.blocks
            .entry(block_digest)
            .and_modify(|(block, _length)| block.notarize());
        info!(target: &self.name(), "Notarized block {:?}", String::from_utf8_lossy(&block_digest));
    }

    fn can_block_be_finalized(&self, block: &Block, expected_epoch: Epoch) -> bool {
        block.is_notarized && block.creation_epoch == expected_epoch
    }

    fn try_finalize_chain(&mut self, block_digest: BlockHash) {
        match self.blocks.entry(block_digest) {
            Entry::Occupied(entry) => {
                let block = entry.into_mut().0;

                if !block.is_notarized {
                    return;
                }

                let can_be_final = self.blocks.get(&block.parent_digest).and_then(|(parent, _length)| {
                    if !self.can_block_be_finalized(parent, block.creation_epoch - 1) {
                        None
                    } else {
                        self.blocks.get(&parent.parent_digest)
                    }
                }).and_then(|(grandparent, _length)| {
                    if !self.can_block_be_finalized(grandparent, block.creation_epoch - 2) {
                        None
                    } else {
                        Some(true)
                    }
                }).unwrap_or(false);
                
                if can_be_final {
                    self.finalized_chain_head_digest = block.parent_digest;
                    info!(
                        target: &self.name(),
                        "New finalized chain head {:?} from epoch {:?}",
                        String::from_utf8_lossy(&self.finalized_chain_head_digest),
                        block.creation_epoch - 1
                    );
                }
            }
            Entry::Vacant(_) => {
                error!(target: &self.name(), "Cannot finalize chain because {:?} does not exist", String::from_utf8_lossy(&block_digest))
            }
        };
    }

    fn adjust_longest_chain_head(&mut self, new_block: &Block, new_length: u32) {
        if !new_block.is_notarized || !self.blocks[&new_block.parent_digest].0.is_notarized {
            return;
        }
        let first_longest_chain_head = self.longest_notarized_chain_head_digests.first().unwrap();
        match self.blocks.entry(*first_longest_chain_head) {
            Entry::Occupied(entry) => {
                let (_head, curr_longest_length) = entry.into_mut();
                if *curr_longest_length < new_length {
                    self.longest_notarized_chain_head_digests = vec![new_block.digest()];
                } else if *curr_longest_length == new_length {
                    self.longest_notarized_chain_head_digests
                        .push(new_block.digest());
                }
                info!(
                    target: &self.name(),
                    "New notarized chain head {:?}",
                    new_block.creation_epoch
                );
            }
            Entry::Vacant(_) => {
                error!(
                    target: &self.name(),
                    "Block with {:?} not exists - cannot set the longest chain head",
                    String::from_utf8_lossy(first_longest_chain_head)
                )
            }
        };
    }
}

impl BaseNode for HonestNode {
    fn on_message_received(&mut self, message: &Message, message_queue: &mut VecDeque<Message>) {
        if message.from == self.id {
            trace!(target: &self.name(), "Ignoring echoed message - it was sent by this node previously");
            return;
        }
        let mut processed_the_unique_message = false;
        match message.content {
            MessageContent::Proposal(ProposalMessage { block }) => {
                if !self.blocks.contains_key(&block.digest()) {
                    trace!(
                        target: &self.name(),
                        "Received a proposal {:?} from {:?}",
                        String::from_utf8_lossy(&block.digest()),
                        message.from
                    );

                    let (_, length) = self.blocks[&block.parent_digest];
                    self.blocks
                        .insert(block.digest(), (block.clone(), length + 1));

                    if self
                        .longest_notarized_chain_head_digests
                        .contains(&block.parent_digest)
                    {
                        message_queue.push_back(VoteMessage::new(
                            self.id,
                            self.current_leader,
                            block.digest(),
                        ))
                    }
                    processed_the_unique_message = true;
                }
            }
            MessageContent::Vote(VoteMessage { block_digest }) => {
                trace!(
                    target: &self.name(),
                    "Received a vote for {:?} from {:?}",
                    String::from_utf8_lossy(&block_digest),
                    message.from
                );
                let votes_for_block = self
                    .votes
                    .entry(block_digest)
                    .or_insert(Vote(0, HashSet::new()));

                if !votes_for_block.has_voted(message.from) {
                    votes_for_block.increment();
                    votes_for_block.add_voter(message.from);
                    processed_the_unique_message = true;
                }
            }
        }
        if processed_the_unique_message {
            // Echo the message to other nodes; we broadcast after processing to prevent looping in the mock network implementation
            self.broadcast(message, message_queue);
        }
    }

    fn at_time(&mut self, epoch: Epoch, message_queue: &mut VecDeque<Message>) {
        for (block_digest, votes_for_block) in self.votes.clone() {
            if self.can_block_be_notarized(votes_for_block.get_votes()) {
                self.notarize_block(block_digest);
                self.try_finalize_chain(block_digest);
            }
            let (block, length) = self.blocks[&block_digest];
            self.adjust_longest_chain_head(&block, length);
        }
        self.current_leader = self.calculate_leader_id(epoch);

        if self.is_leader(epoch) {
            debug!(target: &self.name(), "Is leader in {:?} epoch", epoch);
            let first_longest_chain_head_digest =
                self.longest_notarized_chain_head_digests.first().unwrap();
            let (longest_chain_head, length) = self.blocks[first_longest_chain_head_digest];

            let block = Block::new(epoch, longest_chain_head.digest());
            self.broadcast(
                &ProposalMessage::new(self.id, self.id, block.clone()),
                message_queue,
            );
            self.blocks.insert(block.digest(), (block, length + 1));
        }
        trace!(
            target: &self.name(),
            "Blockchain {:?}",
            self.blocks.iter()
                .map(|(_hash, (block, _length))| (block.creation_epoch, block.is_notarized, self.blocks.get(&block.parent_digest).map(|(b, _l)| b.creation_epoch)))
                .collect::<Vec<(Epoch, bool, Option<Epoch>)>>()
        );
    }
}
