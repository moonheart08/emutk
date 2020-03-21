use emutk_core::flags::*;

pub struct PSL(u16);

impl PSL {
    fn _get_carry(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    fn _set_carry(&mut self, val: bool) {
        self.0 &= !0b0001;
        self.0 |= val as u16;
    }

    fn _get_overflow(&self) -> bool {
        (self.0 & 0x02) != 0
    }

    fn _set_overflow(&mut self, val: bool) {
        self.0 &= !0b0010;
        self.0 |= (val as u16) << 1;
    }

    fn _get_zero(&self) -> bool {
        (self.0 & 0x04) != 0
    }

    fn _set_zero(&mut self, val: bool) {
        self.0 &= !0b0100;
        self.0 |= (val as u16) << 2;
    }

    fn _get_sign(&self) -> bool {
        (self.0 & 0x08) != 0
    }

    fn _set_sign(&mut self, val: bool) {
        self.0 &= !0b1000;
        self.0 |= (val as u16) << 3;
    }

}

impl PSL {
    
}

// Implementations for FlagData traits. Keep these at the bottom of the file,
// they take up far too much space.

impl CarryFlagData for PSL {
    fn get_carry_u4(&self) -> bool {
        self._get_carry()
    }

    fn get_carry_u8(&self) -> bool {
        self._get_carry()
    }

    fn get_carry_u16(&self) -> bool {
        self._get_carry()
    }

    fn get_carry_u32(&self) -> bool {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn get_carry_u64(&self) -> bool {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn get_carry_u128(&self) -> bool {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }

    fn set_carry_u4(&mut self, val: bool) {
        self._set_carry(val)
    }

    fn set_carry_u8(&mut self, val: bool) {
        self._set_carry(val)
    }

    fn set_carry_u16(&mut self, val: bool) {
        self._set_carry(val)
    }

    fn set_carry_u32(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn set_carry_u64(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn set_carry_u128(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }
}

impl ReducedCarryFlagData for PSL {
    fn get_carry(&self) -> bool {
        self._get_carry()
    }
    
    fn set_carry(&mut self, val: bool) {
        self._set_carry(val)
    }
}

impl OverflowFlagData for PSL {
    fn get_overflow_u4(&self) -> bool {
        self._get_overflow()
    }

    fn get_overflow_u8(&self) -> bool {
        self._get_overflow()
    }

    fn get_overflow_u16(&self) -> bool {
        self._get_overflow()
    }

    fn get_overflow_u32(&self) -> bool {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn get_overflow_u64(&self) -> bool {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn get_overflow_u128(&self) -> bool {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }

    fn set_overflow_u4(&mut self, val: bool) {
        self._set_overflow(val)
    }

    fn set_overflow_u8(&mut self, val: bool) {
        self._set_overflow(val)
    }

    fn set_overflow_u16(&mut self, val: bool) {
        self._set_overflow(val)
    }

    fn set_overflow_u32(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn set_overflow_u64(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn set_overflow_u128(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }
}

impl ReducedOverflowFlagData for PSL {
    fn get_overflow(&self) -> bool {
        self._get_overflow()
    }
    
    fn set_overflow(&mut self, val: bool) {
        self._set_overflow(val)
    }
}

impl ZeroFlagData for PSL {
    fn get_zero_u4(&self) -> bool {
        self._get_zero()
    }

    fn get_zero_u8(&self) -> bool {
        self._get_zero()
    }

    fn get_zero_u16(&self) -> bool {
        self._get_zero()
    }

    fn get_zero_u32(&self) -> bool {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn get_zero_u64(&self) -> bool {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn get_zero_u128(&self) -> bool {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }

    fn set_zero_u4(&mut self, val: bool) {
        self._set_zero(val)
    }

    fn set_zero_u8(&mut self, val: bool) {
        self._set_zero(val)
    }

    fn set_zero_u16(&mut self, val: bool) {
        self._set_zero(val)
    }

    fn set_zero_u32(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn set_zero_u64(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn set_zero_u128(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }
}

impl ReducedZeroFlagData for PSL {
    fn get_zero(&self) -> bool {
        self._get_zero()
    }
    
    fn set_zero(&mut self, val: bool) {
        self._set_zero(val)
    }
}

impl SignFlagData for PSL {
    fn get_sign_u4(&self) -> bool {
        self._get_sign()
    }

    fn get_sign_u8(&self) -> bool {
        self._get_sign()
    }

    fn get_sign_u16(&self) -> bool {
        self._get_sign()
    }

    fn get_sign_u32(&self) -> bool {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn get_sign_u64(&self) -> bool {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn get_sign_u128(&self) -> bool {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }

    fn set_sign_u4(&mut self, val: bool) {
        self._set_sign(val)
    }

    fn set_sign_u8(&mut self, val: bool) {
        self._set_sign(val)
    }

    fn set_sign_u16(&mut self, val: bool) {
        self._set_sign(val)
    }

    fn set_sign_u32(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 32-bit operations.")
    }

    fn set_sign_u64(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 64-bit operations.")
    }

    fn set_sign_u128(&mut self, _: bool) {
        unimplemented!("PDP-11 does not support 128-bit operations.")
    }
}

impl ReducedSignFlagData for PSL {
    fn get_sign(&self) -> bool {
        self._get_sign()
    }
    
    fn set_sign(&mut self, val: bool) {
        self._set_sign(val)
    }
}