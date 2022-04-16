mod tx_structs;

use crate::tx_structs::bitcoin_tx::BitcoinTX;
use std::io::stdin;
fn main() {
    let mut tx_string = String::new();

    println!("Paste a raw bitoin transaction in hex to parse: \n");

    stdin()
        .read_line(&mut tx_string)
        .ok()
        .expect("Failed to read line");

    tx_string = tx_string.replace("\n", "");

    let parsed_tx = BitcoinTX::new(tx_string);

    println!("\n{:?}\n", parsed_tx);
}
