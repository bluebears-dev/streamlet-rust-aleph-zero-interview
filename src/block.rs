use ring::digest::{Context, Digest, SHA512};

use crate::node::Epoch;

const INIT_EPOCH: Epoch = 0;
pub type BlockHash = [u8; 64];

fn sha512_digest(data: &[u8]) -> BlockHash {
    let mut context = Context::new(&SHA512);
    context.update(data);
    digest_to_string(context.finish())
}

#[derive(Copy, Clone)]
pub struct Block {
    pub parent_digest: BlockHash,
    pub transaction: BlockHash,
    pub creation_epoch: Epoch,

    pub is_notarized: bool,
}

pub fn digest_to_string(hash: Digest) -> BlockHash {
    let mut str = String::from("");
    for i in hash.as_ref() {
        str.push(*i as char)
    }
    let mut res: [u8; 64] = [0; 64];
    for i in 0..64 {
        res[i] = str.chars().nth(i).unwrap() as u8;
    } 
    res
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
