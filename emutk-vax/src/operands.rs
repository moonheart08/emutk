pub enum OperandMode {
    Literal(u8), // 0..=3
    Indexed(u8), // 4
    Register(u8), // 5
    RegisterDeferred(u8), // 6
    Autodecrement(u8), // 7
    Autoincrement(u8), // 8
    Immediate(), // 8F
    AutoincrementDeferred(u8), // 9
    Absolute(), // 9F

    ByteDisplacement(u8), // 10
    ByteDisplacementDeferred(u8), // 11
    WordDisplacement(u8), // 12
    WordDisplacementDeferred(u8), // 13
    LongwordDisplacement(u8), // 14
    LongwordDisplacementDeferred(u8), // 15

}

impl OperandMode {
    pub fn identify_operand(op_head: u8) -> OperandMode {
        let reg = op_head & 0xF;
        match (op_head & 0xF0) >> 4 {
            0..=3 => OperandMode::Literal(op_head & 0b1100_0000),
            4 => OperandMode::Indexed(reg),
            5 => OperandMode::Register(reg),
            6 => OperandMode::RegisterDeferred(reg),
            7 => OperandMode::Autodecrement(reg),
            8 if reg != 0xF => OperandMode::Autoincrement(reg),
            8 /* if reg == 0xF */ => OperandMode::Immediate(),
            9 if reg != 0xF => OperandMode::AutoincrementDeferred(reg),
            9 /* if reg == 0xF */ => OperandMode::Absolute(),
            10 => OperandMode::ByteDisplacement(reg),
            11 => OperandMode::ByteDisplacementDeferred(reg),
            12 => OperandMode::WordDisplacement(reg),
            13 => OperandMode::WordDisplacementDeferred(reg),
            14 => OperandMode::LongwordDisplacement(reg),
            15 => OperandMode::LongwordDisplacementDeferred(reg),
            16..=u8::MAX => unreachable!(),
        }
    }
}

mod tests {
    use super::*;

    
}