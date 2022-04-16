use hex;
use vecshard::{ShardExt, VecShard};

fn main() {
    let remaining_tx_bytes = hex::decode("0100000001e11af7c4292505f99a4a5f4ff0818ac86c197bb16261f91af3f5cac661259c88000000006a473044022045c7199ffc8069a498135b7bb2678da16e8b5d49455b4a7ace755928c9339c7a022051cbf72024cf273444640f7b993b2bf3d329124b03e6744edaed5158a30e29b8012103fd9bc1e9803e739720e0f1c63e580a94656c7d0cab6cd083f0c0dfb221b90662ffffffff0200b080f6450100001976a9143b9552116adcc2fbd74fad44a4da603a727c816e88aca05ecf1c000100001976a914f90ce447f14847e841d4d2ecc76299b5bc77166188ac00000000").unwrap();

    let (version, remaining_tx_bytes): (VecShard<u8>, VecShard<u8>) =
        remaining_tx_bytes.split_inplace_at(4);

    let version = u32::from_le_bytes(version[..].try_into().unwrap());

    let (input_count, compact_field_len) = parse_compact_field(remaining_tx_bytes.to_vec());

    let (_, remaining_tx_bytes): (VecShard<u8>, VecShard<u8>) =
        remaining_tx_bytes.split_inplace_at(usize::try_from(compact_field_len).unwrap());

    let (input_bytes_count, inputs) = parse_inputs(input_count, remaining_tx_bytes.to_vec());

    let (input_bytes, remaining_tx_bytes) = remaining_tx_bytes.split_inplace_at(input_bytes_count);

    let (output_count, compact_field_len) = parse_compact_field(remaining_tx_bytes.to_vec());

    let (_, remaining_tx_bytes): (VecShard<u8>, VecShard<u8>) =
        remaining_tx_bytes.split_inplace_at(usize::try_from(compact_field_len).unwrap());

    let (output_bytes_count, outputs) = parse_outputs(output_count, remaining_tx_bytes.to_vec());

    let (output_bytes, remaining_tx_bytes) =
        remaining_tx_bytes.split_inplace_at(output_bytes_count);

    let (locktime, remaining_tx_bytes) = remaining_tx_bytes.split_inplace_at(4);

    println!("Parsed TX: \n");

    println!("Version: {:?}", version);

    println!("Input Count: {:?}", input_count);

    println!("Inputs: {:?}", inputs);

    println!("Output Count: {:?}", output_count);

    println!("Outputs: {:?}", outputs);

    println!("Locktime: {:?}", hex::encode(locktime.to_vec()));

    println!("Bytes left: {:?}", hex::encode(remaining_tx_bytes.to_vec()));
}

fn parse_compact_field(byte_vec: Vec<u8>) -> (u32, u32) {
    let (size, byte_vec): (VecShard<u8>, VecShard<u8>) = byte_vec.split_inplace_at(1);

    let size: u8 = u8::from(size[0]);

    if size <= 252 {
        (u32::from(size), 1)
    } else {
        let len_actual_size_bytes = u32::from(size - 252);
        let actual_size_bytes = usize::pow(2, len_actual_size_bytes);
        let size = u32::from_be_bytes(byte_vec[1..actual_size_bytes + 1].try_into().unwrap());
        (size, len_actual_size_bytes + 1)
    }
}

fn parse_inputs(input_count: u32, byte_vec: Vec<u8>) -> (usize, Vec<TXInput>) {
    let mut inputs: Vec<TXInput> = Vec::new();

    let mut input_bytes_offset: usize = 0;

    let mut i = 0;

    while i < input_count {
        let loc_byte_vec = byte_vec[input_bytes_offset..].to_vec();

        let (txid, byte_vec) = loc_byte_vec.split_inplace_at(32);
        let mut txid = txid.to_vec();
        txid.reverse();
        let txid: String = hex::encode(txid);

        input_bytes_offset += 32;

        let (vout, loc_byte_vec) = byte_vec.split_inplace_at(4);
        let vout: u32 = u32::from_le_bytes(vout[..].try_into().unwrap());
        input_bytes_offset += 4;

        let (scriptSig_size, compact_field_len) = parse_compact_field(loc_byte_vec.to_vec());

        let (_, loc_byte_vec): (VecShard<u8>, VecShard<u8>) =
            loc_byte_vec.split_inplace_at(usize::try_from(compact_field_len).unwrap());
        input_bytes_offset += usize::try_from(scriptSig_size + compact_field_len).unwrap();

        let (scriptSig, loc_byte_vec) =
            loc_byte_vec.split_inplace_at(usize::try_from(scriptSig_size).unwrap());
        let scriptSig: String = hex::encode(scriptSig.to_vec());

        let (sequence, _) = loc_byte_vec.split_inplace_at(4);
        let sequence = hex::encode(sequence.to_vec());
        input_bytes_offset += 4;

        let inp = TXInput {
            txid,
            vout,
            scriptSig_size,
            scriptSig,
            sequence,
        };

        i += 1;
        inputs.push(inp);
    }

    (input_bytes_offset, inputs)
}

fn parse_outputs(output_count: u32, byte_vec: Vec<u8>) -> (usize, Vec<TXOutput>) {
    let mut outputs: Vec<TXOutput> = Vec::new();

    let mut output_bytes_offset: usize = 0;

    let mut i = 0;

    while i < output_count {
        let loc_byte_vec = byte_vec[output_bytes_offset..].to_vec();

        let (amount, loc_byte_vec) = loc_byte_vec.split_inplace_at(8);
        let mut amount = amount.to_vec();
        let amount: u64 = u64::from_le_bytes(amount[..].try_into().unwrap());

        output_bytes_offset += 8;

        let (scriptPubKey_size, compact_field_len) = parse_compact_field(loc_byte_vec.to_vec());

        let (_, loc_byte_vec): (VecShard<u8>, VecShard<u8>) =
            loc_byte_vec.split_inplace_at(usize::try_from(compact_field_len).unwrap());
        output_bytes_offset += usize::try_from(scriptPubKey_size + compact_field_len).unwrap();

        let (scriptPubKey, loc_byte_vec) =
            loc_byte_vec.split_inplace_at(usize::try_from(scriptPubKey_size).unwrap());
        let scriptPubKey: String = hex::encode(scriptPubKey.to_vec());

        let out = TXOutput {
            amount,
            scriptPubKey_size,
            scriptPubKey,
        };
        outputs.push(out);
        i += 1;
    }

    (output_bytes_offset, outputs)
}

// struct Bitcoin_TX {
//     version: u32,
//     inputs: Vec<TX_Input>,
//     outputs: Vec<TX_Output>,
//     locktime: u32,
// }

#[derive(Debug)]
struct TXInput {
    txid: String,
    vout: u32,
    scriptSig_size: u32,
    scriptSig: String,
    sequence: String,
}

#[derive(Debug)]
struct TXOutput {
    amount: u64,
    scriptPubKey_size: u32,
    scriptPubKey: String,
}
