/// Trait that defines ways to set and get a emulated CPU's carry flag.
/// # Implementing
/// The majority of implementors do not need to have a spot to store all of the flags, 
/// and can instead implement the extra flag gets and sets as just setting the single unified carry.
pub trait CarryFlagData {
    /// Check for 4 bit carry.
    fn get_carry_u4(&self) -> bool;
    /// Check for 8 bit carry.
    fn get_carry_u8(&self) -> bool;
    /// Check for 16 bit carry.
    fn get_carry_u16(&self) -> bool;
    /// Check for 32 bit carry.
    fn get_carry_u32(&self) -> bool;
    /// Check for 64 bit carry.
    fn get_carry_u64(&self) -> bool;
    /// Check for 128 bit carry.
    fn get_carry_u128(&self) -> bool;

    /// Set 4 bit carry.
    fn set_carry_u4(&mut self, val: bool);
    /// Set 8 bit carry.
    fn set_carry_u8(&mut self, val: bool);
    /// Set 16 bit carry.
    fn set_carry_u16(&mut self, val: bool);
    /// Set 32 bit carry.
    fn set_carry_u32(&mut self, val: bool);
    /// Set 64 bit carry.
    fn set_carry_u64(&mut self, val: bool);
    /// Set 128 bit carry.
    fn set_carry_u128(&mut self, val: bool);
}

/// Many architectures do not need the extra flexibility provided by CarryFlagData,
/// as they only have a single carry flag. This supplements it with a unified set and get.
pub trait ReducedCarryFlagData: CarryFlagData {
    /// Get the carry flag.
    fn get_carry(&self) -> bool;

    /// Set the carry flag.
    fn set_carry(&mut self, val: bool);
}

/// Trait that defines ways to set and get a emulated CPU's overflow flag.
/// # Implementing
/// The majority of implementors do not need to have a spot to store all of the flags, 
/// and can instead implement the extra flag gets and sets as just setting the single unified carry.
pub trait OverflowFlagData {
    /// Check for 4 bit overflow.
    fn get_overflow_u4(&self) -> bool;
    /// Check for 8 bit overflow.
    fn get_overflow_u8(&self) -> bool;
    /// Check for 16 bit overflow.
    fn get_overflow_u16(&self) -> bool;
    /// Check for 32 bit overflow.
    fn get_overflow_u32(&self) -> bool;
    /// Check for 64 bit overflow.
    fn get_overflow_u64(&self) -> bool;
    /// Check for 128 bit overflow.
    fn get_overflow_u128(&self) -> bool;

    /// Set 4 bit overflow.
    fn set_overflow_u4(&mut self, val: bool);
    /// Set 8 bit overflow.
    fn set_overflow_u8(&mut self, val: bool);
    /// Set 16 bit overflow.
    fn set_overflow_u16(&mut self, val: bool);
    /// Set 32 bit overflow.
    fn set_overflow_u32(&mut self, val: bool);
    /// Set 64 bit overflow.
    fn set_overflow_u64(&mut self, val: bool);
    /// Set 128 bit overflow.
    fn set_overflow_u128(&mut self, val: bool);
}

/// Many architectures do not need the extra flexibility provided by OverflowFlagData,
/// as they only have a single overflow flag. This supplements it with a unified set and get.
pub trait ReducedOverflowFlagData: OverflowFlagData {
    /// Get the overflow flag.
    fn get_overflow(&self) -> bool;

    /// Set the overflow flag.
    fn set_overflow(&mut self, val: bool);
}

/// Trait that defines ways to set and get a emulated CPU's parity flag.
/// # Implementing
/// The majority of implementors do not need to have a spot to store all of the flags, 
/// and can instead implement the extra flag gets and sets as just setting the single unified carry.
pub trait ParityFlagData {
    /// Check for 4 bit parity.
    fn get_parity_u4(&self) -> bool;
    /// Check for 8 bit parity.
    fn get_parity_u8(&self) -> bool;
    /// Check for 16 bit parity.
    fn get_parity_u16(&self) -> bool;
    /// Check for 32 bit parity.
    fn get_parity_u32(&self) -> bool;
    /// Check for 64 bit parity.
    fn get_parity_u64(&self) -> bool;
    /// Check for 128 bit parity.
    fn get_parity_u128(&self) -> bool;

    /// Set 4 bit parity.
    fn set_parity_u4(&mut self, val: bool);
    /// Set 8 bit parity.
    fn set_parity_u8(&mut self, val: bool);
    /// Set 16 bit parity.
    fn set_parity_u16(&mut self, val: bool);
    /// Set 32 bit parity.
    fn set_parity_u32(&mut self, val: bool);
    /// Set 64 bit parity.
    fn set_parity_u64(&mut self, val: bool);
    /// Set 128 bit parity.
    fn set_parity_u128(&mut self, val: bool);
}

/// Many architectures do not need the extra flexibility provided by ParityFlagData,
/// as they only have a single parity flag. This supplements it with a unified set and get.
pub trait ReducedParityFlagData: ParityFlagData {
    /// Get the parity flag.
    fn get_parity(&self) -> bool;

    /// Set the parity flag.
    fn set_parity(&mut self, val: bool);
}

/// Trait that defines ways to set and get a emulated CPU's zero flag.
/// # Implementing
/// The majority of implementors do not need to have a spot to store all of the flags, 
/// and can instead implement the extra flag gets and sets as just setting the single unified carry.
pub trait ZeroFlagData {
    /// Check for 4 bit zero.
    fn get_zero_u4(&self) -> bool;
    /// Check for 8 bit zero.
    fn get_zero_u8(&self) -> bool;
    /// Check for 16 bit zero.
    fn get_zero_u16(&self) -> bool;
    /// Check for 32 bit zero.
    fn get_zero_u32(&self) -> bool;
    /// Check for 64 bit zero.
    fn get_zero_u64(&self) -> bool;
    /// Check for 128 bit zero.
    fn get_zero_u128(&self) -> bool;

    /// Set 4 bit zero.
    fn set_zero_u4(&mut self, val: bool);
    /// Set 8 bit zero.
    fn set_zero_u8(&mut self, val: bool);
    /// Set 16 bit zero.
    fn set_zero_u16(&mut self, val: bool);
    /// Set 32 bit zero.
    fn set_zero_u32(&mut self, val: bool);
    /// Set 64 bit zero.
    fn set_zero_u64(&mut self, val: bool);
    /// Set 128 bit zero.
    fn set_zero_u128(&mut self, val: bool);
}

/// Many architectures do not need the extra flexibility provided by ZeroFlagData,
/// as they only have a single zero flag. This supplements it with a unified set and get.
pub trait ReducedZeroFlagData: ZeroFlagData {
    /// Get the zero flag.
    fn get_zero(&self) -> bool;

    /// Set the zero flag.
    fn set_zero(&mut self, val: bool);
}

/// Trait that defines ways to set and get a emulated CPU's sign flag.
/// # Implementing
/// The majority of implementors do not need to have a spot to store all of the flags, 
/// and can instead implement the extra flag gets and sets as just setting the single unified carry.
pub trait SignFlagData {
    /// Check for 4 bit sign.
    fn get_sign_u4(&self) -> bool;
    /// Check for 8 bit sign.
    fn get_sign_u8(&self) -> bool;
    /// Check for 16 bit sign.
    fn get_sign_u16(&self) -> bool;
    /// Check for 32 bit sign.
    fn get_sign_u32(&self) -> bool;
    /// Check for 64 bit sign.
    fn get_sign_u64(&self) -> bool;
    /// Check for 128 bit sign.
    fn get_sign_u128(&self) -> bool;

    /// Set 4 bit sign.
    fn set_sign_u4(&mut self, val: bool);
    /// Set 8 bit sign.
    fn set_sign_u8(&mut self, val: bool);
    /// Set 16 bit sign.
    fn set_sign_u16(&mut self, val: bool);
    /// Set 32 bit sign.
    fn set_sign_u32(&mut self, val: bool);
    /// Set 64 bit sign.
    fn set_sign_u64(&mut self, val: bool);
    /// Set 128 bit sign.
    fn set_sign_u128(&mut self, val: bool);
}

/// Many architectures do not need the extra flexibility provided by SignFlagData,
/// as they only have a single sign flag. This supplements it with a unified set and get.
pub trait ReducedSignFlagData: SignFlagData {
    /// Get the sign flag.
    fn get_sign(&self) -> bool;

    /// Set the sign flag.
    fn set_sign(&mut self, val: bool);
}