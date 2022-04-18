use crate::tx_structs::{tx_input::TXInput, tx_output::TXOutput};

pub fn compact_field(byte_vec: &mut Vec<u8>) -> usize {
    let size: usize = usize::try_from(byte_vec[0]).unwrap();
    byte_vec.drain(..1);
    if size <= 252 {
        size
    } else {
        let actual_size_bytes = usize::pow(2, u32::try_from(size - 252).unwrap());
        let size: Vec<u8> = byte_vec.drain(..actual_size_bytes).collect();
        usize::from_str_radix(&hex::encode(size), 16).unwrap()
    }
}

pub fn tx_inputs(input_count: usize, byte_vec: &mut Vec<u8>) -> Vec<TXInput> {
    let mut inputs: Vec<TXInput> = Vec::new();

    let mut i = 0;

    while i < input_count {
        let mut txid: Vec<u8> = byte_vec.drain(..32).collect();
        txid.reverse();

        let vout: u32 =
            u32::from_le_bytes(byte_vec.drain(..4).as_slice()[0..4].try_into().unwrap());

        let script_sig_size = compact_field(byte_vec);

        let script_sig: Vec<u8> = byte_vec.drain(..script_sig_size).collect();

        let sequence: Vec<u8> = byte_vec.drain(..4).collect();

        let inp = TXInput {
            txid: hex::encode(txid),
            vout,
            script_sig_size,
            script_sig: hex::encode(script_sig),
            sequence: hex::encode(sequence),
        };

        i += 1;
        inputs.push(inp);
    }

    inputs
}

pub fn tx_outputs(output_count: usize, byte_vec: &mut Vec<u8>) -> Vec<TXOutput> {
    let mut outputs: Vec<TXOutput> = Vec::new();

    let mut i: usize = 0;

    while i < output_count {
        let amount: u64 =
            u64::from_le_bytes(byte_vec.drain(..8).as_slice()[0..8].try_into().unwrap());

        let script_pubkey_size = compact_field(byte_vec);

        let script_pubkey: Vec<u8> = byte_vec.drain(..script_pubkey_size).collect();

        let out = TXOutput {
            amount,
            script_pubkey_size,
            script_pubkey: hex::encode(script_pubkey),
        };

        outputs.push(out);
        i += 1;
    }

    outputs
}
