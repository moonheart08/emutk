pub fn calc_u128_carry_overflow(a: u128, b: u128) -> (bool, bool) {
    let (_, overflow) = a.overflowing_add(b);
    let (_, carry) = (a as i128).overflowing_add(b as i128);
    (carry, overflow)
}

pub fn calc_u64_carry_overflow(a: u64, b: u64) -> (bool, bool) {
    let (_, overflow) = a.overflowing_add(b);
    let (_, carry) = (a as i64).overflowing_add(b as i64);
    (carry, overflow)
}

pub fn calc_u32_carry_overflow(a: u32, b: u32) -> (bool, bool) {
    let (_, overflow) = a.overflowing_add(b);
    let (_, carry) = (a as i32).overflowing_add(b as i32);
    (carry, overflow)
}

pub fn calc_u16_carry_overflow(a: u16, b: u16) -> (bool, bool) {
    let (_, overflow) = a.overflowing_add(b);
    let (_, carry) = (a as i16).overflowing_add(b as i16);
    (carry, overflow)
}

pub fn calc_u8_carry_overflow(a: u8, b: u8) -> (bool, bool) {
    let (_, overflow) = a.overflowing_add(b);
    let (_, carry) = (a as i8).overflowing_add(b as i8);
    (carry, overflow)
}

pub fn calc_u128_parity(mut x: u128) -> bool {
    x ^= x >> 64;
    x ^= x >> 32;
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    return ((!x) & 1) != 0;
}

pub fn calc_u64_parity(mut x: u64) -> bool {
    x ^= x >> 32;
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    return (!x) & 1 != 0;
}

pub fn calc_u32_parity(mut x: u32) -> bool {
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    return ((!x) & 1) != 0;
}

pub fn calc_u16_parity(mut x: u16) -> bool {
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    return ((!x) & 1) != 0;
}

pub fn calc_u8_parity(mut x: u8) -> bool {
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;
    return ((!x) & 1) != 0;
}

