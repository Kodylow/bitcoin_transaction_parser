#[derive(Debug)]
pub struct TXOutput {
    pub amount: u64,
    pub script_pubkey_size: usize,
    pub script_pubkey: String,
}
