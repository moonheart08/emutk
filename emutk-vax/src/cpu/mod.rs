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
use crate::cpu::instrs::MultiInstruction;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum PrivilegeMode {
    Kernel = 0,
    Executive = 1,
    Supervisor = 2,
    User = 3,
}

pub struct VAXCPU<'bus, Bus: VAXBus + 'static> {
    pub regfile: VAXRegisterFile,


    halted: bool,

    bus: Option<&'bus mut Bus>,

    itable: Option<[Option<fn(&mut VAXCPU<'_, Bus> , &mut Cycles) -> Result<(), Error>>; 1280]>,

    cur_cycle: Cycles,

    multi_instr_active: MultiInstruction,
}

impl<'bus, Bus: VAXBus> VAXCPU<'bus, Bus> {
    pub fn new() -> Self {
        let mut cpu = VAXCPU {
            regfile: VAXRegisterFile::new(),

            halted: false,
            bus: None,
            itable: None,


            cur_cycle: Cycles(0),
            multi_instr_active: MultiInstruction::None,
        };
        cpu.setup_instr_table();
        cpu
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn cur_cycle(&self) -> usize {
        self.cur_cycle.0
    }

    pub fn give_bus(&mut self, bus: &'bus mut Bus) {
        if let Some(_) = self.bus {
            panic!("Attempted to give CPU that already has a bus a bus.");
        }
        self.bus = Some(bus);
    }

    pub fn take_bus(&mut self) -> Option<&'bus mut Bus> {
        let mut v = None;
        std::mem::swap(&mut v, &mut self.bus);
        v
    }

    pub fn read_val<T: ByteRepr>(&mut self, addr: u32) ->  Result<T, Error> {
        let bus = (&mut self.bus).as_mut().expect("No bus!");

        if self.regfile.get_mapen() {
            todo!()
        } else {
            let (cyc, res) = bus.read_val(addr as usize);
            let out: T = res.unwrap(); 
            self.cur_cycle += cyc;
            Ok(out)
        }
    }

    pub fn write_val<T: ByteRepr>(&mut self, addr: u32, val: T) -> Result<(), Error> {
        let bus = (&mut self.bus).as_mut().expect("No bus!");

        if self.regfile.get_mapen() {
            todo!()
        } else {
            //println!("WR: {:?} to {:02$x}", val.into_le_bytes(), addr, 8);
            let (cyc, _) = bus.write_val(addr as usize, val);
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
        let mut psl = *self.regfile.get_psl();
        psl.set_c(flags.get_c());
        psl.set_v(flags.get_v());
        psl.set_z(flags.get_z());
        psl.set_n(flags.get_n());
        self.regfile.set_psl(psl);
    }
}

impl<'bus, Bus: VAXBus> VAXCPU<'bus, Bus> {
    pub fn prepare_as_microvax(&mut self) {
        self.regfile.set_pc(0x2004_0000);
    }
}