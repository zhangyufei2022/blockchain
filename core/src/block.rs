pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

pub struct Block {
    pub head: BlockHeader,
    pub hash: String,
    pub data: String,
}
