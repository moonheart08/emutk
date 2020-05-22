use emutk_core::{
    cycles::Cycles,
    bus::TaggedBus,
    bus::Bus,
    ByteRepr,
};

pub trait VAXDevice<Err, InTag, OutTag> {
    /// Length of this device's address space in 512b pages.
    fn get_address_space_page_length(&self) -> usize;
    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than the device's page length will panic.
    fn read_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, tag: InTag)
        -> (Cycles, Result<(T, OutTag), Err>);
    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Write sizes larger than the device's page length will panic.
    fn write_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, tag: InTag)
        -> (Cycles, Result<OutTag, Err>);
}

pub trait VAXBus: Bus<()> {}

impl<T> VAXBus for T
    where T: Bus<()> {}
pub struct RAMBus {
    ram: Vec<u8>,
}

impl RAMBus {
    pub fn new(size: usize) -> RAMBus {
        RAMBus {
            ram: vec![0; size],
        }
    }

    pub fn ram(&self) -> &[u8] {
        &self.ram[..]
    }

    pub fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram[..]
    }
}

impl Bus<()> for RAMBus {
    const MAX_OPERATION_SIZE: usize = 16;
    const MAX_ADDRESS: usize = std::u32::MAX as usize;
    fn read_val<T: ByteRepr + Clone>(&mut self, addr: usize) -> (Cycles, Result<T, ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});
        
        if addr + T::BYTE_LEN > self.ram.len() {
            let dat = vec![0; T::BYTE_LEN];
            (cyc, Ok(T::from_le_bytes(&dat)))
        } else {
            let v = T::from_le_bytes(&self.ram[addr..addr+T::BYTE_LEN]);
            (cyc, Ok(v))
        }
    }
    fn write_val<T: ByteRepr + Clone>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});
        if addr + T::BYTE_LEN > self.ram.len() {
            (cyc, Ok(()))
        } else {
            data.copy_to_le_bytes(&mut self.ram[addr..addr+T::BYTE_LEN]);
            (cyc, Ok(()))
        }
    }
}

const UVAX31_ROM_BEGIN: usize = 0x2004_0000;
const UVAX31_ROM_END: usize = 0x2007_FFFF;

const UVAX31_RAM_BEGIN: usize = 0x0000_0000;
const UVAX31_RAM_END: usize = 0x01FF_FFFF;

const UVAX31_BOARD_REGS_BEGIN: usize = 0x2008_0000;
const UVAX31_BOARD_REGS_END: usize = 0x2008_0010;

#[derive(Copy, Clone, Debug)]
pub enum RAMSize {
    Size2MB,
    Size4MB,
    Size8MB,
    Size16MB,
    Size32MB,
}

pub struct MicroVAX3100Bus {
    boot_rom: &'static [u8],
    ram: Vec<u8>,
    ram_size: RAMSize,
}

#[derive(Copy, Clone, Debug)]
enum MicroVAXAddress {
    RAM(usize),
    BootROM(usize),
    KARegs(usize),
    Invalid,
}

impl MicroVAXAddress {
    pub fn match_addr<T:ByteRepr>(addr: usize) -> MicroVAXAddress {
        match (addr, addr.wrapping_sub(T::BYTE_LEN)) {
            (UVAX31_RAM_BEGIN..=UVAX31_RAM_END, UVAX31_RAM_BEGIN..=UVAX31_RAM_END)
                => MicroVAXAddress::RAM(addr - UVAX31_RAM_BEGIN),
            (UVAX31_ROM_BEGIN..=UVAX31_ROM_END, UVAX31_ROM_BEGIN..=UVAX31_ROM_END)
                => MicroVAXAddress::BootROM(addr - UVAX31_ROM_BEGIN),
            (UVAX31_BOARD_REGS_BEGIN..=UVAX31_BOARD_REGS_END, UVAX31_BOARD_REGS_BEGIN..=UVAX31_BOARD_REGS_END)
                => MicroVAXAddress::KARegs(addr - UVAX31_BOARD_REGS_BEGIN),
            _ => {
                println!("Unknown address {:#8x} accessed! (note this may also just be an access that crosses memory boundaries)", addr);
                MicroVAXAddress::Invalid
            }
        }
    }
}


impl Bus<()> for MicroVAX3100Bus {
    const MAX_OPERATION_SIZE: usize = 16;
    const MAX_ADDRESS: usize = std::u32::MAX as usize;
    fn read_val<T: ByteRepr + Clone>(&mut self, addr: usize) -> (Cycles, Result<T, ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});

        match MicroVAXAddress::match_addr::<T>(addr) {
            MicroVAXAddress::RAM(v) => {
                let s = &self.ram[v..(v+T::BYTE_LEN)];
                (cyc, Ok(T::from_le_bytes(s)))
            },
            MicroVAXAddress::BootROM(v) => {
                let s = &self.boot_rom[v..(v+T::BYTE_LEN)];
                (cyc, Ok(T::from_le_bytes(s)))
            },
            MicroVAXAddress::KARegs(_) => {
                // TODO
                let dat = vec![0xFF; T::BYTE_LEN];
                (cyc, Ok(T::from_le_bytes(&dat)))
            }
            MicroVAXAddress::Invalid => {
                let dat = vec![0xFF; T::BYTE_LEN];
                (cyc, Ok(T::from_le_bytes(&dat)))
            }
        }
    }
    fn write_val<T: ByteRepr>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});
        
        match MicroVAXAddress::match_addr::<T>(addr) {
            MicroVAXAddress::RAM(v) => {
                let s = &mut self.ram[v..(v+T::BYTE_LEN)];
                data.copy_to_le_bytes(s);
                (cyc, Ok(()))
            },
            MicroVAXAddress::BootROM(v) => {
                (cyc, Ok(()))
            },
            MicroVAXAddress::KARegs(_) => {
                // TODO
                (cyc, Ok(()))
            }
            MicroVAXAddress::Invalid => {
                (cyc, Ok(()))
            }
        }
    }
}

impl MicroVAX3100Bus {
    pub fn new(boot_rom: &'static [u8], ram_size: RAMSize) -> MicroVAX3100Bus {
        MicroVAX3100Bus {
            boot_rom,
            ram: vec![0; 2097152],
            ram_size,
        }
    }
}