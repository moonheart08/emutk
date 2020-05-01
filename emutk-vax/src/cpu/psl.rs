#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
/// Processor Status Longword
/// 
/// ```text
///  3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1
///  1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5             7 6 5 4 3 2 1 0
/// +-+-+-+-+-+-+---+---+-+---------+-------------+-+-+-+-+-+-+-+-|
/// |C|T|V|M|F|I|CUR|PRV|M|         |             |D|F|I| | | | | |
/// |M|P|M|B|P|S|MOD|MOD|B|   IPL   |     MBZ     |V|U|V|T|N|Z|V|C|
/// | | | |Z|D| |   |   |Z|         |             | | | | | | | | |
/// +-+-+-+-+-+-+---+---+-+---------+-------------+-+-+-+-+-+-+-+-+
/// ```
/// - CM: Compatibility Mode
/// - TP: Trace Pending
/// - VM: Virtual Machine Mode
/// - FPD: First Part Done
/// - IS: Interrupt Stack
/// - CUR_MOD: Current Access Mode
/// - PRV_MOD: Previous Access Mode
/// - IPL: Interrupt Priority Level
/// - DV: Decimal Overflow Enable
/// - FU: Floating Underflow Enable
/// - IV: Integer Overflow Enable
/// - T: Trace Enable
/// - N: Negative
/// - Z: Zero
/// - V: Overflow
/// - C: Carry

pub struct PSL(pub u32);

impl PSL {
    pub fn get_c(self) -> bool {
        (self.0 & 0x01) != 0
    }

    pub fn set_c(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x01;
        self.0 |= val;
    }

    pub fn get_v(self) -> bool {
        (self.0 & 0x02) != 0
    }

    pub fn set_v(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x02;
        self.0 |= val << 1;
    }

    pub fn get_z(self) -> bool {
        (self.0 & 0x04) != 0
    }

    pub fn set_z(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x04;
        self.0 |= val << 2;
    }

    pub fn get_n(self) -> bool {
        (self.0 & 0x08) != 0
    }

    pub fn set_n(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x08;
        self.0 |= val << 3;
    }

    pub fn get_t(self) -> bool {
        (self.0 & 0x10) != 0
    }

    pub fn set_t(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x10;
        self.0 |= val << 4;
    }

    pub fn get_iv(self) -> bool {
        (self.0 & 0x20) != 0
    }

    pub fn set_iv(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x20;
        self.0 |= val << 5;
    }

    pub fn get_fu(self) -> bool {
        (self.0 & 0x40) != 0
    }

    pub fn set_fu(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x40;
        self.0 |= val << 6;
    }

    pub fn get_dv(self) -> bool {
        (self.0 & 0x80) != 0
    }

    pub fn set_dv(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x80;
        self.0 |= val << 7;
    }

    pub fn get_ipl(&self) -> u8 {
        ((self.0 & 0x1F_00_00) >> 16) as u8
    }

    pub fn set_ipl(&mut self, val: u8) {
        assert!(val < 32);
        self.0 &= !0x1f_00_00;
        self.0 |= (val as u32) << 16;
    }

    pub fn get_prv_mod(&self) -> u8 {
        ((self.0 & 0xC0_00_00) >> 22) as u8
    }

    pub fn set_prv_mod(&mut self, val: u8) {
        assert!(val < 4);
        self.0 &= !0xC0_00_00;
        self.0 |= (val as u32) << 22;
    }

    pub fn get_cur_mod(&self) -> u8 {
        ((self.0 & 0x03_00_00_00) >> 24) as u8
    }

    pub fn set_cur_mod(&mut self, val: u8) {
        assert!(val < 4);
        self.0 &= !0x03_00_00_00;
        self.0 |= (val as u32) << 24;
    }

    pub fn get_is(self) -> bool {
        (self.0 & 0x04_00_00_00) != 0
    }

    pub fn set_is(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x04_00_00_00;
        self.0 |= val << 26;
    }

    pub fn get_fpd(self) -> bool {
        (self.0 & 0x08_00_00_00) != 0
    }

    pub fn set_fpd(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x08_00_00_00;
        self.0 |= val << 27;
    }

    pub fn get_vm(self) -> bool {
        (self.0 & 0x20_00_00_00) != 0
    }

    pub fn set_vm(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x20_00_00_00;
        self.0 |= val << 29;
    }

    pub fn get_tp(self) -> bool {
        (self.0 & 0x40_00_00_00) != 0
    }

    pub fn set_tp(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x40_00_00_00;
        self.0 |= val << 30;
    }

    pub fn get_cm(self) -> bool {
        (self.0 & 0x80_00_00_00) != 0
    }

    pub fn set_cm(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x80_00_00_00;
        self.0 |= val << 31;
    }
}
