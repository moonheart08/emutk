use emutk_core::{
    cycles::Cycles,
    bus::{Bus, TaggedBus, BusError},
    bytes::ByteRepr,
};
use num_derive::*;
use num_traits::{ToPrimitive, FromPrimitive};
use bytemuck::{
    Pod,
    bytes_of,
    from_bytes,
};
#[derive(Clone, Debug)]
pub enum MMUDenyReasons {
    InvalidPTE(u32),
    LengthViolation(),
    PTEAccessFailed {
        target_addr: u32,
    },
    InvalidAddress(), // if addr larger than (2^31 - 1) * 3
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum PrivilegeMode {
    Kernel = 0,
    Executive = 1,
    Supervisor = 2,
    User = 3,
}

impl PTEProtectionCode {
    pub fn can_access(self, mode: PrivilegeMode, access: MemoryAccessType) -> bool {
        let m = self.to_u8().unwrap();
        let rm = ((m & 0b1100) >> 2) as u8;
        let wm = !(m & 0b0011) as u8;

        // This is a magic algorithm that DEC used for this same purpose.
        // Don't worry too much about how it works, it just does(tm)
        match mode.to_u8().unwrap() {
            16..=u8::MAX => unreachable!(),

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


impl BusError for VAXBusError {
    fn get_triggering_address(&self) -> usize {
        match self {
            Self::OpenBusError { trigger_addr: v } => *v as usize,
            Self::MMUDeniedRead { trigger_addr: v } => *v as usize,
            Self::MMUDeniedWrite { trigger_addr: v } => *v as usize,
            Self::MMUTranslationError { trigger_addr: v, reason: _ } => *v as usize,
            Self::DeviceDeniedWrite { trigger_addr: v } => *v as usize,
            Self::DeviceDeniedRead { trigger_addr: v } => *v as usize,
        }
    }
}

const VAX_REGION_SIZE: usize = (1 << 31) - 1;
const VAX_MMU_PENALTY: usize = 1;

pub struct VAXBus {
    ram: Vec<u8>,
    ram_begin: usize,
    ram_end: usize,
    rom: Vec<u8>,
    rom_begin: usize,
    rom_end: usize,

    mmu_enabled: bool,

    p0_len: u32, // no larger than 2^23-1
    p0_base: u32,
    p1_len: u32, // no larger than 2^23-1
    p1_base: u32,
    sys_len: u32, // no larger than 2^23-1
    sys_base: u32,
}

impl TaggedBus<VAXBusError, VAXBusTag, VAXBusReturnTag> for VAXBus {
    const MAX_OPERATION_SIZE: usize = 512;
    const MAX_ADDRESS: usize = u32::MAX as usize;


    fn read_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, tag: VAXBusTag)
        -> (Cycles, Result<(T, VAXBusReturnTag), VAXBusError>) {
        self.sanity_check();
        let size = std::mem::size_of::<T>();
        assert!(size <= Self::MAX_OPERATION_SIZE);
        assert!(Self::MAX_ADDRESS > addr);
        
        if self.mmu_enabled {
            self.read_val_tagged_mmu(addr, tag)
        } else {
            self.read_val_tagged_nommu(addr, tag)
        }
    }

    fn write_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, tag: VAXBusTag) 
        -> (Cycles, Result<VAXBusReturnTag, VAXBusError>) {
        self.sanity_check();
        let size = std::mem::size_of::<T>();
        assert!(size < Self::MAX_OPERATION_SIZE + 1);
        assert!(Self::MAX_ADDRESS > addr);

        if self.mmu_enabled {
            self.write_val_tagged_mmu(addr, data, tag)
        } else {
            self.write_val_tagged_nommu(addr, data, tag)
        }
    }
}

impl VAXBus {
    fn sanity_check(&self) {
        debug_assert!((self.p0_len as usize) < VAX_REGION_SIZE / 512);
        debug_assert!((self.p1_len as usize) < VAX_REGION_SIZE / 512);
        debug_assert!((self.sys_len as usize) < VAX_REGION_SIZE / 512);
        debug_assert_eq!(self.ram.len(), self.ram_end - self.ram_begin);
        debug_assert_eq!(self.rom.len(), self.rom_end - self.rom_begin);
    }

    fn read_pte(&self, addr: usize) -> Option<u32> {
        let base = match (addr as u32 & 0x3000_0000) >> 30 {
            0 => self.p0_base,
            1 => self.p1_base,
            2 => self.sys_base,
            _ => unreachable!(),
        } as usize;
        if base >= self.ram_begin && base + 4 <= self.ram_end {
            let begin = base - self.ram_begin;
            let end = base + 4 - self.ram_begin;
            Some(*from_bytes::<u32>(&self.ram[begin..=end]))
        } else if base >= self.rom_begin && base + 4 <= self.rom_end {
            let begin = base - self.rom_begin;
            let end = base + 4 - self.rom_begin;
            Some(*from_bytes::<u32>(&self.rom[begin..=end]))
        } else {
            None // Choose to just not handle IO devices, or the table running
                 // off the end of RAM or ROM.
        }
    }

    fn read_val_tagged_nommu<T: ByteRepr + Clone>(&mut self, addr: usize, tag: VAXBusTag)
        -> (Cycles, Result<(T, VAXBusReturnTag), VAXBusError>) {
        
        let size = std::mem::size_of::<T>();
        let base_cyc_count = size / 4;
        if addr >= self.ram_begin && addr + size <= self.ram_end {
            let begin = addr - self.ram_begin;
            let end = addr + size - self.ram_begin;
            return (
                Cycles(base_cyc_count), 
                Ok(
                    (
                    T::from_le_bytes(&self.ram[begin..end]).clone(),
                    VAXBusReturnTag(),
                    )
                )
            );
        }

        if addr >= self.rom_begin && addr + size <= self.rom_end {
            let begin = addr - self.rom_begin;
            let end = addr + size - self.rom_begin;
            return (
                Cycles(base_cyc_count), 
                Ok(
                    (
                    T::from_le_bytes(&self.rom[begin..end]).clone(),
                    VAXBusReturnTag(),
                    )
                )
            );
        }

        todo!()
    }

    fn write_val_tagged_nommu<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, tag: VAXBusTag) 
        -> (Cycles, Result<VAXBusReturnTag, VAXBusError>) {
        let size = std::mem::size_of::<T>();
        let base_cyc_count = size / 4;

        if addr >= self.ram_begin && addr + size <= self.ram_end {
            let begin = addr - self.ram_begin;
            let end = addr + size - self.ram_begin;
            let buf = &mut self.ram[begin..=end];

            data.copy_into_le_bytes(buf);

            return (Cycles(base_cyc_count), Ok(VAXBusReturnTag()));
        }

        if addr >= self.rom_begin && addr + size <= self.rom_end {
            let begin = addr - self.ram_begin;
            let end = addr + size - self.ram_begin;

            return (Cycles(1), Err(VAXBusError::DeviceDeniedWrite{trigger_addr: addr as u32}));
        }

        todo!()
    }

    fn read_val_tagged_mmu<T: ByteRepr + Clone>(&mut self, addr: usize, tag: VAXBusTag)
        -> (Cycles, Result<(T, VAXBusReturnTag), VAXBusError>) {
        let size = std::mem::size_of::<T>();
        let base_cyc_count = (size / 4) + VAX_MMU_PENALTY; // MMU penalty applied.
        let pte = self.read_pte(addr);
        
        todo!();
    }

    fn write_val_tagged_mmu<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, tag: VAXBusTag) 
        -> (Cycles, Result<VAXBusReturnTag, VAXBusError>) {
        let size = std::mem::size_of::<T>();
        let base_cyc_count = (size / 4) + VAX_MMU_PENALTY; // MMU penalty applied.
        let pte = self.read_pte(addr);

        todo!();
    }
}

impl VAXBus {
    pub fn new(ram_begin: usize, ram_end: usize, rom_begin: usize, rom_end: usize) -> Self {
        VAXBus {
            ram: vec![0 ; ram_end - ram_begin],
            rom: vec![0 ; rom_end - rom_begin],
            ram_begin,
            ram_end,
            rom_begin,
            rom_end,
            mmu_enabled: false,
            p0_base: 0,
            p0_len: 0,
            p1_base: 0,
            p1_len: 0,
            sys_base: 0,
            sys_len: 0,
        }
    }

    #[inline]
    pub fn ram(&self) -> &[u8] {
        &self.ram[..]
    }
    #[inline]
    pub fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram[..]
    }
    #[inline]
    pub fn rom(&self) -> &[u8] {
        &self.rom[..]
    }
    #[inline]
    pub fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom[..]
    }
}

#[derive(Clone, Debug)]
pub struct VAXBusTag {
    pub priv_mode: PrivilegeMode,
}

/// Shell type for now.
#[derive(Clone, Debug)]
pub struct VAXBusReturnTag();