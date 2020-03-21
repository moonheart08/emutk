use crate::flags::OCZSFlagData;
trait FlaggedAdd<Rhs = Self> {
    type Output;

    #[must_use]
    fn flagged_add<T: OCZSFlagData>(self, other: Rhs, flags: T) -> (Self::Output, T);
}