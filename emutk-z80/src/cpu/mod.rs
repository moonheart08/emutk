use emutk_core::{
    bus::{
        Bus,
    },
    cycles::Cycles,
    ByteRepr,
    split::*,
};

use crate::bus::Z80BusError;
use std::num::Wrapping;

pub mod exec;
mod regfile;
pub use regfile::Z80Registers;
pub struct Z80CPU<T: Bus<Z80BusError>> {
    regs: Z80Registers,

    bus: T,
}

impl<T: Bus<Z80BusError>> Z80CPU<T> {
    pub fn regs(&self) -> &Z80Registers {
        &self.regs
    }

    pub fn regs_mut(&mut self) -> &mut Z80Registers {
        &mut self.regs
    }

    pub fn bus(&self) -> &T {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut T {
        &mut self.bus
    }
}

impl<T: Bus<Z80BusError>> Z80CPU<T> {
    pub fn read_val<V: ByteRepr>(&mut self, addr: u16) -> (Cycles, V) {
        let (cy, res) = self.bus.read_val::<V>(addr as usize);
        (cy, res.expect("Impossible."))
    }

    pub fn write_val<V: ByteRepr>(&mut self, addr: u16, data: V) -> Cycles {
        let (cy, _) = self.bus.write_val::<V>(addr as usize, data);
        cy
    }
}