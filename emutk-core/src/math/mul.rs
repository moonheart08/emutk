use crate::flags::OCZSFlagData;
trait FlaggedMul<Rhs = Self> {
    type Output;

    #[must_use]
    fn flagged_mul<T: OCZSFlagData>(self, other: Rhs, flags: T) -> (Self::Output, T);
}