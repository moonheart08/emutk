pub mod bus;
pub mod bytes;

pub use bus::Bus;
pub use bytes::ByteRepr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
