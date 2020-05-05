mod arith;
mod misc;
mod control;
mod util;
#[cfg(test)]
mod tests;

use crate::cpu::VAXCPU;
use crate::cpu::instrs::InstructionType;
use crate::bus::VAXBus;
use crate::Error;
use control::BranchCondition;
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
        PUSHL => misc::instr_pushl(cpu, cycle_count),
        INCB => arith::instr_inc::<_, u8>(cpu, cycle_count),
        INCW => arith::instr_inc::<_, u16>(cpu, cycle_count),
        INCL => arith::instr_inc::<_, u32>(cpu, cycle_count),
        DECB => arith::instr_dec::<_, u8>(cpu, cycle_count),
        DECW => arith::instr_dec::<_, u16>(cpu, cycle_count),
        DECL => arith::instr_dec::<_, u32>(cpu, cycle_count),
        ADAWI => arith::instr_add2::<_, u32>(cpu, cycle_count),
        ADDB2 => arith::instr_add2::<_, u8>(cpu, cycle_count),
        ADDB3 => arith::instr_add3::<_, u8>(cpu, cycle_count),
        ADDW2 => arith::instr_add2::<_, u16>(cpu, cycle_count),
        ADDW3 => arith::instr_add3::<_, u16>(cpu, cycle_count),
        ADDL2 => arith::instr_add2::<_, u32>(cpu, cycle_count),
        ADDL3 => arith::instr_add3::<_, u32>(cpu, cycle_count),
        ADWC => arith::instr_adwc::<_, u32>(cpu, cycle_count),
        ASHL => arith::instr_ash::<_, u32>(cpu, cycle_count),
        ASHQ => arith::instr_ash::<_, u64>(cpu, cycle_count),
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
        CMPB => arith::instr_cmp::<_, u8>(cpu, cycle_count),
        CMPW => arith::instr_cmp::<_, u16>(cpu, cycle_count),
        CMPL => arith::instr_cmp::<_, u32>(cpu, cycle_count),
        BGTR => control::instr_branch_cond(cpu, cycle_count, BranchCondition::NOrZUnset),
        BLEQ => control::instr_branch_cond(cpu, cycle_count, BranchCondition::NOrZSet),
        BNEQ => control::instr_branch_cond(cpu, cycle_count, BranchCondition::ZUnset),
        BEQL => control::instr_branch_cond(cpu, cycle_count, BranchCondition::ZSet),
        BGEQ => control::instr_branch_cond(cpu, cycle_count, BranchCondition::NUnset),
        BLSS => control::instr_branch_cond(cpu, cycle_count, BranchCondition::NSet),
        BGTRU => control::instr_branch_cond(cpu, cycle_count, BranchCondition::COrZUnset),
        BLEQU => control::instr_branch_cond(cpu, cycle_count, BranchCondition::COrZSet),
        BVC => control::instr_branch_cond(cpu, cycle_count, BranchCondition::VUnset),
        BVS => control::instr_branch_cond(cpu, cycle_count, BranchCondition::VSet),
        BGEQU => control::instr_branch_cond(cpu, cycle_count, BranchCondition::CUnset),
        BLSSU => control::instr_branch_cond(cpu, cycle_count, BranchCondition::CSet),
        BLBC => control::instr_branch_low_bit(cpu, cycle_count, false),
        BLBS => control::instr_branch_low_bit(cpu, cycle_count, true),
        BRB => control::instr_branch_byte(cpu, cycle_count),
        BRW => control::instr_branch_word(cpu, cycle_count),
        BSBB => control::instr_branch_subroutine_byte(cpu, cycle_count),
        BSBW => control::instr_branch_subroutine_word(cpu, cycle_count),
        CASEB => control::instr_case::<u8, _>(cpu, cycle_count),
        CASEW => control::instr_case::<u16, _>(cpu, cycle_count),
        CASEL => control::instr_case::<u32, _>(cpu, cycle_count),
        JMP => control::instr_jmp(cpu, cycle_count),
        JSB => control::instr_jsb(cpu, cycle_count),
        RSB => control::instr_rsb(cpu, cycle_count),
        POPR => misc::instr_popr(cpu, cycle_count),
        PUSHR => misc::instr_pushr(cpu, cycle_count),
        MOVAB => misc::instr_mova::<_, u8>(cpu, cycle_count),
        MOVAW => misc::instr_mova::<_, u16>(cpu, cycle_count),
        MOVAL => misc::instr_mova::<_, u32>(cpu, cycle_count),
        MOVAQ => misc::instr_mova::<_, u64>(cpu, cycle_count),
        MOVAO => misc::instr_mova::<_, u128>(cpu, cycle_count),
        MTPR => misc::instr_mtpr(cpu, cycle_count),
        
        v => todo!("Unimplemented instr {:?}", v)
    }
}

