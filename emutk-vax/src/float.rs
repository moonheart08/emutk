use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
};

const HFLOAT_SIGN_MASK: u128  = 0x8000_0000_0000_0000_0000_0000_0000_0000;
const DFLOAT_SIGN_MASK: u64   = 0x8000_0000_0000_0000;
const HFLOAT_FRACT_MASK: u128 = 0x00FF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
const DFLOAT_FRACT_MASK: u64  = 0x00FF_FFFF_FFFF_FFFF;

#[derive(Copy, Clone)]
pub struct HFloat(u128);

impl HFloat {
    pub fn mantissa(self) -> u8 {
        let lsb = self.0.to_le_bytes()[0];
        lsb & 0x7F
    }

    pub fn fractional(self) -> u128 {
        self.0 & HFLOAT_FRACT_MASK >> 8
    }

    pub fn sign(self) -> bool {
        self.0 & HFLOAT_SIGN_MASK != 0
    }

    pub fn finite(self) -> bool {
        if self.mantissa() == 0x7F {
            false
        } else {
            true
        }
    }
}

impl Neg for HFloat {
    type Output = HFloat;

    fn neg(self) -> Self::Output {
        HFloat(self.0 ^ HFLOAT_SIGN_MASK)
    }
}

pub struct DFloat(u64);

impl DFloat {
    pub fn mantissa(self) -> u8 {
        let lsb = self.0.to_le_bytes()[0];
        lsb & 0x7F
    }

    pub fn fractional(self) -> u64 {
        self.0 & DFLOAT_FRACT_MASK
    }

    pub fn sign(self) -> bool {
        self.0 & DFLOAT_SIGN_MASK != 0
    }

    pub fn finite(self) -> bool {
        if self.mantissa() == 0x7F {
            false
        } else {
            true
        }
    }
}


impl Neg for DFloat {
    type Output = DFloat;

    fn neg(self) -> Self::Output {
        DFloat(self.0 ^ DFLOAT_SIGN_MASK)
    }
}