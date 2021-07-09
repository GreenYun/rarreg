use std::convert::TryInto;

use sha1::{Digest, Sha1};

pub fn sha1(msg: Vec<u8>) -> Vec<u8> {
    let result = Sha1::digest(&msg);

    let mut ret: Vec<u8> = vec![];
    for i in 0..5 {
        let x = i32::from_be_bytes(result[i * 4..(i + 1) * 4].try_into().unwrap());
        ret.extend(x.to_le_bytes().iter());
    }

    ret
}
