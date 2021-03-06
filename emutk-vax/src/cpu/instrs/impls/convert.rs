use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;
use emutk_core::{
    cycles::Cycles,
};

pub fn instr_movzbw
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u8, cpu: &mut VAXCPU<B>| -> Result<u16, Error> {
        Ok(i as u16)
    })
}

pub fn instr_movzbl
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u8, cpu: &mut VAXCPU<B>| -> Result<u32, Error> {
        Ok(i as u32)
    })
}

pub fn instr_movzwl
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u16, cpu: &mut VAXCPU<B>| -> Result<u32, Error> {
        Ok(i as u32)
    })
}

pub fn instr_cvtbw
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u8, cpu: &mut VAXCPU<B>| -> Result<u16, Error> {
        Ok(i as i8 as i16 as u16)
    })
}

pub fn instr_cvtbl
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u8, cpu: &mut VAXCPU<B>| -> Result<u32, Error> {
        Ok(i as i8 as i32 as u32)
    })
}

pub fn instr_cvtwb
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u16, cpu: &mut VAXCPU<B>| -> Result<u8, Error> {
        Ok(i as i16 as i8 as u8)
    })
}


pub fn instr_cvtwl
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u16, cpu: &mut VAXCPU<B>| -> Result<u32, Error> {
        Ok(i as i16 as i32 as u32)
    })
}

pub fn instr_cvtlb
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u32, cpu: &mut VAXCPU<B>| -> Result<u8, Error> {
        Ok(i as i32 as i8 as u8)
    })
}

pub fn instr_cvtlw
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rw_instr_wrap(cpu, |i: u32, cpu: &mut VAXCPU<B>| -> Result<u16, Error> {
        Ok(i as i32 as i16 as u16)
    })
}