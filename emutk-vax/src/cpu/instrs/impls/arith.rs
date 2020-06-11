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

pub fn instr_inc
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    m_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_add(T::primitive_from(1_u32));
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_dec
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    m_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_sub(T::primitive_from(1_u32));
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}


pub fn instr_adwc
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let c: u32 = cpu.regfile.get_psl().get_c() as u32;

        let v = x.flagged_add(y.wrapping_add(&T::primitive_from(c)));
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
    rrw_instr_wrap::<B, T, T, T, _>(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
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


pub fn instr_ash
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: u8, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let (flags, val) = y.flagged_ash(x as i8);
        cpu.commit_flags(flags);
        Ok(val)
    })
}

pub fn instr_cmp
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rr_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<(), Error> {
        let (flags, _) = x.flagged_sub(y);
        cpu.commit_flags(flags);
        Ok(())
    })
}

pub fn instr_bic3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x & !y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_bic2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x & !y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_bis3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x | y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_bis2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x | y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_bit
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x & y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_emul
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrrw_instr_wrap(cpu, |x: u32, y: u32, z: u32, cpu: &mut VAXCPU<B>| -> Result<u64, Error> {
        let x = x as i32 as i64;
        let y = x as i32 as i64;
        let z = x as i32 as i64;
        let out = (x * y).wrapping_add(z) as u64;
        let mut flags = out.calc_nz();
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_tst
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    r_instr_wrap(cpu, |x: T, cpu: &mut VAXCPU<B>| -> Result<(), Error> {
        let v = x.flagged_sub(T::primitive_from(0_u32));
        cpu.commit_flags(v.0);
        Ok(())
    })
}

pub fn instr_xor3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x ^ y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_xor2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let out = x ^ y;
        let mut flags = out.calc_nz();
        flags.set_c(cpu.regfile.get_psl().get_c());
        cpu.commit_flags(flags);
        Ok(out)
    })
}

pub fn instr_div3
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrw_instr_wrap::<B, T, T, T, _>(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_div(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_div2
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rm_instr_wrap(cpu, |x: T, y: T, cpu: &mut VAXCPU<B>| -> Result<T, Error> {
        let v = x.flagged_div(y);
        cpu.commit_flags(v.0);
        Ok(v.1)
    })
}

pub fn instr_ediv
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rrww_instr_wrap(cpu, |divr: u32, divd: u64, cpu: &mut VAXCPU<B>| -> Result<(u32, u32), Error> {
        let (flags, quo) = divd.flagged_div(divr as u64);
        cpu.commit_flags(flags);
        let rem = (divd as i64) % (divr as i64);
        //println!("{}", rem);
        Ok((quo as u32, rem as u64 as u32))
    })
}