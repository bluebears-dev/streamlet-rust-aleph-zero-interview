use crate::{digest::{BlockHash, sha512_digest}, node::Epoch};

pub const INIT_EPOCH: Epoch = 0;

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub parent_digest: BlockHash,
    pub transaction: BlockHash,
    pub creation_epoch: Epoch,

    pub is_notarized: bool,
}

impl Block {
    pub fn new(epoch: Epoch, parent_digest: BlockHash) -> Self {
        Self {
            parent_digest,
            transaction: sha512_digest(&epoch.to_be_bytes()),
            creation_epoch: epoch,
            is_notarized: false,
        }
    }

    pub fn genesis_block() -> Self {
        let mut block = Self::new(INIT_EPOCH, sha512_digest(&INIT_EPOCH.to_be_bytes()));
        block.notarize();
        block
    }

    pub fn notarize(&mut self) {
        self.is_notarized = true;
    }

    pub fn digest(&self) -> BlockHash {
        let mut bytes = self.parent_digest.to_vec();
        bytes.append(&mut self.transaction.to_vec());
        bytes.append(&mut self.creation_epoch.to_be_bytes().to_vec());
        sha512_digest(&bytes)
    }
}
