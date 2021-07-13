use crate::{block::{Block}, node::NodeIdentifier, utils::BlockHash};


#[derive(Copy, Clone, Debug)]
pub struct ProposalMessage {
    pub block: Block,
}


#[derive(Copy, Clone, Debug)]
pub struct VoteMessage {
    pub block_digest: BlockHash,
}

#[derive(Copy, Clone, Debug)]
pub enum MessageContent {
    Proposal(ProposalMessage),
    Vote(VoteMessage),
}

#[derive(Copy, Clone, Debug)]
pub struct Message {
    pub from: NodeIdentifier,
    pub to: NodeIdentifier,
    pub content: MessageContent,
}

impl<'a> ProposalMessage {
    pub fn new(from: NodeIdentifier, to: NodeIdentifier, block: Block) -> Message {
        Message {
            from,
            to,
            content: MessageContent::Proposal(Self { block }),
        }
    }
}

impl VoteMessage {
    pub fn new(from: NodeIdentifier, to: NodeIdentifier, block_digest: BlockHash) -> Message {
        Message {
            from,
            to,
            content: MessageContent::Vote(Self { block_digest }),
        }
    }
}
