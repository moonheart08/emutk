pub mod bus;
pub mod split;
pub mod math;
pub mod flags;
pub mod cycles;

pub use byterepr::{
    ByteRepr,
    ByteReprNum,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
