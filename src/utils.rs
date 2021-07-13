use ring::digest::{Context, Digest, SHA512};

pub type BlockHash = [u8; 64];

pub fn sha512_digest(data: &[u8]) -> BlockHash {
    let mut context = Context::new(&SHA512);
    context.update(data);
    digest_to_string(context.finish())
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