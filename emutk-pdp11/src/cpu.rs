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

impl ReducedCarryFlagData for PSL {
    fn get_carry(&self) -> bool {
        self._get_carry()
    }
    
    fn set_carry(&mut self, val: bool) {
        self._set_carry(val)
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

impl ReducedZeroFlagData for PSL {
    fn get_zero(&self) -> bool {
        self._get_zero()
    }
    
    fn set_zero(&mut self, val: bool) {
        self._set_zero(val)
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