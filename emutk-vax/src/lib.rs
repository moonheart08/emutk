#![feature(test)]

extern crate test;
use test::Bencher;

pub mod operands;
#[bench]
fn test_decode_times(b: &mut Bencher) {
    b.iter(|| {
        let op =  test::black_box(0x9F);
        operands::OperandMode::identify_operand(op)
    })
}