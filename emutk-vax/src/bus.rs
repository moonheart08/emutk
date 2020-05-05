use emutk_core::{
    cycles::Cycles,
    bus::TaggedBus,
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

pub trait VAXBus: TaggedBus<(), (), ()> {}

impl<T> VAXBus for T
    where T: TaggedBus<(), (), ()> {}
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

impl TaggedBus<(), (), ()> for RAMBus {
    const MAX_OPERATION_SIZE: usize = 16;
    const MAX_ADDRESS: usize = std::u32::MAX as usize;
    fn read_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, _tag: ()) -> (Cycles, Result<(T, ()), ()>) {
        let cyc = Cycles(T::BYTE_LEN/4);
        
        if addr + T::BYTE_LEN > self.ram.len() {
            let dat = vec![0; T::BYTE_LEN];
            (cyc, Ok((T::from_le_bytes(&dat), ())))
        } else {
            let v = T::from_le_bytes(&self.ram[addr..addr+T::BYTE_LEN]);
            (cyc, Ok((v, ())))
        }
    }
    fn write_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, _tag: ()) -> (Cycles, Result<(), ()>) {
        let cyc = Cycles(T::BYTE_LEN/4);
        if addr + T::BYTE_LEN > self.ram.len() {
            (cyc, Ok(()))
        } else {
            data.copy_to_le_bytes(&mut self.ram[addr..addr+T::BYTE_LEN]);
            (cyc, Ok(()))
        }
    }
}