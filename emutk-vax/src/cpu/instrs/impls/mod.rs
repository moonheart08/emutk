mod arith;
mod misc;
mod util;
#[cfg(test)]
mod tests;

use crate::cpu::VAXCPU;
use crate::cpu::instrs::InstructionType;
use crate::bus::VAXBus;
use crate::Error;
use emutk_core::{
    cycles::Cycles,
};

/*
// Instruction form:
fn instr_name(&mut VAXCPU, &mut Cycles)

*/

pub fn execute_instr<T: VAXBus>
    (
        instr: Option<InstructionType>, 
        cpu: &mut VAXCPU<T>, 
        cycle_count: &mut Cycles
    )
    -> Result<(), Error>
{
    if let None = instr {
        return misc::instr_invalid(cpu, cycle_count);
    }
    use InstructionType::*;
    match instr.unwrap() {
        HALT => misc::instr_halt(cpu, cycle_count),
        NOP => Ok(()),
        ADAWI => arith::instr_add2::<_, u32>(cpu, cycle_count),
        ADDB2 => arith::instr_add2::<_, u8>(cpu, cycle_count),
        ADDB3 => arith::instr_add3::<_, u8>(cpu, cycle_count),
        ADDW2 => arith::instr_add2::<_, u16>(cpu, cycle_count),
        ADDW3 => arith::instr_add3::<_, u16>(cpu, cycle_count),
        ADDL2 => arith::instr_add2::<_, u32>(cpu, cycle_count),
        ADDL3 => arith::instr_add3::<_, u32>(cpu, cycle_count),
        SUBB2 => arith::instr_sub2::<_, u8>(cpu, cycle_count),
        SUBB3 => arith::instr_sub3::<_, u8>(cpu, cycle_count),
        SUBW2 => arith::instr_sub2::<_, u16>(cpu, cycle_count),
        SUBW3 => arith::instr_sub3::<_, u16>(cpu, cycle_count),
        SUBL2 => arith::instr_sub2::<_, u32>(cpu, cycle_count),
        SUBL3 => arith::instr_sub3::<_, u32>(cpu, cycle_count),
        MCOMB => arith::instr_mcom::<_, u8>(cpu, cycle_count),
        MCOMW => arith::instr_mcom::<_, u16>(cpu, cycle_count),
        MCOML => arith::instr_mcom::<_, u32>(cpu, cycle_count),
        MNEGB => arith::instr_mneg::<_, u8>(cpu, cycle_count),
        MNEGW => arith::instr_mneg::<_, u16>(cpu, cycle_count),
        MNEGL => arith::instr_mneg::<_, u32>(cpu, cycle_count),
        MOVB => arith::instr_mov::<_, u8>(cpu, cycle_count),
        MOVW => arith::instr_mov::<_, u16>(cpu, cycle_count),
        MOVL => arith::instr_mov::<_, u32>(cpu, cycle_count),
        MOVQ => arith::instr_mov::<_, u64>(cpu, cycle_count),
        MOVO => arith::instr_mov::<_, u128>(cpu, cycle_count),
        MULB2 => arith::instr_mul2::<_, u8>(cpu, cycle_count),
        MULB3 => arith::instr_mul3::<_, u8>(cpu, cycle_count),
        MULW2 => arith::instr_mul2::<_, u16>(cpu, cycle_count),
        MULW3 => arith::instr_mul3::<_, u16>(cpu, cycle_count),
        MULL2 => arith::instr_mul2::<_, u32>(cpu, cycle_count),
        MULL3 => arith::instr_mul3::<_, u32>(cpu, cycle_count),
        CLRB => misc::instr_clr::<_, u8>(cpu, cycle_count),
        CLRW => misc::instr_clr::<_, u16>(cpu, cycle_count),
        CLRL => misc::instr_clr::<_, u32>(cpu, cycle_count),
        CLRQ => misc::instr_clr::<_, u64>(cpu, cycle_count),
        CLRO => misc::instr_clr::<_, u128>(cpu, cycle_count),
        v => todo!("Unimplemented instr {:?}", v)
    }
}

