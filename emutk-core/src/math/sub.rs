use crate::flags::OCZSFlagData;
trait FlaggedSub<T: OCZSFlagData, Rhs = Self> {
    type Output;

    #[must_use]
    fn flagged_sub(self, other: Rhs, flags: T) -> (Self::Output, T);
}
impl<T: OCZSFlagData> FlaggedSub<T> for u8 {
    type Output = Self;

    #[must_use]
    fn flagged_sub(self, other: Self, mut flags: T) -> (Self::Output, T) {
        let (val, carry) = self.overflowing_sub(other);
        let (_, overflow) = (self as i8).overflowing_sub(other as i8);
        let half_carry = ((((self & 0xf) | 0x10).wrapping_sub(other & 0xf)) & 0x10) == 0x0; // half carry magic
        let half_overflow = ((((self & 0x7) | 0x8).wrapping_sub(other & 0x7)) & 0x8) == 0x0; // half overflow magic
        flags.set_carry_u4(half_carry);
        flags.set_overflow_u4(half_overflow);

        flags.set_sign_u4(val & 0x8 != 0);
        flags.set_zero_u4(val & 0xF == 0);

        flags.set_carry_u8(carry);
        flags.set_overflow_u8(overflow); // make sure to set the most significant one last.

        flags.set_sign_u8((val as i8).signum() == -1);
        flags.set_zero_u8(val == 0);

        (val, flags)
    }
}

impl<T: OCZSFlagData> FlaggedSub<T> for u16 {
    type Output = Self;

    #[must_use]
    fn flagged_sub(self, other: Self, flags: T) -> (Self::Output, T) {
        let (val, carry) = self.overflowing_sub(other);
        let (_, overflow) = (self as i16).overflowing_sub(other as i16);
        let (_, mut flags) = (self as u8).flagged_sub(other as u8, flags);
        flags.set_carry_u16(carry);
        flags.set_overflow_u16(overflow);
        flags.set_sign_u16((val as i16).signum() == -1);
        flags.set_zero_u16(val == 0);
        (val, flags)
    }
}

impl<T: OCZSFlagData> FlaggedSub<T> for u32 {
    type Output = Self;

    #[must_use]
    fn flagged_sub(self, other: Self, flags: T) -> (Self::Output, T) {
        let (val, carry) = self.overflowing_sub(other);
        let (_, overflow) = (self as i32).overflowing_sub(other as i32);
        let (_, mut flags) = (self as u16).flagged_sub(other as u16, flags);
        flags.set_carry_u32(carry);
        flags.set_overflow_u32(overflow);
        flags.set_sign_u32((val as i32).signum() == -1);
        flags.set_zero_u32(val == 0);
        (val, flags)
    }
}

impl<T: OCZSFlagData> FlaggedSub<T> for u64 {
    type Output = Self;

    #[must_use]
    fn flagged_sub(self, other: Self, flags: T) -> (Self::Output, T) {
        let (val, carry) = self.overflowing_sub(other);
        let (_, overflow) = (self as i64).overflowing_sub(other as i64);
        let (_, mut flags) = (self as u32).flagged_sub(other as u32, flags);
        flags.set_carry_u64(carry);
        flags.set_overflow_u64(overflow);
        flags.set_sign_u64((val as i64).signum() == -1);
        flags.set_zero_u64(val == 0);
        (val, flags)
    }
}


impl<T: OCZSFlagData> FlaggedSub<T> for u128 {
    type Output = Self;

    #[must_use]
    fn flagged_sub(self, other: Self, flags: T) -> (Self::Output, T) {
        let (val, carry) = self.overflowing_sub(other);
        let (_, overflow) = (self as i128).overflowing_sub(other as i128);
        let (_, mut flags) = (self as u64).flagged_sub(other as u64, flags);
        flags.set_carry_u64(carry);
        flags.set_overflow_u64(overflow);
        flags.set_sign_u64((val as i128).signum() == -1);
        flags.set_zero_u64(val == 0);
        (val, flags)
    }
}