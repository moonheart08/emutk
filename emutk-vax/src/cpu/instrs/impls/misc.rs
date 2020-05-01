use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;
use emutk_core::{
    cycles::Cycles,
};


pub fn instr_invalid
    <T: VAXBus>
    (_cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    todo!()
}

pub fn instr_nop
    <T: VAXBus>
    (_cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    Ok(())
}

pub fn instr_halt
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    cpu.halt();
    Ok(())
}


pub fn instr_clr
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    parse_write_operand::<B,T>(cpu)?.write(cpu, T::primitive_from(0_u32))
}