use emutk_core::{
    cycles::Cycles,
    bus::TaggedBus,
    bus::Bus,
    ByteRepr,
};


pub struct VirtVAXBus {
    boot_rom: &'static [u8],
    ram: Vec<u8>,
    devices: Vec<Box<dyn Device>>,
}

pub enum AddressSpace {
    RAM,     // Space 0x0
    ROM,     // Space 0x1
    Display, // Space 0x2
    Devices, // Space 0x3
}

pub enum DeviceOrigin {
    Native,
    JNI,
}

pub trait Device {
    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn read_u32(&mut self, addr: usize)
        -> (Cycles, u32);

    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn write_u32(&mut self, addr: usize, data: u32)
        -> Cycles;

    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn read_u16(&mut self, addr: usize)
        -> (Cycles, u16);

    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn write_u16(&mut self, addr: usize, data: u16)
        -> Cycles;
    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn read_u8(&mut self, addr: usize)
        -> (Cycles, u8);

    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than 64KiB will panic.
    fn write_u8(&mut self, addr: usize, data: u8)
        -> Cycles;


    fn device_active(&mut self) -> bool;

    fn device_origin(&mut self) -> DeviceOrigin;

    fn interrupt_pending(&mut self) -> bool;

    fn tick(&mut self);

    fn serialize(&mut self, space: &mut [u8]);
}


impl VirtVAXBus {
    pub fn new(boot_rom: &'static [u8], ram_size: usize) -> VirtVAXBus {
        VirtVAXBus {
            boot_rom,
            ram: vec![0; ram_size],
            devices: vec![],
        }
    }
}

impl Bus<()> for VirtVAXBus {
    const MAX_OPERATION_SIZE: usize = 16;
    const MAX_ADDRESS: usize = std::u32::MAX as usize;
    fn read_val<T: ByteRepr + Clone>(&mut self, addr: usize) -> (Cycles, Result<T, ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});
        
        let res = match addr >> 28 {
            0x0 => {
                if addr + T::BYTE_LEN < self.ram.len() {
                    if addr > 0x00FF {
                        //println!("RE: {}", addr);
                    }
                    Ok(T::from_le_bytes(&self.ram[addr..addr+T::BYTE_LEN]))
                } else {
                    //println!("ack");
                    Ok(T::zeroed())
                }
            },
            0x1 => {
                let offs_addr = (addr - 0x1000_0000);
                if offs_addr + T::BYTE_LEN < self.boot_rom.len() {
                    Ok(T::from_le_bytes(&self.boot_rom[offs_addr..offs_addr+T::BYTE_LEN]))
                } else {
                    Ok(T::zeroed())
                }
            },
            0x2 => {
                Ok(T::zeroed())
            },
            0x3 => {
                Ok(T::zeroed())
            },
            _ => Ok(T::zeroed()),
        };
        (cyc, res)
    }
    fn write_val<T: ByteRepr>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), ()>) {
        let cyc = Cycles(if T::BYTE_LEN < 4 {1} else {T::BYTE_LEN/4});

        match addr >> 28 {
            0x0 => {
                if addr > 0x00FF {
                    //println!("WR: {}", addr);
                }
                if addr + T::BYTE_LEN < self.ram.len() {
                    data.copy_to_le_bytes(&mut self.ram[addr..addr+T::BYTE_LEN])
                } 
            },
            
            _ => {},
        };
        (cyc, Ok(()))
    }
}