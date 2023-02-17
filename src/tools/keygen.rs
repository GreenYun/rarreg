use num_bigint::BigUint;
use sha1::{Digest, Sha1};

use super::{
    ec::{Point, G},
    sha1::sha1 as new_hash,
};

#[must_use]
pub fn gen_priv_key(msg: String) -> Vec<u8> {
    let mut src = vec![0u8; 4];

    if msg.is_empty() {
        src.extend(vec![
            0x81u8, 0xb7, 0x3e, 0xeb, 0x29, 0x53, 0x26, 0x50, 0xa3, 0xf4, 0x5e, 0xdc, 0xd5, 0xb9, 0x47, 0x68, 0x4c,
            0x3b, 0xe4, 0xcd,
        ]);
    } else {
        src.extend(new_hash(&msg.into_bytes()));
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

#[must_use]
pub fn calc_pub_key(priv_key: &[u8]) -> Point {
    let priv_key = BigUint::from_bytes_le(priv_key);

    G * priv_key
}

#[must_use]
pub fn compress_pub_key(p: Point) -> BigUint {
    let x = p.get_x();
    let y = p.get_y();
    let quotient = y / x;
    let quotient: BigUint = quotient.into();
    let last_bit = u64::from(quotient.bit(0));

    let mut x = x.into();
    x <<= 1;
    x += BigUint::from(last_bit);

    x
}
