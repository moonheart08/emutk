use crate::bytes::ByteRepr;
use std::marker::PhantomData;


pub trait Bus<Err, I> {
    type Tag;

    /// Read a ByteRepr implementing value from the bus at the selected address.
    fn read<T: ByteRepr>(&mut self, addr: usize) -> Result<T, Err>;
    /// Write a ByteRepr implementing value to the bus at the selected address.
    fn write<T: ByteRepr>(&mut self, addr: usize, data: T) -> Result<(), Err>;

    /// Read a ByteRepr implementing value from the bus at the selected address, with a tag.
    fn read_tagged<T: ByteRepr>(&mut self, addr: usize, _tag: Self::Tag) -> Result<T, Err> {
        self.read(addr)
    }
    /// Write a ByteRepr implementing value to the bus at the selected address, with a tag.
    fn write_tagged<T: ByteRepr>(&mut self, addr: usize, data: T, _tag: Self::Tag) -> Result<(), Err> {
        self.write(addr, data)
    }

    fn execute_cycle(&mut self) -> I;
}

pub trait BusErrorCommon {
    fn out_of_bounds() -> Self;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SimpleBusError {
    InvalidRead,
    InvalidWrite,
    OutOfBounds,
}

impl BusErrorCommon for SimpleBusError {
    fn out_of_bounds() -> Self {
        SimpleBusError::OutOfBounds
    }
}

/// Very simple little endian bus with 32KiB of RAM and 32KiB of ROM, ment for testing.
pub struct RAMROMBusBE<Err> {
    ram: Vec<u8>,
    rom: Vec<u8>,
    __err: std::marker::PhantomData<Err>,
}

impl<E> RAMROMBusBE<E> {
    pub fn new<Err>() -> RAMROMBusBE<Err> {
        RAMROMBusBE {
            ram: vec![0; 32768],
            rom: vec![0; 32768],
            __err: PhantomData,
        }
    }
}

impl<E: BusErrorCommon> Bus<E, ()> for RAMROMBusBE<E> {
    type Tag = ();

    fn read<T: ByteRepr>(&mut self, addr: usize) -> Result<T, E> {
        let len = T::BYTE_LEN;

        if addr + len > u16::MAX as usize {
            return Err(E::out_of_bounds());
        } else {
            let v: T = match addr {
                n if n < 32768 - len => {
                    let s = &self.ram[addr..len];
                    T::from_be_bytes(s)
                } 
                n if n > 32768 => {
                    let s = &self.rom[addr..len];
                    T::from_be_bytes(s)
                }
                _ => {
                    let x = &self.ram[addr..32768];
                    let y = &self.rom[0..(len - x.len())];
                    let f = [x, y].concat();
                    T::from_be_bytes(&f)
                }
            };
            return Ok(v);
        }
    }

    fn write<T: ByteRepr>(&mut self, addr: usize, data: T) -> Result<(), E> {
        let len = T::BYTE_LEN;
        if addr + len > u16::MAX as usize {
            return Err(E::out_of_bounds());
        } else {
            match addr {
                n if n < 32768 - len => {
                    let s = &mut self.ram[addr..len];
                    data.copy_into_be_bytes( s);
                } 
                n if n > 32768 => {
                    let s = &mut self.rom[addr..len];
                    data.copy_into_be_bytes(s);
                }
                _ => {
                    let x = &mut self.ram[addr..32768];
                    let y = &mut self.rom[0..(len - x.len())];
                    
                    let mut v = vec![0; len];
                    data.copy_into_be_bytes(&mut v);

                    x.copy_from_slice(&v[0..x.len()]);
                    y.copy_from_slice(&v[x.len()..v.len()]);
                }
            };
            return Ok(());
        }
    }
    
    fn execute_cycle(&mut self) -> () {
        // Do nothing, this bus doesn't do any processing.
    }
}


/// Very simple little endian bus with 32KiB of RAM and 32KiB of ROM, ment for testing.
pub struct RAMROMBusLE<Err> {
    ram: Vec<u8>,
    rom: Vec<u8>,
    __err: std::marker::PhantomData<Err>,
}

impl<E> RAMROMBusLE<E> {
    pub fn new<Err>() -> RAMROMBusLE<Err> {
        RAMROMBusLE {
            ram: vec![0; 32768],
            rom: vec![0; 32768],
            __err: PhantomData,
        }
    }
}

impl<E: BusErrorCommon> Bus<E, ()> for RAMROMBusLE<E> {
    type Tag = ();

    fn read<T: ByteRepr>(&mut self, addr: usize) -> Result<T, E> {
        let len = T::BYTE_LEN;

        if addr + len > u16::MAX as usize {
            return Err(E::out_of_bounds());
        } else {
            let v: T = match addr {
                n if n < 32768 - len => {
                    let s = &self.ram[addr..len];
                    T::from_le_bytes(s)
                } 
                n if n > 32768 => {
                    let s = &self.rom[addr..len];
                    T::from_le_bytes(s)
                }
                _ => {
                    let x = &self.ram[addr..32768];
                    let y = &self.rom[0..(len - x.len())];
                    let f = [x, y].concat();
                    T::from_le_bytes(&f)
                }
            };
            return Ok(v);
        }
    }

    fn write<T: ByteRepr>(&mut self, addr: usize, data: T) -> Result<(), E> {
        let len = T::BYTE_LEN;
        if addr + len > u16::MAX as usize {
            return Err(E::out_of_bounds());
        } else {
            match addr {
                n if n < 32768 - len => {
                    let s = &mut self.ram[addr..len];
                    data.copy_into_le_bytes( s);
                } 
                n if n > 32768 => {
                    let s = &mut self.rom[addr..len];
                    data.copy_into_le_bytes(s);
                }
                _ => {
                    let x = &mut self.ram[addr..32768];
                    let y = &mut self.rom[0..(len - x.len())];
                    
                    let mut v = vec![0; len];
                    data.copy_into_le_bytes(&mut v);

                    x.copy_from_slice(&v[0..x.len()]);
                    y.copy_from_slice(&v[x.len()..v.len()]);
                }
            };
            return Ok(());
        }
    }

    fn execute_cycle(&mut self) -> () {
        // Do nothing, this bus doesn't do any processing.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_ramrom_bus_boundaries() {
        let mut bus: RAMROMBusLE<SimpleBusError> = RAMROMBusLE::<SimpleBusError>::new(); 

        assert_eq!(bus.write::<u32>(32766, 0x1234_5678), Ok(()));

        assert_eq!(bus.read::<u32>(32766), Ok(0x1234_5678));
    }
}