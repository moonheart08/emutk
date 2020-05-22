use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;
use emutk_core::{
    cycles::Cycles,
};
use crate::cpu::instrs::operands::*;

pub enum BitfieldOperand {
    Register{ pos: u32, field_len: u8, reg: u8 },
    Memory{ pos: u32, field_len: u8, base: u32 },
}

fn check_bitfield
    <T: VAXBus>
    (
        pos: u32,
        field_len: u8,
        mut base: UnresolvedOperand<u32>,
        sign_extend: bool,
    ) -> Result<BitfieldOperand, Error>
{
    if field_len == 0 {
        return Ok(BitfieldOperand::Register {
            pos,
            field_len,
            reg: 0,
        });
    }

    if field_len > 32 {
        return Err(Error::new_reserved_operand_fault());
    }


    match base {
        UnresolvedOperand::Value(_, reg) => {
            Ok(BitfieldOperand::Register {
                pos,
                field_len,
                reg,
            })
        },
        mut v => {
            Ok(BitfieldOperand::Memory {
                pos,
                field_len,
                base: v.address()?,
            })
        }
    }

}