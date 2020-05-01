use crate::cpu::VAXCPU;
use crate::cpu::instrs::operands::*;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;

pub fn parse_read_operand
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>)
    -> Result<UnresolvedOperand<T>, Error>
{
    let pc = cpu.regfile.get_pc();
    let op_head: [u8;2] = cpu.read_val(pc)?;
    let mode = OperandMode::identify_operand(op_head)?;
    cpu.regfile.set_pc(pc + mode.byte_size::<T>() as u32);
    mode.create_resolvable::<B,T>(cpu, false)
}

pub fn parse_write_operand
    <B: VAXBus, T: VAXNum>
    (cpu: &mut VAXCPU<B>)
    -> Result<UnresolvedOperand<T>, Error>
{
    let pc = cpu.regfile.get_pc();
    let op_head: [u8;2] = cpu.read_val(pc)?;
    let mode = OperandMode::identify_operand(op_head)?;
    cpu.regfile.set_pc(pc + mode.byte_size::<T>() as u32);
    mode.create_resolvable::<B,T>(cpu, true)
}

pub fn rrw_instr_wrap
    <T: VAXBus, I: VAXNum, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<T>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(I, I, &mut VAXCPU<T>) -> Result<O, Error>
{
    let opr_a = parse_read_operand::<T,I>(cpu)?.read(cpu)?;
    let opr_b = parse_read_operand::<T,I>(cpu)?.read(cpu)?;
    let opw_o = parse_write_operand::<T,O>(cpu)?;

    let v = func(opr_a, opr_b, cpu)?;
    opw_o.write(cpu, v)?;
    Ok(())
}

pub fn rm_instr_wrap
    <B: VAXBus, V: VAXNum, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(V, O, &mut VAXCPU<B>) -> Result<O, Error>
{
    let opr_a = parse_read_operand::<B,V>(cpu)?.read(cpu)?;
    let mut opm_o = parse_write_operand::<B,O>(cpu)?;

    let v = func(opr_a, opm_o.read(cpu)?, cpu)?;
    opm_o.execute_write(cpu, v);
    Ok(())
}

pub fn rw_instr_wrap
    <B: VAXBus, I: VAXNum, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(I, &mut VAXCPU<B>) -> Result<O, Error>
{
    let opr_a = parse_read_operand::<B,I>(cpu)?.read(cpu)?;
    let opw_o = parse_write_operand::<B,O>(cpu)?;

    let v = func(opr_a, cpu)?;
    opw_o.write(cpu, v)?;
    Ok(())
}