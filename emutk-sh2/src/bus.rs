use emutk_core::{
    bus::{
        Bus,
    },
    cycles::Cycles,
    ByteRepr,
};

pub struct RAMBus(Vec<u8>);

impl RAMBus {
    pub fn new() -> Self {
        RAMBus(vec![0;65535])
    }
}

impl Bus<()> for RAMBus {
    const MAX_OPERATION_SIZE: usize = 8;
    const MAX_ADDRESS: usize = 65535;

    fn read_val<T: ByteRepr + Clone>(&mut self, addr: usize) -> (Cycles, Result<T,()>) {
        let cycles = Cycles(std::mem::size_of::<T>());

        let sl = &self.0[addr..std::mem::size_of::<T>()];
        (cycles, Ok(T::from_le_bytes(sl)))
    }

    fn write_val<T: ByteRepr + Clone>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), ()>) {
        let cycles = Cycles(std::mem::size_of::<T>());
        let sl = &mut self.0[addr..std::mem::size_of::<T>()];
        data.copy_to_le_bytes(sl);
        (cycles, Ok(()))
    }
}