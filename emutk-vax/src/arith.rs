use emutk_core::ByteReprNum;
use num::PrimInt;
use num::traits::cast::AsPrimitive;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CVZN(u8); // integer op flags

impl CVZN {
    pub fn blank() -> CVZN {
        CVZN(0)
    }
    pub fn get_c(self) -> bool {
        (self.0 & 0x01) != 0
    }

    pub fn set_c(&mut self, val: bool) {
        let val = val as u8;
        self.0 &= !0x01;
        self.0 |= val;
    }

    pub fn get_v(self) -> bool {
        (self.0 & 0x02) != 0
    }

    pub fn set_v(&mut self, val: bool) {
        let val = val as u8;
        self.0 &= !0x02;
        self.0 |= val << 1;
    }

    pub fn get_z(self) -> bool {
        (self.0 & 0x04) != 0
    }

    pub fn set_z(&mut self, val: bool) {
        let val = val as u8;
        self.0 &= !0x04;
        self.0 |= val << 2;
    }

    pub fn get_n(self) -> bool {
        (self.0 & 0x08) != 0
    }

    pub fn set_n(&mut self, val: bool) {
        let val = val as u8;
        self.0 &= !0x08;
        self.0 |= val << 3;
    }
}

pub trait VAXNum: ByteReprNum + PrimInt + AsPrimitive<u32> + PrimitiveFrom<u8> + PrimitiveFrom<u32>
    + std::fmt::LowerHex
{
    fn from_u8(val: u8) -> Self;
    fn flagged_add(self, other: Self) -> (CVZN, Self);
    fn flagged_sub(self, other: Self) -> (CVZN, Self);
    fn flagged_mul(self, other: Self) -> (CVZN, Self);
    fn flagged_div(self, other: Self) -> (CVZN, Self);
    fn flagged_ash(self, cnt: i8) -> (CVZN, Self);
    fn flagged_neg(self) -> (CVZN, Self);

    fn flagged_cmp(self, other: Self) -> CVZN {
        self.flagged_add(other).0
    }

    fn calc_nz(self) -> CVZN;
}

pub trait PrimitiveFrom<F>: 'static + Copy {
    fn primitive_from(from:F)->Self;
}

impl<F,T> PrimitiveFrom<F> for T
where
    F: AsPrimitive<T>,
    T: 'static + Copy ,
{

    fn primitive_from(from:F)->Self{
       from.as_()
    }
}

macro_rules! impl_vaxnum {
    ($($num:ty, $signednum:ty);+) => {
        $(
        impl VAXNum for $num {
            fn from_u8(val: u8) -> Self {
                val as $num
            }

            fn flagged_add(self, other: Self) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                let (val, carry) = self.overflowing_add(other);
                let (_, overflow) = (self as $signednum).overflowing_add(other as $signednum);
                flags.set_c(carry);
                flags.set_v(overflow);
                flags.set_n((val as $signednum) < 0);
                flags.set_z(val == 0);
                (flags, val)
            }

            fn flagged_sub(self, other: Self) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                let (val, carry) = self.overflowing_sub(other);
                let (_, overflow) = (self as $signednum).overflowing_sub(other as $signednum);
                flags.set_c(carry);
                flags.set_v(overflow);
                flags.set_n((val as $signednum) < 0);
                flags.set_z(val == 0);
                (flags, val)
            }

            fn flagged_div(self, other: Self) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                if (other == 0) {
                    flags.set_v(true);
                    (flags, 0)
                } else {
                    let (val, overflow) = (self as $signednum).overflowing_div(other as $signednum);
                    flags.set_n(val < 0);
                    flags.set_v(overflow);
                    flags.set_z(val == 0);
                    (flags, val as $num)
                }
            }

            fn flagged_mul(self, other: Self) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                let (val, overflow) = (self as $signednum).overflowing_mul(other as $signednum);
                flags.set_n(val < 0);
                flags.set_v(overflow);
                flags.set_z(val == 0);
                (flags, val as $num)
            }

            fn flagged_ash(self, cnt: i8) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                if (cnt > 0) {
                    let (val, _) = self.overflowing_shl(cnt as u32);
                    flags.set_n(val as $signednum < 0);
                    flags.set_z(val == 0);
                    if !(val as $signednum < 0 && self as $signednum < 0) {
                        flags.set_v(true);
                    }
                    (flags, val)
                } else {
                    let cnt = cnt.abs();
                    let (val, _) = (self as $signednum).overflowing_shr(cnt as u32);
                    flags.set_n(val < 0);
                    flags.set_z(val == 0);
                    (flags, val as $num)
                }
            }

            fn flagged_neg(self) -> (CVZN, Self) {
                let mut flags = CVZN::blank();
                let (val, overflow) = (self as $signednum).overflowing_neg();
                flags.set_n(val < 0);
                flags.set_v(overflow);
                flags.set_z(val == 0);
                flags.set_c(val != 0);
                (flags, val as $num)
            }

            fn calc_nz(self) -> CVZN {
                let mut flags = CVZN::blank();
                flags.set_n((self as $signednum) < 0);
                flags.set_z(self == 0);
                flags
            }
        }
        )+
    };
}

impl_vaxnum!(u8, i8; u16, i16; u32, i32; u64, i64; u128, i128);