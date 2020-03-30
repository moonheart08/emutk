use emutk_core::{
    bus::{Bus, BusError},
};

#[derive(Clone, Debug)]
pub enum MMUDenyReasons {
    InvalidPTE(u32),
    LengthViolation(),
    PTEAccessFailed {
        target_addr: u32,
    },
}
#[derive(Clone, Debug)]
pub enum VAXBusError {
    OpenBusError {
        trigger_addr: u32,
    },
    MMUDeniedRead {
        trigger_addr: u32,
    },
    MMUDeniedWrite {
        trigger_addr: u32,
    },
    MMUTranslationError {
        trigger_addr: u32,
        reason: MMUDenyReasons,
    },
    DeviceDeniedRead {
        trigger_addr: u32,
    },
    DeviceDeniedWrite {
        trigger_addr: u32,
    },
    OutOfBounds {
        trigger_addr: usize,
    }
}

impl BusError for VAXBusError {
    fn out_of_bounds(addr: usize) -> Self {
        Self::OutOfBounds{
            trigger_addr: addr,
        }
    }

    fn is_oob_error(&self) -> bool {
        if let Self::OutOfBounds{ trigger_addr: _ } = self {
            return true;
        } else {
            return false;
        }
    }

    fn get_triggering_address(&self) -> usize {
        match self {
            Self::OpenBusError { trigger_addr: v } => *v as usize,
            Self::MMUDeniedRead { trigger_addr: v } => *v as usize,
            Self::MMUDeniedWrite { trigger_addr: v } => *v as usize,
            Self::MMUTranslationError { trigger_addr: v, reason: _ } => *v as usize,
            Self::DeviceDeniedWrite { trigger_addr: v } => *v as usize,
            Self::DeviceDeniedRead { trigger_addr: v } => *v as usize,
            Self::OutOfBounds { trigger_addr: v } => *v as usize,
        }
    }
}

pub struct VAXBus {
    ram: Vec<u8>,
    ram_begin: usize,
    ram_end: usize,
    rom: Vec<u8>,
    rom_begin: usize,
    rom_end: usize,
}

impl Bus<VAXBusError> for VAXBus {
    const MAX_OPERATION_SIZE: usize = 

    fn read(&mut self, addr: usize, size: usize) -> (Cycles, Result<&[u8], Err>) {

    }
}