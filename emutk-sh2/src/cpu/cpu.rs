use emutk_core::{
    bus::{
        Bus,
    },
    cycles::Cycles,
    ByteRepr,
};

use crate::cpu::SH2RegisterFile;

pub struct SH2CPU<B: Bus<()>> {
    bus: B,
    regs: SH2RegisterFile,
}