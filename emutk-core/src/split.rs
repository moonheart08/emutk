pub trait Splitable<T: Sized + Copy + Clone + Default>: Sized + Copy + Clone + private::Sealed {
    fn split_le(self) -> [T; 2];
    fn join_le(halves: [T; 2]) -> Self;

    fn split_be(self) -> [T; 2];
    fn join_be(halves: [T; 2]) -> Self;

    fn modify_lower_le(self, val: T) -> Self {
        let mut old = self.split_le();
        old[0] = val;
        Self::join_le(old)
    }

    fn modify_lower_be(self, val: T) -> Self {
        let mut old = self.split_be();
        old[0] = val;
        Self::join_be(old)
    }

    fn modify_upper_le(self, val: T) -> Self {
        let mut old = self.split_le();
        old[1] = val;
        Self::join_le(old)
    }

    fn modify_upper_be(self, val: T) -> Self {
        let mut old = self.split_be();
        old[0] = val;
        Self::join_be(old)
    }

    fn swap_halves(self) -> Self {
        let mut old = self.split_le();
        Self::join_be(old) // Endianness magic trick
    }
}

impl Splitable<u8> for u16 {
    fn split_le(self) -> [u8; 2] {
        self.to_le_bytes()
    }

    fn join_le(halves: [u8; 2]) -> Self {
        u16::from_le_bytes(halves)
    }

    fn split_be(self) -> [u8; 2] {
        self.to_be_bytes()
    }

    fn join_be(halves: [u8; 2]) -> Self {
        u16::from_be_bytes(halves)
    }
}

//TODO: Implement for u32, u64, and u128. Figure out how to make it "sealed" so it can't be implemented by others.

mod private {
    pub trait Sealed {}

    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}