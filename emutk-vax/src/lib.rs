pub mod operands;
pub mod instructiontypes;
pub mod cpu;
pub mod bus;
pub mod mmu;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DataSize {
    Byte     = 1, // 8b
    Word     = 2, // 16b
    Longword = 3, // 32b
    Quadword = 4, // 64b
    Octaword = 5, // 128b
}

impl DataSize {
    #[inline]
    pub fn byte_len(self) -> usize {
        1 << (self as u8) // Algorithm magic :D
        /*
        use crate::DataSize::*;
        match self {
            Byte => 1,
            Word => 2,
            Longword => 4,
            Quadword => 8,
            Octaword => 16,
        }
        */
    }

    #[inline]
    pub fn bit_len(self) -> usize {
        self.byte_len() * 8
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TaggedData {
    Byte(u8),
    Word(u16),
    Longword(u32),
    Quadword(u64),
    Octaword(u128),
}