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
    return Err(Error::new_reserved_instruction_fault())
}

pub fn instr_halt
    <T: VAXBus>
    (cpu: &mut VAXCPU<T>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    assert_kernel_mode(cpu)?;
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

pub fn instr_pushl
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let long = parse_read_operand::<B, u32>(cpu)?.read(cpu)?;
    push(cpu, long)
}

pub fn instr_popr
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mask = parse_read_operand::<B, u16>(cpu)?.read(cpu)?;
    let bv = bit_vec::BitVec::from_elem(mask as usize, false); // eww, allocation :T

    'lp: for (i, v) in bv.iter().enumerate() {
        if i == 15 { break 'lp; }
        if v {
            let val: u32 = pop(cpu)?;
            cpu.regfile.write_gpr(i as u8, val);
        }
    }

    Ok(())
}

pub fn instr_pushr
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mask = parse_read_operand::<B, u16>(cpu)?.read(cpu)?;
    let bv = bit_vec::BitVec::from_elem(mask as usize, false); // eww, allocation :T

    'lp: for (i, v) in bv.iter().enumerate() {
        if i == 15 { break 'lp; }
        if v {
            let val = cpu.regfile.read_gpr(i as u8);
            push(cpu, val)?;
        }
    }

    Ok(())
}

pub fn instr_mova
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let mut opr_a = parse_read_operand::<B,T>(cpu)?;
    opr_a.validate(cpu)?;
    let opw_o = parse_write_operand::<B,u32>(cpu)?;
    opw_o.write(cpu, opr_a.address()?)?;
    Ok(())
}

pub fn instr_mtpr
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    rr_instr_wrap(cpu, |x: u32, y: u32, cpu: &mut VAXCPU<B>| -> Result<(), Error> {
        cpu.regfile.write_msr(y as u16, x)
    })
}