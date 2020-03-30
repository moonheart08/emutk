use bytemuck::{
    Pod,
    from_bytes,
    bytes_of,
};
use std::mem::size_of;
use std::marker::PhantomData;

use crate::cycles::Cycles;

/// A generic bus error.
pub trait BusError: std::fmt::Debug + Clone {
    /// Emits an Out Of Bounds error.
    fn out_of_bounds(addr: usize) -> Self;
    /// Checks if the error is an Out Of Bound error.
    fn is_oob_error(&self) -> bool;
    /// Address the error occured on.
    fn get_triggering_address(&self) -> usize;
}

/// An emulated system bus that supports read and write operations.
pub trait Bus<Err: BusError> {
    /// Maximum size of a data read. Read sizes larger than this will panic.
    const MAX_OPERATION_SIZE: usize;
    /// Maximum value of Addr before the Bus returns an Out Of Bound error
    const MAX_ADDRESS: usize;

    /// Read a byte slice from the bus at the specified address.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read(&mut self, addr: usize, size: usize) -> (Cycles, Result<&[u8], Err>);
    /// Read a piece of data from the bus at the specified address.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read_val<T: Pod + Clone>(&mut self, addr: usize) -> (Cycles, Result<T, Err>);

    /// Write a byte slice to the bus at the specified address.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write(&mut self, addr: usize, data: &[u8]) -> (Cycles, Result<(), Err>);
    /// Write a piece of data to the bus at the specified address.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write_val<T: Pod + Clone>(&mut self, addr: usize, data: T) -> (Cycles, Result<(), Err>);
}

/// An extension to Bus that adds places to transport arbitrary data.
pub trait TaggedBus<Err: BusError, InTag, OutTag>: Bus<Err> {
    /// Read a byte slice from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read_tagged(&mut self, addr: usize, size: usize, tag: InTag)
        -> (Cycles, Result<(&[u8], OutTag), Err>);
    /// Read a piece of data from the bus at the specified address with tags.
    /// ## Panics
    /// Read sizes larger than MAX_OPERATION_SIZE will panic.
    fn read_val_tagged<T: Pod + Clone>(&mut self, addr: usize, size: usize, tag: InTag)
        -> (Cycles, Result<(T, OutTag), Err>);

    /// Write a byte slice to the bus at the specified address with tags.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write_tagged(&mut self, addr: usize, data: &[u8], tag: InTag) 
        -> (Cycles, Result<OutTag, Err>);
    /// Write a piece of data to the bus at the specified address with tags.
    /// ## Panics
    /// Write sizes larger than MAX_OPERATION_SIZE will panic.
    fn write_val_tagged<T: Pod + Clone>(&mut self, addr: usize, data: T, tag: InTag)
        -> (Cycles, Result<OutTag, Err>);
}