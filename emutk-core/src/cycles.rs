/// The number of cycles an operation took.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cycles(pub usize);

impl std::ops::AddAssign for Cycles {
    
    fn add_assign(&mut self, rhs: Self) {
        *self = Cycles(self.0 + rhs.0);
    }
}