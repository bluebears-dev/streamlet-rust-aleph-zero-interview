# Streamlet

This is a streamlet implementation in Rust with custom mock of the network.

## Module description

- `main.rs` is the entrypoint. Here we process the args and start the whole network (along with node creation).
- `node.rs` contains the trait for the nodes along with other useful definitions like `Vote`. There is only one node implementation base on this trait, it is called `HonestNode`. You can find it inside `node/honest_node.rs`.
- `block.rs` contains the block structure definition used in the node state to construct the blockchain.
- `message.rs` contains the enum `Message` and definitions of the messages (proposal and vote) passed between the nodes.
- `digest.rs` is a module that exports two functions related to hashing.