#[derive(Debug)]
pub struct TXInput {
    pub txid: String,
    pub vout: u32,
    pub script_sig_size: usize,
    pub script_sig: String,
    pub sequence: String,
}
