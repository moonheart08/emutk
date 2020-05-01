use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;
use emutk_core::{
    cycles::Cycles,
};

pub fn instr_add3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_add(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_add2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_add(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_sub3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_sub(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_sub2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_sub(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_mcom
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = !x;
        let mut flags = v.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(v)
    })
}


pub fn instr_mneg
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_neg();
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_mov
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let mut flags = x.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(x)
    })
}

pub fn instr_mul3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_mul(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_mul2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_mul(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}
