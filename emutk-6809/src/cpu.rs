use crate::regfile::RegisterFile;
use emutk_core::{
    bus::{
        Bus,
        SimpleBusError,
    },
    ByteRepr,
};
pub struct Motorola6809<B>
    where B: Bus<SimpleBusError, ()>
{
    regs: RegisterFile,
    halted: bool,

    bus: B,
}

impl<B> Motorola6809<B> 
    where B: Bus<SimpleBusError, ()>
{
    pub fn read_direct_argument<T: ByteRepr>(&mut self, pc: u16) -> (u16, T) {
        let nb: u8 = self.bus.read((pc.overflowing_add(1).0) as usize).unwrap();
        let addr = self.regs.get_dp_addr(nb);
        let val: T = self.bus.read(addr as usize).unwrap();
        (addr, val)
    }

    pub fn test_and_set_flags(&mut self, overflow: bool, carry: bool) {
        
    }

    pub fn execute_instruction(&mut self) {
        let pc = self.regs.get_pc();

        let opcode: u8 = self.bus.read(pc as usize).unwrap();

        match opcode {
            // NEG direct
            0x00 => {
                let (addr, val) = self.read_direct_argument::<u16>(pc);
                let (val, c) = (val as i16).overflowing_neg();
                let _ = self.bus.write(addr as usize, val as u16);
            },

            // NOP
            0x12 => {
                
            },

            _ => {
                self.invalid_instruction();
            }
        }
    }

    pub fn invalid_instruction(&mut self) {
        // Do nothing for now.
        self.regs.set_pc(self.regs.get_pc() + 1);
    }

    pub fn add_to_pc(&mut self, amnt: i16) {
        let pc = self.regs.get_pc() as i16;
        let (pc, _) = pc.overflowing_add(amnt);
        self.regs.set_pc(pc as u16);
    } 
}