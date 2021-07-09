use num_bigint::BigUint;
use sha1::{Digest, Sha1};

use super::ec::{Point, G};
use super::sha1::sha1 as new_hash;

pub fn gen_priv_key(msg: String) -> Vec<u8> {
    let mut src = vec![0u8; 4];

    if !msg.is_empty() {
        src.extend(new_hash(msg.into_bytes()));
    } else {
        src.extend(vec![
            0x81u8, 0xb7, 0x3e, 0xeb, 0x29, 0x53, 0x26, 0x50, 0xa3, 0xf4, 0x5e, 0xdc, 0xd5, 0xb9,
            0x47, 0x68, 0x4c, 0x3b, 0xe4, 0xcd,
        ]);
    }

    let mut key: Vec<u8> = vec![];

    for _ in 0..15 {
        src[0] += 1;

        let mut hasher = Sha1::new();
        hasher.update(src.clone());
        let digest = hasher.finalize();

        key.push(digest[3]);
        key.push(digest[2]);
    }

    key
}

pub fn calc_pub_key(priv_key: Vec<u8>) -> Point {
    let priv_key = BigUint::from_bytes_le(&priv_key);

    G * priv_key
}

pub fn compress_pub_key(p: Point) -> BigUint {
    let x = p.get_x();
    let y = p.get_y();
    let quotient = y / x;
    let quotient = quotient.uint();
    let last_bit = if quotient.bit(0) { 1u64 } else { 0 };

    let mut x = x.uint();
    x <<= 1;
    x += BigUint::from(last_bit);

    x
}
