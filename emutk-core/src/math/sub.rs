use crate::flags::OCZSFlagData;
trait FlaggedSub<Rhs = Self> {
    type Output;

    #[must_use]
    fn flagged_sub<T: OCZSFlagData>(self, other: Rhs, flags: T) -> (Self::Output, T);
}