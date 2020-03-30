use emutk_core::{
    cycles::Cycles,
    bus::{
        TaggedBus,
    },
    bytes::ByteRepr,
};

use crate::{
    bus::{
        VAXBus,
        VAXBusError,
        VAXBusTag,
        VAXBusReturnTag,
        PrivilegeMode,
    }
};

use bytemuck::{
    Pod,
    bytes_of,
    from_bytes,
};

use std::num::Wrapping;

pub mod exec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
/// Processor Status Longword
/// 
/// ```text
///  3 3 2 2 2 2 2   2   2 2         1
///  1 0 9 8 7 6 5   3   1 0         5             7 6 5 4 3 2 1 0
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

pub struct PSL(u32);

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
        self.0 |= val;
    }

    pub fn get_z(self) -> bool {
        (self.0 & 0x04) != 0
    }

    pub fn set_z(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x04;
        self.0 |= val;
    }

    pub fn get_n(self) -> bool {
        (self.0 & 0x08) != 0
    }

    pub fn set_n(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x08;
        self.0 |= val;
    }

    pub fn get_t(self) -> bool {
        (self.0 & 0x10) != 0
    }

    pub fn set_t(&mut self, val: bool) {
        let val = val as u32;
        self.0 &= !0x10;
        self.0 |= val;
    }
}

pub struct VAXCPU {
    gpr: [Wrapping<u32>;16],
    psl: PSL,

    halted: bool,

    bus: VAXBus,

    last_read: Option<u32>,
    last_read_data: u32, // Only used if last_read is Some.
}

impl VAXCPU {
    pub fn new(bus: VAXBus) -> Self {
        VAXCPU {
            gpr: [Wrapping(0); 16],
            psl: PSL(0),
            bus,

            last_read: None,
            last_read_data: 0xDEADBEEF,
            halted: false,
        }
    }

    #[inline]
    pub fn pc(&self) -> Wrapping<u32> {
        self.gpr[15]
    }

    #[inline]
    pub fn pc_mut(&mut self) -> &mut Wrapping<u32> {
        &mut self.gpr[15]
    }

    #[inline]
    pub fn set_pc(&mut self, new: Wrapping<u32>) {
        self.gpr[15] = new;
    }

    #[inline]
    pub fn sp(&self) -> Wrapping<u32> {
        self.gpr[14]
    }

    #[inline]
    pub fn sp_mut(&mut self) -> &mut Wrapping<u32> {
        &mut self.gpr[14]
    }

    #[inline]
    pub fn set_sp(&mut self, new: Wrapping<u32>) {
        self.gpr[14] = new;
    }

    #[inline]
    pub fn gpr(&self) -> &[Wrapping<u32>;16] {
        &self.gpr
    }

    #[inline]
    pub fn gpr_mut(&mut self) -> &mut [Wrapping<u32>;16] {
        &mut self.gpr
    }

    #[inline]
    pub fn set_gpr(&mut self, gpr: [Wrapping<u32>; 16]) {
        self.gpr = gpr;
    }

    #[inline]
    pub fn psl(&self) -> PSL {
        self.psl
    }

    #[inline]
    pub fn psl_mut(&mut self) -> &mut PSL {
        &mut self.psl
    }

    #[inline]
    pub fn set_psl(&mut self, new: PSL) {
        self.psl = new;
    }
}

impl VAXCPU {

    pub fn read_val<T: ByteRepr + Clone>(&mut self, addr: usize) -> (Cycles, Result<T, ()>) {
        let tag = VAXBusTag {
            priv_mode: PrivilegeMode::Kernel, //FIXME: properly read priv mode from PSL
        };

        let (cycles, res) = self.bus.read_val_tagged(addr, tag);
    
        match res {
            Ok((v, _)) => {
                return (cycles, Ok(v));
            }
            Err(_) => {
                todo!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::VAXBus;

    #[test]
    fn gpr() {
        let mut cpu = VAXCPU::new(VAXBus::new(0, 0, 0, 0));
        let a = cpu.gpr[3];
        *cpu.pc_mut() = a + Wrapping(1);
        cpu.gpr_mut()[2] += Wrapping(5);
    }
}