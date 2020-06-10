use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::{VAXNum, Error};
use crate::cpu::PSL;
use emutk_core::{
    cycles::Cycles,
};

/// TODO: figure out if there's some sort of pattern here, to replace the 
/// switch table with some simple logic
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BranchCondition {
    /// BGTR
    NOrZUnset,
    /// BLEQ
    NOrZSet,
    /// BNEQ
    /// BNEQU
    ZUnset,
    /// BEQL
    /// BEQLU
    ZSet,
    /// BGEQ
    NUnset,
    /// BLSS
    NSet,
    /// BGTRU
    COrZUnset,
    /// BLEQU
    COrZSet,
    /// BVC
    VUnset,
    /// BVS
    VSet,
    /// BGEQU
    /// BCC
    CUnset,
    /// BLSSU
    /// BCS
    CSet,
}

impl BranchCondition {
    pub fn test(self, flags: PSL) -> bool {
        match self {
            BranchCondition::NOrZUnset => (!flags.get_n()) || !flags.get_z(),
            BranchCondition::NOrZSet => flags.get_n() || flags.get_z(),
            BranchCondition::ZUnset => !flags.get_z(),
            BranchCondition::ZSet => flags.get_z(),
            BranchCondition::NUnset => !flags.get_n(),
            BranchCondition::NSet => flags.get_n(),
            BranchCondition::COrZUnset => (!flags.get_c()) || !flags.get_z(),
            BranchCondition::COrZSet => flags.get_c() || flags.get_z(),
            BranchCondition::VUnset => !flags.get_v(),
            BranchCondition::VSet => flags.get_v(),
            BranchCondition::CUnset => !flags.get_c(),
            BranchCondition::CSet => flags.get_c(),
        }
    }
}

pub fn instr_branch_cond
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles, cond: BranchCondition)
    -> Result<(), Error>
{
    let displ = read_data::<u8, T>(cpu)?;
    let will_branch = cond.test(*cpu.regfile.get_psl());
    if will_branch {
        jump_with_byte_displacement(cpu, displ);
    } else {
        // Do nothing.
    }
    Ok(())
}

pub fn instr_branch_cond_norzus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::NOrZUnset)
}


pub fn instr_branch_cond_norzs
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::NOrZSet)
}

pub fn instr_branch_cond_zus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::ZUnset)
}

pub fn instr_branch_cond_zs
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::ZSet)
}

pub fn instr_branch_cond_nus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::NUnset)
}

pub fn instr_branch_cond_ns
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::NSet)
}

pub fn instr_branch_cond_corzus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::COrZUnset)
}

pub fn instr_branch_cond_corzs
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::COrZSet)
}

pub fn instr_branch_cond_vus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::VUnset)
}

pub fn instr_branch_cond_vs
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::VSet)
}

pub fn instr_branch_cond_cus
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::CUnset)
}

pub fn instr_branch_cond_cs
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_cond(cpu, cycle_count, BranchCondition::CSet)
}


pub fn instr_branch_low_bit
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles, val: bool)
    -> Result<(), Error>
{
    let op = parse_read_operand::<T, u32>(cpu)?.read(cpu)?;
    let displ = read_data::<u8, T>(cpu)?;
    if (op & 0x1 != 0) == val {
        jump_with_byte_displacement(cpu, displ);
    }
    Ok(())
}

pub fn instr_branch_low_bit_true
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_low_bit(cpu, cycle_count, true)
}

pub fn instr_branch_low_bit_false
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    instr_branch_low_bit(cpu, cycle_count, false)
}

pub fn instr_branch_byte
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let displ = read_data::<u8, T>(cpu)?;
    jump_with_byte_displacement(cpu, displ);
    Ok(())
}

pub fn instr_branch_word
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let displ = read_data::<u16, T>(cpu)?;
    jump_with_word_displacement(cpu, displ);
    Ok(())
}

pub fn instr_branch_subroutine_byte
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let displ = read_data::<u8, T>(cpu)?;
    push(cpu, cpu.regfile.get_pc())?;
    jump_with_byte_displacement(cpu, displ);
    Ok(())
}

pub fn instr_branch_subroutine_word
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let displ = read_data::<u16, T>(cpu)?;
    push(cpu, cpu.regfile.get_pc())?;
    jump_with_word_displacement(cpu, displ);
    Ok(())
}

pub fn instr_case
    <T: VAXNum, B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let selector = parse_read_operand::<B,T>(cpu)?.read(cpu)?;
    let base = parse_read_operand::<B,T>(cpu)?.read(cpu)?;
    let limit = parse_read_operand::<B,T>(cpu)?.read(cpu)?;

    let tmp: u32 = selector.wrapping_sub(&base).as_();
    let (flags, _) = tmp.flagged_sub(limit.as_());

    let pc = cpu.regfile.get_pc();

    let disp = if flags.get_n() || flags.get_z() {
        let idx = tmp.wrapping_mul(2);
        let addr = pc.wrapping_add(idx);
        cpu.read_val::<u16>(addr)?
    } else {
        let addr = pc.wrapping_add(2 + 2_u32.wrapping_mul(limit.as_()));
        cpu.read_val::<u16>(addr)?
    };

    jump_with_word_displacement(cpu, disp);
    Ok(())
}

pub fn instr_jmp
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mut selector = parse_read_operand::<T, u32>(cpu)?;
    selector.validate(cpu)?;
    let addr = selector.address()?;
    cpu.regfile.set_pc(addr);
    Ok(())
}

pub fn instr_jsb
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mut selector = parse_read_operand::<T, u32>(cpu)?;
    selector.validate(cpu)?;
    let addr = selector.address()?;
    push(cpu, cpu.regfile.get_pc())?;
    cpu.regfile.set_pc(addr);
    Ok(())
}

pub fn instr_rsb
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let npc = pop(cpu)?;
    cpu.regfile.set_pc(npc);
    Ok(())
}

pub fn instr_callg
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mut arglist = parse_read_operand::<T, u8>(cpu)?;
    arglist.validate(cpu)?;
    let mut dest = parse_read_operand::<T, u8>(cpu)?;
    dest.validate(cpu)?;

    let header = cpu.read_val::<u16>(dest.address()?)?;


    let old_sp = cpu.regfile.get_sp(); 
    let sp = old_sp & !0x3; // zero lower 2 bits for alignment
    cpu.regfile.set_sp(sp);
    // read mask and push registers as needed
    for i in 11..0 {
        if header & (1 << i) != 0 {
            let reg = cpu.regfile.read_gpr(i);
            push(cpu, reg)?;
        }
    }
    push(cpu, cpu.regfile.get_pc())?;
    push(cpu, cpu.regfile.read_gpr(13))?; // FP
    push(cpu, cpu.regfile.read_gpr(12))?; // AP
    let psl = cpu.regfile.get_psl_mut();
    psl.0 &= !0xF; // zero NZVC
    let masks = ((old_sp & 0x3) << 30) 
              | ((header as u32 & 0b111_1111_1111) << 15)
              | (psl.0 & 0xFFFF);
    push(cpu, masks)?;
    push(cpu, 0_u32)?;
    cpu.regfile.write_gpr(13, sp);
    cpu.regfile.write_gpr(12, arglist.address()?);

    let psl = cpu.regfile.get_psl_mut(); // rustc pls
    psl.set_dv((header & (1 << 15)) != 0);
    psl.set_iv((header & (1 << 14)) != 0);
    psl.set_fu(false);

    Ok(())
}

pub fn instr_calls
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mut argnum = parse_read_operand::<T, u32>(cpu)?.read(cpu)?;
    let mut dest = parse_read_operand::<T, u8>(cpu)?;
    dest.validate(cpu)?;

    push(cpu, argnum)?;
    let tmp1 = cpu.regfile.get_sp();

    let header = cpu.read_val::<u16>(dest.address()?)?;


    let old_sp = cpu.regfile.get_sp(); 
    let sp = old_sp & !0x3; // zero lower 2 bits for alignment
    cpu.regfile.set_sp(sp);
    // read mask and push registers as needed
    for i in 11..0 {
        if header & (1 << i) != 0 {
            let reg = cpu.regfile.read_gpr(i);
            push(cpu, reg)?;
        }
    }
    push(cpu, cpu.regfile.get_pc())?;
    push(cpu, cpu.regfile.read_gpr(13))?; // FP
    push(cpu, cpu.regfile.read_gpr(12))?; // AP
    let psl = cpu.regfile.get_psl_mut();
    psl.0 &= !0xF; // zero NZVC
    let masks = ((old_sp & 0x3) << 30) 
              | ((header as u32 & 0b111_1111_1111) << 15)
              | (psl.0 & 0xFFFF)
              | 1 << 29;
    push(cpu, masks)?;
    push(cpu, 0_u32)?;
    cpu.regfile.write_gpr(13, sp);
    cpu.regfile.write_gpr(12, tmp1);

    let psl = cpu.regfile.get_psl_mut(); // rustc pls
    psl.set_dv((header & (1 << 15)) != 0);
    psl.set_iv((header & (1 << 14)) != 0);
    psl.set_fu(false);

    Ok(())
}

pub fn instr_ret
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    {
        let sp = cpu.regfile.get_sp();
        cpu.regfile.set_sp(sp + 4);
    }
    let tmp1: u32 = pop(cpu)?;
    let ap = pop(cpu)?;
    cpu.regfile.write_gpr(12, ap);
    let fp = pop(cpu)?;
    cpu.regfile.write_gpr(13, fp);
    let pc = pop(cpu)?;
    cpu.regfile.set_pc(pc);
    let tmp2 = (tmp1 >> 16) & 0b111_1111_1111;
    for i in 0..11 {
        if tmp2 & (1 << i) != 0 {
            let rval = pop(cpu)?;
            cpu.regfile.write_gpr(i, rval);
        }
    }
    let offs = (tmp1 >> 30) & 0x3;
    let sp = cpu.regfile.get_sp();
    cpu.regfile.set_sp(sp + offs);

    if ((tmp1 >> 29) & 0x1) != 0 {
        let v: u32 = pop(cpu)?;
        let tmp4 = 4 * (v & 0xFF);
        cpu.regfile.set_sp(sp + offs + tmp4);
    }
    let psl = cpu.regfile.get_psl_mut();
    psl.0 &= tmp1 & 0xFFFF;

    Ok(())
}