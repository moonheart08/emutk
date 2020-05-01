use crate::cpu::{
    PrivilegeMode,
    VAXCPU,
};

use emutk_core::{
    bus::TaggedBus,
    ByteRepr,
};

use num_traits::{ToPrimitive, FromPrimitive};
use num_derive::*;

#[derive(Clone, Debug)]
pub enum MMUDenyReasons {
    InvalidPTE(u32),
    LengthViolation(),
    PTEAccessFailed {
        target_addr: u32,
    },
    InvalidAddress(), // if addr larger than (2^30 - 1) * 3
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum PTEProtectionCode {
                             // KESU
    NoAccess      = 0b0000,  // ----
    ZeroPage      = 0b0001,  // RRRR
    KernW         = 0b0010,  // W---
    KernR         = 0b0011,  // R---
    UserW         = 0b0100,  // WWWW
    ExecW         = 0b0101,  // WW--
    ExecRKernW    = 0b0110,  // WR--
    ExecR         = 0b0111,  // RR--
    SuperW        = 0b1000,  // WWW-
    SuperRExecW   = 0b1001,  // WWR-
    SuperRKernW   = 0b1010,  // WRR-
    SuperR        = 0b1011,  // RRR-
    UserRSuperW   = 0b1100,  // WWWR
    UserRExecW    = 0b1101,  // WWRR
    UserRKernW    = 0b1110,  // WRRR
    UserR         = 0b1111,  // RRRR
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryAccessType {
    Read,
    Write,
}

impl PTEProtectionCode {
    pub fn can_access(self, mode: PrivilegeMode, access: MemoryAccessType) -> bool {
        let m = self.to_u8().unwrap();
        let rm = ((m & 0b1100) >> 2) as u8;
        let wm = !(m & 0b0011) as u8;

        // This is a magic algorithm that DEC used for this same purpose.
        // Don't worry too much about how it works, it just does(tm)
        match mode.to_u8().unwrap() {
            16..=std::u8::MAX => unreachable!(),

            0 => false,
            4 => true,

            v if v < wm => {
                true
            },

            v if (access == MemoryAccessType::Read) && (v <= rm)  => {
                true
            },

            _ => false,

        }
    }
}

pub fn translate_address<Bus: TaggedBus<(), (), ()>>(iaddr: u32, cpu: &mut VAXCPU<Bus>) -> u32 {
    todo!()
}

