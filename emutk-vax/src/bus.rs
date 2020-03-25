use emutk_core::{
    bus::{Bus, BusErrorCommon},
};

pub enum VAXBusError {
    OpenBusError {
        trigger_addr: u32,
    },
    MMUDeniedReadAccess {
        trigger_addr: u32,
    },
    MMUDeniedWriteAccess {
        trigger_addr: u32,
    },
    MMUTranslationError {
        trigger_addr: u32,
    },
    DeviceDeniedRead {
        trigger_addr: u32,
    },
    DeviceDeniedWrite {
        trigger_addr: u32,
    },
    DeviceTranslationError {
        trigger_addr: u32,
    },
    OutOfBounds,
}

impl BusErrorCommon for VAXBusError {
    fn out_of_bounds() -> Self {
        VAXBusError::OutOfBounds
    }
}

pub struct VAXBus {
    ram: Vec<u8>,
    rom: Vec<u8>,
}