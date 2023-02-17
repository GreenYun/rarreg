use num_bigint::{BigInt, BigUint, RandBigInt};
use rand::thread_rng;

use super::{ec::G, sha1::sha1};

lazy_static::lazy_static! {
    pub static ref ORDER: BigUint = BigUint::from_bytes_be(&[
        0x00, 0x01, 0x02, 0x6d, 0xd8, 0x50, 0x81, 0xb8, 0x23, 0x14, 0x69, 0x1c, 0xed, 0x9b, 0xbe, 0xc3, 0x05, 0x47,
        0x84, 0x0e, 0x4b, 0xf7, 0x2d, 0x8b, 0x5e, 0x0d, 0x25, 0x84, 0x42, 0xbb, 0xcd, 0x31,
    ]);
}

#[must_use]
pub fn hash(msg: String) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];

    ret.extend(sha1(&msg.into_bytes()));

    ret.extend(vec![0x43, 0x8d, 0xfd, 0x0f, 0x7c, 0x3c, 0xe3, 0xb4, 0xd1, 0x1b]);

    ret
}

#[must_use]
pub fn sign(msg: String, priv_key: &[u8]) -> (BigUint, BigUint) {
    let priv_key = BigUint::from_bytes_le(priv_key);

    let h = hash(msg);
    let h = &BigUint::from_bytes_le(&h);

    let mut rng = thread_rng();

    loop {
        let n = rng.gen_biguint_range(&BigUint::from(1u8), &ORDER);

        let r: BigUint = (G * n.clone()).get_x().into();
        let r = (r + h) % ORDER.clone();

        if r == BigUint::from(0u8) || r.clone() + n.clone() == *ORDER {
            continue;
        }

        let mut s = (BigInt::from_biguint(num_bigint::Sign::Plus, n)
            - BigInt::from_biguint(num_bigint::Sign::Plus, priv_key.clone() * r.clone()))
            % BigInt::from_biguint(num_bigint::Sign::Plus, ORDER.to_owned());

        if s == BigInt::from(0) {
            continue;
        }

        if s < BigInt::from(0) {
            s += BigInt::from_biguint(num_bigint::Sign::Plus, ORDER.to_owned());
        }

        let s = s.to_biguint().unwrap();

        return (r, s);
    }
}
