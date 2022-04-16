use crate::tx_structs::{parsers, tx_input::TXInput, tx_output::TXOutput};

#[derive(Debug)]

pub struct BitcoinTX {
    pub version: u32,
    pub input_count: usize,
    pub inputs: Vec<TXInput>,
    pub output_count: usize,
    pub outputs: Vec<TXOutput>,
    pub locktime: u32,
}

impl BitcoinTX {
    pub fn new(hex_string: String) -> Self {
        let mut tx_bytes = hex::decode(hex_string).unwrap();

        let mut version: [u8; 4] = [0; 4];
        version.clone_from_slice(tx_bytes.drain(..4).as_slice());
        let version = u32::from_le_bytes(version[..].try_into().unwrap());

        let (input_count, compact_field_len) = parsers::compact_field(tx_bytes.to_vec());
        tx_bytes.drain(..compact_field_len);

        let (input_bytes_count, inputs) = parsers::tx_inputs(input_count, tx_bytes.to_vec());
        tx_bytes.drain(..input_bytes_count);

        let (output_count, compact_field_len) = parsers::compact_field(tx_bytes.to_vec());
        tx_bytes.drain(..compact_field_len);

        let (output_bytes_count, outputs) = parsers::tx_outputs(output_count, tx_bytes.to_vec());
        tx_bytes.drain(..output_bytes_count);

        let mut locktime: [u8; 4] = [0; 4];
        locktime.clone_from_slice(tx_bytes.drain(..4).as_slice());
        let locktime = u32::from_le_bytes(locktime[..].try_into().unwrap());

        Self {
            version,
            input_count,
            inputs,
            output_count,
            outputs,
            locktime,
        }
    }
}
