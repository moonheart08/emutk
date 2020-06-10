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
    <T: VAXBus, X: VAXNum, Y: VAXNum, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<T>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(X, Y, &mut VAXCPU<T>) -> Result<O, Error>
{
    let opr_a = parse_read_operand::<T,X>(cpu)?.read(cpu)?;
    let opr_b = parse_read_operand::<T,Y>(cpu)?.read(cpu)?;
    let opw_o = parse_write_operand::<T,O>(cpu)?;

    let v = func(opr_a, opr_b, cpu)?;
    opw_o.write(cpu, v)?;
    Ok(())
}

pub fn rrrw_instr_wrap
    <T: VAXBus, X: VAXNum, Y: VAXNum, Z: VAXNum, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<T>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(X, Y, Z, &mut VAXCPU<T>) -> Result<O, Error>
{
    let opr_x = parse_read_operand::<T,X>(cpu)?.read(cpu)?;
    let opr_y = parse_read_operand::<T,Y>(cpu)?.read(cpu)?;
    let opr_z = parse_read_operand::<T,Z>(cpu)?.read(cpu)?;
    let opw_o = parse_write_operand::<T,O>(cpu)?;

    let v = func(opr_x, opr_y, opr_z, cpu)?;
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

pub fn rr_instr_wrap
    <B: VAXBus, X: VAXNum, Y: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(X, Y, &mut VAXCPU<B>) -> Result<(), Error>
{
    let opr_a = parse_read_operand::<B,X>(cpu)?.read(cpu)?;
    let opm_o = parse_read_operand::<B,Y>(cpu)?.read(cpu)?;

    func(opr_a, opm_o, cpu)
}


pub fn r_instr_wrap
    <B: VAXBus, X: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(X, &mut VAXCPU<B>) -> Result<(), Error>
{
    let opr_a = parse_read_operand::<B,X>(cpu)?.read(cpu)?;

    func(opr_a, cpu)
}



pub fn m_instr_wrap
    <B: VAXBus, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(O, &mut VAXCPU<B>) -> Result<O, Error>
{
    let mut opm_o = parse_write_operand::<B,O>(cpu)?;

    let v = func(opm_o.read(cpu)?, cpu)?;
    opm_o.execute_write(cpu, v);
    Ok(())
}

pub fn w_instr_wrap
    <B: VAXBus, O: VAXNum, F>
    (
        cpu: &mut VAXCPU<B>,
        func: F
    )
    -> Result<(), Error>
    where F: Fn(&mut VAXCPU<B>) -> Result<O, Error>
{
    let mut opm_o = parse_write_operand::<B,O>(cpu)?;

    let v = func(cpu)?;
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
    let mut opw_o = parse_write_operand::<B,O>(cpu)?;
    opw_o.validate(cpu)?;

    let v = func(opr_a, cpu)?;
    opw_o.write(cpu, v)?;
    Ok(())
}

#[inline]
pub fn assert_kernel_mode<B: VAXBus>(cpu: &mut VAXCPU<B>) -> Result<(), Error> {
    if cpu.regfile.get_psl().get_cur_mod() == 0 {
        Ok(())
    } else {
        Err(Error::new_privileged_instruction_fault())
    }
}

#[inline]
pub fn read_data<T: VAXNum, B: VAXBus>(cpu: &mut VAXCPU<B>) -> Result<T, Error> {
    let pc = cpu.regfile.get_pc();
    let result = cpu.read_val::<T>(pc);
    cpu.regfile.set_pc(pc + T::BYTE_LEN as u32);
    result
}

#[inline]
pub fn jump_with_byte_displacement<B: VAXBus>(cpu: &mut VAXCPU<B>, disp: u8) {
    let full_disp = disp as i8 as i32 as u32; // Sign extend.
    let pc = cpu.regfile.get_pc();
    cpu.regfile.set_pc(pc.wrapping_add(full_disp)); // Weeee!
}


#[inline]
pub fn jump_with_word_displacement<B: VAXBus>(cpu: &mut VAXCPU<B>, disp: u16) {
    let full_disp = disp as i16 as i32 as u32; // Sign extend.
    let pc = cpu.regfile.get_pc();
    cpu.regfile.set_pc(pc.wrapping_add(full_disp)); // Weeee!
}

#[inline]
pub fn push<B: VAXBus, T: VAXNum>(cpu: &mut VAXCPU<B>, val: T) -> Result<(), Error> {
    let sp = cpu.regfile.get_sp();
    cpu.write_val( sp, val)?;
    cpu.regfile.set_sp(sp.wrapping_sub(T::BYTE_LEN as u32));
    Ok(())
}

pub fn pop<B: VAXBus, T: VAXNum>(cpu: &mut VAXCPU<B>) -> Result<T, Error> {
    let sp = cpu.regfile.get_sp();
    let res = cpu.read_val(sp)?;
    cpu.regfile.set_sp(sp.wrapping_add(T::BYTE_LEN as u32));
    Ok(res)
}