use std::collections::HashMap;


fn main() {
let mut genesis = HashMap::new();


genesis.insert("block", "genesis");
genesis.insert("height", "0");
genesis.insert("hash", "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
genesis.insert("merkleroot", "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
genesis.insert("time", "1231006505");
genesis.insert("bits", "1d00ffff");
genesis.insert("difficulty", "1");
genesis.insert("nonce", "2083236893");
genesis.insert("nTx", "1");


println!("{:?}", genesis);
