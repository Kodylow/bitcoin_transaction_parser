use crate::tx_structs::{tx_input::TXInput, tx_output::TXOutput};

pub fn compact_field(byte_vec: &mut Vec<u8>) -> usize {
    let size: usize = usize::try_from(byte_vec[0]).unwrap();
    byte_vec.drain(..1);

    if size <= 252 {
        size
    } else {
        let len_actual_size_bytes = size - 252;
        let actual_size_bytes = usize::pow(2, u32::try_from(len_actual_size_bytes).unwrap());
        let size: Vec<u8> = byte_vec.drain(..actual_size_bytes).collect();
        usize::from_le_bytes(size.as_slice().try_into().unwrap())
    }
}

pub fn tx_inputs(input_count: usize, byte_vec: &mut Vec<u8>) -> Vec<TXInput> {
    let mut inputs: Vec<TXInput> = Vec::new();

    let mut i = 0;

    while i < input_count {
        let mut txid: Vec<u8> = byte_vec.drain(..32).collect();
        txid.reverse();
        let txid: String = hex::encode(txid).clone();

        let vout: u32 =
            u32::from_le_bytes(byte_vec.drain(..4).as_slice()[0..4].try_into().unwrap());

        let script_sig_size = compact_field(byte_vec);

        let script_sig: Vec<u8> = byte_vec.drain(..script_sig_size).collect();
        let script_sig: String = hex::encode(script_sig);

        let mut sequence: Vec<u8> = byte_vec.drain(..4).collect();
        sequence.reverse();
        let sequence: String = hex::encode(sequence);

        let inp = TXInput {
            txid,
            vout,
            script_sig_size,
            script_sig,
            sequence,
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
        let script_pubkey: String = hex::encode(script_pubkey);

        let out = TXOutput {
            amount,
            script_pubkey_size,
            script_pubkey,
        };

        outputs.push(out);
        i += 1;
    }

    outputs
}
