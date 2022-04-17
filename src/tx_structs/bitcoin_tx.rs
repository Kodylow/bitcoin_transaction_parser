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

        let version = u32::from_le_bytes(tx_bytes.drain(..4).as_slice()[0..4].try_into().unwrap());

        let input_count = parsers::compact_field(&mut tx_bytes);

        let inputs = parsers::tx_inputs(input_count, &mut tx_bytes);

        let output_count = parsers::compact_field(&mut tx_bytes);

        let outputs = parsers::tx_outputs(output_count, &mut tx_bytes);

        let locktime = u32::from_le_bytes(tx_bytes.drain(..4).as_slice()[0..4].try_into().unwrap());

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
