use crate::flags::OCZSFlagData;
trait FlaggedDiv<Rhs = Self> {
    type Output;

    #[must_use]
    fn flagged_div<T: OCZSFlagData>(self, other: Rhs, flags: T) -> (Self::Output, T);
}