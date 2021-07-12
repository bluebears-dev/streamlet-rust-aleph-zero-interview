use crate::node::NodeIdentifier;

pub struct ProposalMessage {}

pub struct VoteMessage {}

pub enum MessageContent {
    Proposal(ProposalMessage),
    Vote(VoteMessage),
}
pub struct Message {
    pub from: NodeIdentifier,
    pub to: NodeIdentifier,
    pub content: MessageContent,
}

impl Message {
    pub fn new(from: NodeIdentifier, to: NodeIdentifier, content: MessageContent) -> Self {
        Self { from, to, content }
    }
}
