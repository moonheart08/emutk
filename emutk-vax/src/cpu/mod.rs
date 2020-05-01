use emutk_core::{
    cycles::Cycles,
    ByteRepr,
};

use num_derive::*;

pub mod exec;
pub mod instrs;
pub mod regfile;

mod psl;
pub use psl::PSL;
use regfile::VAXRegisterFile;

use crate::Error;
use crate::bus::VAXBus;
use crate::CVZN;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum PrivilegeMode {
    Kernel = 0,
    Executive = 1,
    Supervisor = 2,
    User = 3,
}

pub struct VAXCPU<Bus: VAXBus> {
    regfile: VAXRegisterFile,


    halted: bool,

    bus: Option<Bus>,

    cur_cycle: Cycles,
}

impl<Bus: VAXBus> VAXCPU<Bus> {
    pub fn new() -> Self {
        VAXCPU {
            regfile: VAXRegisterFile::new(),

            halted: false,
            bus: None,

            cur_cycle: Cycles(0),
        }
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn give_bus(&mut self, bus: Bus) {
        if let Some(_) = self.bus {
            panic!("Attempted to give CPU that already has a bus a bus.");
        }
        self.bus = Some(bus);
    }

    pub fn take_bus(&mut self) -> Option<Bus> {
        let mut v = None;
        std::mem::swap(&mut v, &mut self.bus);
        v
    }

    pub fn read_val<T: ByteRepr>(&mut self, addr: u32) ->  Result<T, Error> {
        let bus = (&mut self.bus).as_mut().expect("No bus!");

        if self.regfile.get_mapen() {
            todo!()
        } else {
            let (cyc, res) = bus.read_val_tagged(addr as usize, ());
            let out: T = res.unwrap().0; 
            self.cur_cycle += cyc;
            Ok(out)
        }
    }

    pub fn write_val<T: ByteRepr>(&mut self, addr: u32, val: T) -> Result<(), Error> {
        let bus = (&mut self.bus).as_mut().expect("No bus!");

        if self.regfile.get_mapen() {
            todo!()
        } else {
            let (cyc, _) = bus.write_val_tagged(addr as usize, val, ());
            self.cur_cycle += cyc;
            Ok(())
        }
    }

    pub fn can_read_val<T: ByteRepr>(&mut self, _addr: u32) -> Result<(), Error> {
        if self.regfile.get_mapen() {
            todo!()
        } else {
            Ok(())
        }
    }

    pub fn can_write_val<T: ByteRepr>(&mut self, _addr: u32) -> Result<(), Error> {
        if self.regfile.get_mapen() {
            todo!()
        } else {
            Ok(())
        }
    }

    pub fn commit_flags(&mut self, flags: CVZN) {
        self.regfile.get_psl_mut().set_c(flags.get_c());
        self.regfile.get_psl_mut().set_v(flags.get_v());
        self.regfile.get_psl_mut().set_z(flags.get_z());
        self.regfile.get_psl_mut().set_n(flags.get_n());
    }
}