use byterepr::ByteRepr;
use std::mem::size_of;
use std::marker::PhantomData;

use crate::cycles::Cycles;

/// An emulated bus that supports read and write operations. 
/// Not necessarily just the system bus, can be devices too.
pub trait Bus<Err> {
    /// Maximum size of a data read. Read sizes larger than this will panic.
    const MAX_OPERATION_SIZE: usize;
    /// Maximum value of an address. Any higher is a logic error and will panic.
    const MAX_ADDRESS: usize;

    /// Read a piece of data from the bus at the specified address.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read_val<T: ByteRepr>(&mut self, addr: usize) -> (Cycles, Result<T, Err>);
    /// Write a piece of data to the bus at the specified address.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write_val<T: ByteRepr>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), Err>);
}

/// An alternative to Bus that adds places to transport arbitrary data.
/// Not necessarily just the system bus, can be devices too.
/// Should not be combined with Bus, as 99% of the time the tag is important
/// to emulated system operation. Choose one or the other.
pub trait TaggedBus<Err, InTag, OutTag> {
    /// Maximum size of a data read. Read sizes larger than this will panic.
    const MAX_OPERATION_SIZE: usize;
    /// Maximum value of an address. Any higher is a logic error and will panic.
    const MAX_ADDRESS: usize;
    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, tag: InTag)
        -> (Cycles, Result<(T, OutTag), Err>);
    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write_val_tagged<T: ByteRepr + Clone>(&mut self, addr: usize, data: T, tag: InTag)
        -> (Cycles, Result<OutTag, Err>);
}