use emutk_core::{
    cycles::Cycles,
    bus::TaggedBus,
    ByteRepr,
};

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