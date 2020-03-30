use std::num::NonZeroUsize;
use crate::*;
use cpu::VAXCPU;
use emutk_core::cycles::Cycles;
use std::num::Wrapping;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperandMode {
    Literal(u8), // 0..=3
    // 4
    IndexedRegisterDeferred(u8, u8),
    IndexedAutodecrement(u8, u8),
    IndexedAutoincrement(u8, u8),
    IndexedAutoincrementDeferred(u8, u8),
    IndexedByteDisplacement(u8, u8),
    IndexedByteDisplacementDeferred(u8, u8),
    IndexedWordDisplacement(u8, u8),
    IndexedWordDisplacementDeferred(u8, u8),
    IndexedLongwordDisplacement(u8, u8),
    IndexedLongwordDisplacementDeferred(u8, u8),
    IndexedAbsolute(u8),

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
    pub fn identify_operand(op_head: [u8; 2]) -> Option<OperandMode> {
        let reg = op_head[0] & 0xF;
        
        Some(match (op_head[0] & 0xF0) >> 4 {
            0..=3 => OperandMode::Literal(op_head[0] & 0b1100_0000),
            4 => {
                let indexed_reg = op_head[1] & 0xF;
                let indexed_op = (op_head[1] & 0xF0) >> 4;
                match indexed_op {
                    6 => OperandMode::IndexedRegisterDeferred(reg, indexed_reg),
                    7 => OperandMode::IndexedAutodecrement(reg, indexed_reg),
                    8 => OperandMode::IndexedAutoincrement(reg, indexed_reg),
                    9 if indexed_reg != 0xF =>
                        OperandMode::IndexedAbsolute(reg),
                    9 => OperandMode::IndexedAutoincrementDeferred(reg, indexed_reg),
                    10 => OperandMode::IndexedByteDisplacement(reg, indexed_reg),
                    11 => OperandMode::IndexedByteDisplacementDeferred(reg, indexed_reg),
                    12 => OperandMode::IndexedWordDisplacement(reg, indexed_reg),
                    13 => OperandMode::IndexedWordDisplacementDeferred(reg, indexed_reg),
                    14 => OperandMode::IndexedLongwordDisplacement(reg, indexed_reg),
                    15 => OperandMode::IndexedLongwordDisplacementDeferred(reg, indexed_reg),
                    _ => return None,
                }
            },
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
        })
    }

    pub fn byte_size(self) -> usize {
        use OperandMode::*;
        match self {
            Literal(_) | Register(_) | RegisterDeferred(_) |
            Autodecrement(_) | Autoincrement(_) | AutoincrementDeferred(_) =>
                1, // returns an Option. always Some.
            Immediate() =>
                1, // Not a lie.
            Absolute() => 
                1,
            ByteDisplacement(_) | ByteDisplacementDeferred(_) =>
                1,
            WordDisplacement(_) | WordDisplacementDeferred(_) =>
                1,
            LongwordDisplacement(_) | LongwordDisplacementDeferred(_) =>
                1,
            
            IndexedRegisterDeferred(_, _) | IndexedAutodecrement(_, _) |
            IndexedAutoincrement(_, _) | IndexedAutoincrementDeferred(_, _) 
            => 2,

            IndexedByteDisplacement(_, _) | IndexedByteDisplacementDeferred(_, _)
                => 2,
            IndexedWordDisplacement(_, _) | IndexedWordDisplacementDeferred(_, _)
                => 2,
            IndexedLongwordDisplacement(_, _) | IndexedLongwordDisplacementDeferred(_, _)
                => 2,

            IndexedAbsolute(_) => 2,
        }
    }

    pub fn pc_add_amnt(self, d: DataSize) -> usize {
        match self {
            _ => self.byte_size()
        }
    }

    pub fn load_u8(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU) -> (Cycles, Result<u8, ()>) {
        use OperandMode::*;
        match self {
            Literal(v) => (Cycles(0), Ok(v as u8)),
            Register(v) => (Cycles(0), Ok(gpr[v as usize].0 as u8)),
            RegisterDeferred(v) => {
                cpu.read_val::<u8>(gpr[v as usize].0 as usize)
            }
            Immediate() => {
                let v = cpu.read_val::<u8>((*pc).0 as usize);
                *pc += Wrapping(1);
                v
            }

            _ => todo!(),
        }
    }

    pub fn load_u16(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU) -> (Cycles, Result<u16, ()>) {
        use OperandMode::*;
        match self {
            Literal(v) => (Cycles(0), Ok(v as u16)),
            Register(v) => (Cycles(0), Ok(gpr[v as usize].0 as u16)),
            RegisterDeferred(v) => {
                cpu.read_val::<u16>(gpr[v as usize].0 as usize)
            }
            Immediate() => {
                let v = cpu.read_val::<u16>((*pc).0 as usize);
                *pc += Wrapping(2);
                v
            }

            _ => todo!(),
        }
    }

    pub fn load_u32(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU) -> (Cycles, Result<u32, ()>) {
        use OperandMode::*;
        match self {
            Literal(v) => (Cycles(0), Ok(v as u32)),
            Register(v) => (Cycles(0), Ok(gpr[v as usize].0)),
            RegisterDeferred(v) => {
                cpu.read_val::<u32>(gpr[v as usize].0 as usize)
            }
            Immediate() => {
                let v = cpu.read_val::<u32>((*pc).0 as usize);
                *pc += Wrapping(4);
                v
            }

            _ => todo!(),
        }
    }

    pub fn store_u8(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU, value: u8) -> (Cycles, Result<(), ()>) {
        use OperandMode::*;
        match self {
            Literal(_) | Immediate() => {
                (Cycles(0), Err(()))
            }

            Register(v) => {
                gpr[v as usize] = Wrapping(value as u32);
                (Cycles(0), Ok(()))
            }

            _ => todo!(),
        }
    }

    pub fn store_u16(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU, value: u16) -> (Cycles, Result<(), ()>) {
        use OperandMode::*;
        match self {
            Literal(_) | Immediate() => {
                (Cycles(0), Err(()))
            }

            Register(v) => {
                gpr[v as usize] = Wrapping(value as u32);
                (Cycles(0), Ok(()))
            }

            _ => todo!(),
        }
    }

    pub fn store_u32(self, pc: &mut Wrapping<u32>, gpr: &mut [Wrapping<u32>; 16], cpu: &mut VAXCPU, value: u32) -> (Cycles, Result<(), ()>) {
        use OperandMode::*;
        match self {
            Literal(_) | Immediate() => {
                (Cycles(0), Err(()))
            }

            Register(v) => {
                gpr[v as usize] = Wrapping(value);
                (Cycles(0), Ok(()))
            }

            _ => todo!(),
        }
    }
}