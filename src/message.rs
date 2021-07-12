use ring::digest::Digest;

use crate::{block::Block, node::NodeIdentifier};

pub struct ProposalMessage {
    pub block: Block,
}

pub struct VoteMessage {
    pub block_digest: Digest,
}

pub enum MessageContent {
    Proposal(ProposalMessage),
    Vote(VoteMessage),
}
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
    pub fn new(from: NodeIdentifier, to: NodeIdentifier, block_digest: Digest) -> Message {
        Message {
            from,
            to,
            content: MessageContent::Vote(Self { block_digest }),
        }
    }
}
