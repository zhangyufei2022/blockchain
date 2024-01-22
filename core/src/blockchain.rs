use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = self.blocks.get(self.blocks.len() - 1).unwrap();
        // let pre_block = &self.blocks[self.blocks.len() - 1];
        let block = Block::new(data, pre_block.hash.clone());
        self.blocks.push(block);
    }

    pub fn new() -> Self {
        let genesis_block = Block::new(String::from("genesis block"), String::from(""));
        BlockChain {
            blocks: vec![genesis_block],
        }
    }
}
