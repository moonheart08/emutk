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