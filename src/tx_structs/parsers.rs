use crate::tx_structs::{tx_input::TXInput, tx_output::TXOutput};
use vecshard::{ShardExt, VecShard};

pub fn compact_field(byte_vec: Vec<u8>) -> (usize, usize) {
    let (size, byte_vec): (VecShard<u8>, VecShard<u8>) = byte_vec.split_inplace_at(1);

    let size: usize = usize::try_from(size[0]).unwrap();

    if size <= 252 {
        (size, 1)
    } else {
        let len_actual_size_bytes = size - 252;
        let actual_size_bytes = usize::pow(2, u32::try_from(len_actual_size_bytes).unwrap());
        let size = usize::from_be_bytes(byte_vec[1..actual_size_bytes + 1].try_into().unwrap());
        (size, len_actual_size_bytes + 1)
    }
}

pub fn tx_inputs(input_count: usize, byte_vec: Vec<u8>) -> (usize, Vec<TXInput>) {
    let mut inputs: Vec<TXInput> = Vec::new();

    let mut input_bytes_offset: usize = 0;

    let mut i = 0;

    while i < input_count {
        let loc_byte_vec = byte_vec.clone();

        let mut txid: [u8; 32] = [0; 32];
        txid.clone_from_slice(&loc_byte_vec[input_bytes_offset..(input_bytes_offset + 32)]);
        txid.reverse();
        let txid: String = hex::encode(txid);
        input_bytes_offset += 32;

        let mut vout: [u8; 4] = [0; 4];
        vout.clone_from_slice(&loc_byte_vec[input_bytes_offset..(input_bytes_offset + 4)]);
        let vout: u32 = u32::from_le_bytes(vout[..].try_into().unwrap());
        input_bytes_offset += 4;

        let (script_sig_size, compact_field_len) =
            compact_field(loc_byte_vec[input_bytes_offset..].to_vec());

        input_bytes_offset += compact_field_len;

        let script_sig: Vec<u8> =
            loc_byte_vec[input_bytes_offset..input_bytes_offset + script_sig_size].to_vec();
        let script_sig: String = hex::encode(script_sig);

        input_bytes_offset += script_sig_size;

        let mut sequence: [u8; 4] = [0; 4];
        sequence.clone_from_slice(&loc_byte_vec[input_bytes_offset..(input_bytes_offset + 4)]);
        let sequence: String = hex::encode(sequence);
        input_bytes_offset += 4;

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

    (input_bytes_offset, inputs)
}

pub fn tx_outputs(output_count: usize, byte_vec: Vec<u8>) -> (usize, Vec<TXOutput>) {
    let mut outputs: Vec<TXOutput> = Vec::new();

    let mut output_bytes_offset: usize = 0;

    let mut i: usize = 0;

    while i < output_count {
        let loc_byte_vec = byte_vec.clone();

        let mut amount: [u8; 8] = [0; 8];
        amount.clone_from_slice(&loc_byte_vec[output_bytes_offset..(output_bytes_offset + 8)]);
        let amount: u64 = u64::from_le_bytes(amount);
        output_bytes_offset += 8;

        let (script_pubkey_size, compact_field_len) =
            compact_field(loc_byte_vec[output_bytes_offset..].to_vec());

        output_bytes_offset += compact_field_len;

        let script_pubkey: Vec<u8> =
            loc_byte_vec[output_bytes_offset..output_bytes_offset + script_pubkey_size].to_vec();
        let script_pubkey: String = hex::encode(script_pubkey);

        output_bytes_offset += script_pubkey_size;

        let out = TXOutput {
            amount,
            script_pubkey_size,
            script_pubkey,
        };

        outputs.push(out);
        i += 1;
    }

    (output_bytes_offset, outputs)
}
