use emutk_core::{
    split::Splitable,
    flags
};
pub const FLAG_CARRY: u8      = 0b0000_0001;
pub const FLAG_OVERFLOW: u8   = 0b0000_0010;
pub const FLAG_ZERO: u8       = 0b0000_0100;
pub const FLAG_NEGATIVE: u8   = 0b0000_1000;
pub const FLAG_IRQ_MASK: u8   = 0b0001_0000;
pub const FLAG_HALF_CARRY: u8 = 0b0010_0000;
pub const FLAG_FIRQ_MASK: u8  = 0b0100_0000;
pub const FLAG_ENTIRE: u8     = 0b1000_0000;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CPUFlags(u8);

impl CPUFlags {
    pub fn test_flag(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    pub fn set_flag(&mut self, flag: u8, val: bool) {
        let mask = !flag;
        self.0 &= mask;
        self.0 |= flag * val as u8; // magic! only resets the flag is val is true.
    }
}

pub struct RegisterFile {
    x: u16,
    y: u16,
    u: u16,
    s: u16,
    pc: u16,
    d: u16,
    dp: u8,
    flags: CPUFlags,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            x: 0,
            y: 0,
            u: 0,
            s: 0,
            pc: 0,
            d: 0,
            dp: 0,
            flags: CPUFlags(0),
        }
    }
}

impl RegisterFile {
    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn get_u(&self) -> u16 {
        self.u
    }

    pub fn get_s(&self) -> u16 {
        self.s
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_d(&self) -> u16 {
        self.d
    }

    pub fn get_a(&self) -> u8 {
        self.d.split_be()[0]
    }

    pub fn get_b(&self) -> u8 {
        self.d.split_be()[0]
    }

    pub fn get_dp(&self) -> u8 {
        self.dp
    }

    pub fn get_flags(&self) -> CPUFlags {
        self.flags
    }

    pub fn test_flag(&self, flag: u8) -> bool {
        self.flags.test_flag(flag)
    }
}

impl RegisterFile {
    pub fn set_x(&mut self, val: u16) {
        self.x = val;
    }

    pub fn set_y(&mut self, val: u16) {
        self.y = val;
    }

    pub fn set_u(&mut self, val: u16) {
        self.u = val;
    }

    pub fn set_s(&mut self, val: u16) {
        self.s = val;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn set_d(&mut self, val: u16) {
        self.d = val;
    }

    pub fn set_a(&mut self, val: u8) {
        self.d = self.d.modify_lower_be(val);
    }

    pub fn set_b(&mut self, val: u8) {
        self.d = self.d.modify_upper_be(val);
    }

    pub fn set_dp(&mut self, val: u8) {
        self.dp = val;
    }

    pub fn set_flags(&mut self, val: CPUFlags) {
        self.flags = val;
    }

    pub fn set_flag(&mut self, flag: u8, val: bool) {
        self.flags.set_flag(flag, val)
    }

}

impl RegisterFile {
    pub fn get_dp_addr(&self, lower: u8) -> u16 {
        u16::from_be_bytes([lower, self.dp])
    }
}

impl flags::CarryFlagData for CPUFlags {
    fn get_carry_u4(&self) -> bool {
        self.test_flag(FLAG_HALF_CARRY)
    }

    fn get_carry_u8(&self) -> bool {
        self.test_flag(FLAG_CARRY)
    }

    fn get_carry_u16(&self) -> bool {
        self.test_flag(FLAG_CARRY)
    }

    fn get_carry_u32(&self) -> bool { 
        self.test_flag(FLAG_CARRY)
    }
    fn get_carry_u64(&self) -> bool { 
        self.test_flag(FLAG_CARRY)
    }
    fn get_carry_u128(&self) -> bool { 
        self.test_flag(FLAG_CARRY)
    }
   
    fn set_carry_u4(&mut self, val: bool) {
        self.set_flag(FLAG_HALF_CARRY, val)
    }
    
    fn set_carry_u8(&mut self, val: bool) {
        self.set_flag(FLAG_CARRY, val)
    }
    
    fn set_carry_u16(&mut self, val: bool) {
        self.set_flag(FLAG_CARRY, val)
    }

    fn set_carry_u32(&mut self, val: bool) { 
        self.set_flag(FLAG_CARRY, val)
    }

    fn set_carry_u64(&mut self, val: bool) {  
        self.set_flag(FLAG_CARRY, val)
    }
    
    fn set_carry_u128(&mut self, val: bool) { 
        self.set_flag(FLAG_CARRY, val)
    }
}

impl flags::ReducedOverflowFlagData for CPUFlags {
    fn get_overflow(&self) -> bool {
        self.test_flag(FLAG_OVERFLOW)
    }

    fn set_overflow(&mut self, val: bool) {
        self.set_flag(FLAG_OVERFLOW, val)
    }
}

impl flags::ReducedZeroFlagData for CPUFlags {
    fn get_zero(&self) -> bool {
        self.test_flag(FLAG_OVERFLOW)
    }

    fn set_zero(&mut self, val: bool) {
        self.set_flag(FLAG_OVERFLOW, val)
    }
}

impl flags::ReducedSignFlagData for CPUFlags {
    fn get_sign(&self) -> bool {
        self.test_flag(FLAG_NEGATIVE)
    }

    fn set_sign(&mut self, val: bool) {
        self.set_flag(FLAG_NEGATIVE, val)
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_dp_addr() {
        let rf = RegisterFile::new();

        assert_eq!(rf.get_dp_addr(12), 12);
    }
}