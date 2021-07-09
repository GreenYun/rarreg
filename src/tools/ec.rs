use core::ops::{AddAssign, Mul, Neg};

use num_bigint::BigUint;

use super::field::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: GaloisField2p15p17,
    y: GaloisField2p15p17,
}

pub const G: Point = Point {
    x: GaloisField2p15p17 {
        num: gf_2_15_const_arr![
            0x38CC, 0x052F, 0x2510, 0x45AA, 0x1B89, 0x4468, 0x4882, 0x0D67, 0x4FEB, 0x55CE, 0x0025,
            0x4CB7, 0x0CC2, 0x59DC, 0x289E, 0x65E3, 0x56FD,
        ],
    },
    y: GaloisField2p15p17 {
        num: gf_2_15_const_arr![
            0x31A7, 0x65F2, 0x18C4, 0x3412, 0x7388, 0x54C1, 0x539B, 0x4A02, 0x4D07, 0x12D6, 0x7911,
            0x3B5E, 0x4F0E, 0x216F, 0x2BF2, 0x1974, 0x20DA,
        ],
    },
};

impl Point {
    // These are useless and ignored.
    /*
    const A: GaloisField2p15p17 = GaloisField2p15p17 {
        num: [GaloisField2p15 { num: 0 }; 17],
    };

    const B: GaloisField2p15p17 = GaloisField2p15p17 {
        num: gf_2_15_const_arr![161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    */

    pub fn new(x: GaloisField2p15p17, y: GaloisField2p15p17) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self {
            x: GaloisField2p15p17::zero(),
            y: GaloisField2p15p17::zero(),
        }
    }

    pub fn double_assign(&mut self) {
        let m = self.y / self.x + self.x;

        let x = m * m + m;

        // Since A = 0, this step has no effect
        // x += Self::A;

        let mut y = self.x + x;
        y *= m;
        y += x + self.y;

        *self = Self { x, y }
    }

    pub fn get_x(self) -> GaloisField2p15p17 {
        self.x
    }

    pub fn get_y(self) -> GaloisField2p15p17 {
        self.y
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point {
            x: self.x,
            y: self.y + self.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        if *self == Self::zero() {
            *self = rhs;
            return;
        }

        if self.x == rhs.x {
            self.double_assign();
            return;
        }

        let m = (self.y + rhs.y) / (self.x + rhs.x);

        let x = m * m + m + self.x + rhs.x;

        // Since A = 0, this step has no effect
        // x += Self::A;

        let mut y = self.x + x;
        y *= m;
        y += x + self.y;

        *self = Self { x, y };
    }
}

impl Mul<BigUint> for Point {
    type Output = Self;

    fn mul(self, rhs: BigUint) -> Self::Output {
        let mut ret = Self::zero();
        let mut tmp = self.clone();

        for i in 0..rhs.bits() {
            if rhs.bit(i) {
                ret += tmp;
            }

            tmp.double_assign();
        }

        ret
    }
}

#[cfg(test)]
mod ec_test {
    use super::*;

    #[test]
    fn add_test() {
        let mut a = Point::new(
            GaloisField2p15p17::new([
                0x655E, 0x4AEC, 0x5EC4, 0x280E, 0x2D9C, 0x018A, 0x6CE6, 0x6446, 0x4F96, 0x64E4,
                0x5A48, 0x36AB, 0x79E3, 0x7854, 0x785A, 0x6CB9, 0x5E76,
            ]),
            GaloisField2p15p17::new([
                0x5243, 0x4418, 0x0E66, 0x5BC2, 0x7474, 0x6F88, 0x18F1, 0x66FC, 0x70AC, 0x21D0,
                0x32C8, 0x3048, 0x3670, 0x5494, 0x645F, 0x2000, 0x02E0,
            ]),
        );
        let b = Point::new(
            GaloisField2p15p17::new([
                0x16C8, 0x6E22, 0x6C50, 0x4047, 0x21C2, 0x0076, 0x4E69, 0x0713, 0x5BB9, 0x1876,
                0x53E3, 0x4D6A, 0x4E1C, 0x3C95, 0x0849, 0x2C91, 0x4CB3,
            ]),
            GaloisField2p15p17::new([
                0x4079, 0x498C, 0x48CC, 0x2623, 0x4BAD, 0x5660, 0x6E3E, 0x5C7B, 0x1E5C, 0x5CD6,
                0x3C60, 0x42C7, 0x0285, 0x3C96, 0x282C, 0x291A, 0x3D9B,
            ]),
        );
        let c = Point::new(
            GaloisField2p15p17::new([
                0x20BE, 0x22A6, 0x3969, 0x7EF6, 0x72E3, 0x2EB7, 0x59B0, 0x254D, 0x3E78, 0x43C1,
                0x6209, 0x0D70, 0x1BA0, 0x1BB0, 0x765E, 0x094F, 0x2A88,
            ]),
            GaloisField2p15p17::new([
                0x19B7, 0x235B, 0x1E4D, 0x33A4, 0x07A3, 0x73E5, 0x793A, 0x698E, 0x4E2E, 0x4966,
                0x3FF9, 0x4004, 0x0AAF, 0x1692, 0x7900, 0x27B5, 0x16EA,
            ]),
        );

        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn mul_test() {
        let a = Point::new(
            GaloisField2p15p17::new([
                0x38CC, 0x052F, 0x2510, 0x45AA, 0x1B89, 0x4468, 0x4882, 0x0D67, 0x4FEB, 0x55CE,
                0x0025, 0x4CB7, 0x0CC2, 0x59DC, 0x289E, 0x65E3, 0x56FD,
            ]),
            GaloisField2p15p17::new([
                0x31A7, 0x65F2, 0x18C4, 0x3412, 0x7388, 0x54C1, 0x539B, 0x4A02, 0x4D07, 0x12D6,
                0x7911, 0x3B5E, 0x4F0E, 0x216F, 0x2BF2, 0x1974, 0x20DA,
            ]),
        );
        let b = BigUint::from_bytes_be(&[
            0x35, 0xc6, 0xab, 0x90, 0x48, 0xe2, 0xc5, 0xc6, 0x2f, 0x02, 0x38, 0xf1, 0x83, 0xd2,
            0x85, 0x19, 0xaa, 0x87, 0x48, 0x8b, 0xf3, 0x8f, 0x5b, 0x63, 0x4c, 0xf2, 0x81, 0x90,
            0xbd, 0xf4,
        ]);
        let c = Point::new(
            GaloisField2p15p17::new([
                0x651C, 0x1207, 0x331A, 0x5769, 0x76BD, 0x690D, 0x5159, 0x672C, 0x10C7, 0x559C,
                0x004C, 0x5F04, 0x4476, 0x7DDB, 0x6912, 0x2E81, 0x5FA6,
            ]),
            GaloisField2p15p17::new([
                0x1DD9, 0x38CE, 0x6F31, 0x24A9, 0x66D6, 0x4E59, 0x0301, 0x08C6, 0x1759, 0x39EE,
                0x2B52, 0x1839, 0x6F5A, 0x7FBC, 0x59A3, 0x30F8, 0x66E0,
            ]),
        );

        assert_eq!(a * b, c)
    }
}
