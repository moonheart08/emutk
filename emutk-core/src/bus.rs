use crate::bytes::ByteRepr;

pub trait Bus {
    type Tag;

    /// Read a ByteRepr implementing value from the bus at the selected address.
    fn read<T: ByteRepr>(&mut self, addr: usize) -> T;
    /// Write a ByteRepr implementing value to the bus at the selected address.
    fn write<T: ByteRepr>(&mut self, addr: usize, data: T);

    /// Read a ByteRepr implementing value from the bus at the selected address.
    fn read_tagged<T: ByteRepr>(&mut self, addr: usize, tag: Self::Tag) -> T;
    /// Write a ByteRepr implementing value to the bus at the selected address.
    fn write_tagged<T: ByteRepr>(&mut self, addr: usize, data: T, tag: Self::Tag);
}