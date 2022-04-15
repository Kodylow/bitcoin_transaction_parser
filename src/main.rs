use hex;
use vecshard::{ShardExt, VecShard};

fn main() {
    
    let remaining_tx_bytes = hex::decode("0100000001e11af7c4292505f99a4a5f4ff0818ac86c197bb16261f91af3f5cac661259c88000000006a473044022045c7199ffc8069a498135b7bb2678da16e8b5d49455b4a7ace755928c9339c7a022051cbf72024cf273444640f7b993b2bf3d329124b03e6744edaed5158a30e29b8012103fd9bc1e9803e739720e0f1c63e580a94656c7d0cab6cd083f0c0dfb221b90662ffffffff0200b080f6450100001976a9143b9552116adcc2fbd74fad44a4da603a727c816e88aca05ecf1c000100001976a914f90ce447f14847e841d4d2ecc76299b5bc77166188ac00000000").unwrap();

    let (version, remaining_tx_bytes): (VecShard<u8>, VecShard<u8>) = remaining_tx_bytes.split_inplace_at(4);

    let version = u32::from_be_bytes(version[..].try_into().unwrap());

    let (input_count, compact_field_len) = parse_compact_field(remaining_tx_bytes.to_vec());

    let (_, remaining_tx_bytes): (VecShard<u8>, VecShard<u8>) = remaining_tx_bytes.split_inplace_at(usize::try_from(compact_field_len).unwrap());

    let remaining_tx_bytes: Vec<u8> = remaining_tx_bytes.into();
    

    println!("Version: {:?}", version);

    println!("Input Count: {:?}", input_count);

    println!("Input Count Len: {:?}", compact_field_len);

    println!("Bytes left: {:?}", hex::encode(remaining_tx_bytes));
}

fn parse_compact_field(byte_vec: Vec<u8>) -> (u64, u32) {

    let (size, byte_vec): (VecShard<u8>, VecShard<u8>) = byte_vec.split_inplace_at(1);

    let size:u8 = u8::from(size[0]);
    
    if size <= 252 {
        (u64::from(size), 1)
    } else {
        let len_actual_size_bytes = u32::from(size - 252);
        let actual_size_bytes = usize::pow(2, len_actual_size_bytes);
        let size = u64::from_be_bytes(byte_vec[1..actual_size_bytes+1].try_into().unwrap());
        (size, len_actual_size_bytes + 1)
    }


}

// struct Bitcoin_TX {
//     version: u32,
//     inputs: Vec<TX_Input>,
//     outputs: Vec<TX_Output>,
//     locktime: u32,
// }

// struct TX_Input {
//     txid: String,
//     vout: u32,
//     scriptSig: Vec<u8>,
//     sequence: u32,
// }

// struct TX_Output {
//     amount: u64,
//     scriptPubKey: Vec<u8>
// }