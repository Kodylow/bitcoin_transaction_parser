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

    // let tx_string = "0100000001463c9cf77d401698ba1353d182215a3b1e5d4d2a83596547a8e3a5b29bca0264030000006a473044022071d398d694e0dccb7ca97fb54578d37635e1f2e3b8f9d7d219822111c256717502201f029e89d51dc4b10aebf9ef3519da3c6b5da323196a466395ac33220b8cf52801210355459a64f40fdee501f55f3c4b778caf1d4c9d26fe8d3873df4d6e463495f4aefdffffff040000000000000000536a4c5058325b7ec4787de66c09e7b399998c502665e8a16ad1647c6fa489aa13ffce12c175ecdc2434d31343840eff0b54ac85efd398b2830781297a2f1b5943332f88d50f06000b2ac30069000b1e2e00ec28007102000000000017a914459556788403d60ecdd554e5b484761fce8b45b087007102000000000017a914352481ec2fecfde0c5cdc635a383c4ac27b9f71e8756c54002000000001976a9144397615609ebb13c36847a6ca710b386ccfd4e1a88ac00000000".to_string();

    let parsed_tx = BitcoinTX::new(tx_string);

    println!("\n{:?}\n", parsed_tx);
}
