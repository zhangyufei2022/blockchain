use core::blockchain;

fn main() {
    println!("Hello, world!");
    let mut blockchain = blockchain::BlockChain::new();

    blockchain.add_block(String::from("a->b: 1"));
    blockchain.add_block(String::from("a->c: 2"));
    print_bc(&blockchain);
}

fn print_bc(blockchain: &blockchain::BlockChain) {
    for block in &blockchain.blocks {
        println!("{:#?}", block);
    }
}
