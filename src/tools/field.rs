use core::{
    convert::TryInto,
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};
use std::ops::{BitXor, BitXorAssign, Deref};

use arrayvec::ArrayVec;
use num_bigint::BigUint;

use super::table::TABLE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) struct GaloisField2p15(pub u16);

/*
impl GaloisField2p15 {
    // Only use for generate the table.
    // However we hardcoded that.
    const POLY : u16 = 0x8003;
}
*/

impl Add for GaloisField2p15 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self ^ rhs
    }
}

impl AddAssign for GaloisField2p15 {
    fn add_assign(&mut self, rhs: Self) {
        *self ^= rhs;
    }
}

impl BitXor for GaloisField2p15 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(*self ^ *rhs)
    }
}

impl BitXorAssign for GaloisField2p15 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= *rhs;
    }
}

impl Deref for GaloisField2p15 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul for GaloisField2p15 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if *self == 0 || *rhs == 0 {
            return Self(0);
        }

        if *self == 1 {
            return rhs;
        }

        if *rhs == 1 {
            return self;
        }

        let mut g = TABLE.log[*self as usize] + TABLE.log[*rhs as usize];
        if g >= 0x7fff {
            g -= 0x7fff;
        }

        Self(TABLE.exp[g as usize])
    }
}

impl MulAssign for GaloisField2p15 {
    fn mul_assign(&mut self, rhs: Self) {
        if self.0 == 0 || *rhs == 1 {
            return;
        }

        if *rhs == 0 {
            *self = Self(0);
            return;
        }

        if self.0 == 1 {
            *self = rhs;
            return;
        }

        let mut g = TABLE.log[self.0 as usize] + TABLE.log[*rhs as usize];
        if g >= 0x7fff {
            g -= 0x7fff;
        }

        *self = Self(TABLE.exp[g as usize]);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GaloisField2p15p17 {
    pub(super) num: [GaloisField2p15; 17],
}

impl GaloisField2p15p17 {
    const POLY: [GaloisField2p15; 18] = gf_2_15_const_arr![1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

    #[must_use]
    pub fn new(num: [u16; 17]) -> Self {
        let num = {
            use std::mem::{transmute, MaybeUninit};

            let mut arr: [MaybeUninit<GaloisField2p15>; 17] = unsafe { MaybeUninit::uninit().assume_init() };

            for (i, &n) in num.iter().enumerate() {
                arr[i] = MaybeUninit::new(GaloisField2p15(n));
            }

            unsafe { transmute(arr) }
        };

        Self { num }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self {
            num: [GaloisField2p15(0); 17],
        }
    }

    #[must_use]
    pub fn inverse(self) -> Self {
        let mut ret = Self {
            num: [GaloisField2p15(0); 17],
        };
        ret.num[0].0 = 1;

        let mut tmp = self;
        for _ in 1..15 * 17 {
            tmp *= tmp;
            ret *= tmp;
        }

        ret
    }
}

impl From<GaloisField2p15p17> for BigUint {
    fn from(g: GaloisField2p15p17) -> Self {
        g.num
            .iter()
            .map(|&n| *n)
            .enumerate()
            .fold(Self::from(0u8), |u, (i, n)| u | Self::from(n) << (i * 15))
    }
}

impl Add for GaloisField2p15p17 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: self
                .num
                .iter()
                .zip(rhs.num)
                .map(|(&l, r)| l + r)
                .collect::<ArrayVec<_, 17>>()
                .into_inner()
                .unwrap(),
        }
    }
}

impl AddAssign for GaloisField2p15p17 {
    fn add_assign(&mut self, rhs: Self) {
        self.num.iter_mut().zip(rhs.num).for_each(|(l, r)| *l += r);
    }
}

impl Mul for GaloisField2p15p17 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut tmp = [GaloisField2p15(0); 33];

        self.num
            .iter()
            .enumerate()
            .for_each(|(i, &l)| rhs.num.iter().enumerate().for_each(|(j, &r)| tmp[i + j] += l * r));

        for i in (17..33).rev() {
            if *(tmp[i]) != 0 {
                for j in 0..17 {
                    tmp[i - 17 + j] += tmp[i] * Self::POLY[j];
                }
            }
        }

        let ret: ArrayVec<GaloisField2p15, 17> = tmp[..17].try_into().unwrap();
        Self {
            num: ret.into_inner().unwrap(),
        }
    }
}

impl MulAssign for GaloisField2p15p17 {
    fn mul_assign(&mut self, rhs: Self) {
        let mut tmp = [GaloisField2p15(0); 33];

        self.num
            .iter()
            .enumerate()
            .for_each(|(i, &l)| rhs.num.iter().enumerate().for_each(|(j, &r)| tmp[i + j] += l * r));

        for i in (17..33).rev() {
            if *(tmp[i]) != 0 {
                for j in 0..17 {
                    tmp[i - 17 + j] += tmp[i] * Self::POLY[j];
                }
            }
        }

        let ret: ArrayVec<GaloisField2p15, 17> = tmp[..17].try_into().unwrap();
        self.num = ret.into_inner().unwrap();
    }
}

impl Div for GaloisField2p15p17 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

#[cfg(test)]
mod field_test {
    use super::*;

    #[test]
    fn mul_test() {
        let a = GaloisField2p15p17::new([
            0x4CAB, 0x7F00, 0x409B, 0x784F, 0x6105, 0x2D19, 0x4699, 0x4D0F, 0x5420, 0x5625, 0x7342, 0x2D0D, 0x1DCE,
            0x1052, 0x3450, 0x0595, 0x6CCD,
        ]);
        let b = GaloisField2p15p17::new([
            0x5AA7, 0x7315, 0x2132, 0x12D1, 0x1DD3, 0x1540, 0x71BF, 0x42C6, 0x3BAE, 0x3F1C, 0x3A14, 0x2619, 0x63E7,
            0x6936, 0x2919, 0x76ED, 0x1D9B,
        ]);
        let c = GaloisField2p15p17::new([
            0x65C2, 0x677A, 0x0931, 0x5067, 0x1FDA, 0x0E0C, 0x1801, 0x779D, 0x0918, 0x6F18, 0x3A36, 0x688F, 0x78EF,
            0x17E0, 0x27C7, 0x7F67, 0x10DB,
        ]);

        assert_eq!(a * b, c);

        let mut a = a;
        a *= b;
        assert_eq!(a, c);
    }

    #[test]
    fn inverse_test() {
        let a = GaloisField2p15p17::new([
            0x7CEC, 0x3646, 0x2C52, 0x1363, 0x7A87, 0x4666, 0x083E, 0x2B1E, 0x088B, 0x5142, 0x1D95, 0x0BAD, 0x31A0,
            0x6EB6, 0x2116, 0x5818, 0x0147,
        ]);
        let b = GaloisField2p15p17::new([
            0x5AA7, 0x7315, 0x2132, 0x12D1, 0x1DD3, 0x1540, 0x71BF, 0x42C6, 0x3BAE, 0x3F1C, 0x3A14, 0x2619, 0x63E7,
            0x6936, 0x2919, 0x76ED, 0x1D9B,
        ]);

        assert_eq!(a.inverse(), b);
    }
}
