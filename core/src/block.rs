use chrono::Utc;
use serde::{Deserialize, Serialize};
use utils::coder::{get_hash, my_serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,  // merkle hash of transcations data
    pub pre_hash: String, // hash of previos block
}

#[derive(Debug)]
pub struct Block {
    pub head: BlockHeader,
    pub hash: String, // hash of block head
    pub data: String, // transcations data
}

impl BlockHeader {
    fn new(data: &str, pre_hash: String) -> Self {
        let value = my_serialize(data);
        let tx_hash = get_hash(&value);
        BlockHeader {
            time: Utc::now().timestamp(),
            tx_hash: tx_hash,
            pre_hash: pre_hash,
        }
    }
}

impl Block {
    pub fn new(data: String, pre_hash: String) -> Self {
        let head = BlockHeader::new(&data, pre_hash);
        let serialized = my_serialize(&head);
        let hash = get_hash(&serialized);
        Block { head, hash, data }
    }
}
