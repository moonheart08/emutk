use super::util::*;
use crate::cpu::VAXCPU;
use crate::bus::VAXBus;
use crate::Error;
use crate::VAXNum;
use emutk_core::{
    cycles::Cycles,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MultiInstruction {
    None,
    CMPC3 {
        len: u16,
    },
    CMPC5 {
        fill: u8,
    },
    LOCC {
        char: u8,
    },
    MATCHC {},
    MOVC3 {},
    MOVC5 {
        fill: u8,
    },
    MOVTC {
        fill: u8,
    },
    MOVTUC {
        esc: u8,
    },
    SCANC {
        mask: u8,
    },
    SKPC {
        char: u8,
    },
    SPANC {
        mask: u8,
    },
}

pub fn instr_movc3
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    let len = parse_read_operand::<B,u16>(cpu)?.read(cpu)?;
    let mut srcaddr = parse_read_operand::<B,u8>(cpu)?;
    srcaddr.validate(cpu)?;
    let mut dstaddr = parse_read_operand::<B,u8>(cpu)?;
    dstaddr.validate(cpu)?;

    cpu.regfile.set_r0(len as u32);
    cpu.regfile.set_r1(srcaddr.address()?);
    cpu.regfile.set_r2(0);
    cpu.regfile.set_r3(dstaddr.address()?);
    cpu.regfile.set_r4(0);
    cpu.regfile.set_r5(0);
    cpu.multi_instr_active = MultiInstruction::MOVC3{};
    Ok(())
}


pub fn exec_multi_instructions
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    match cpu.multi_instr_active {
        MultiInstruction::MOVC3 {} => {
            exec_movc3(cpu, cycle_count)?;
        }
        _=> unimplemented!(),
    }
    Ok(())
}

pub fn exec_movc3
    <B: VAXBus>
    (cpu: &mut VAXCPU<B>, _cycle_count: &mut Cycles)
    -> Result<(), Error>
{
    // this is literally just memcpy :T
    let mut full_len = cpu.regfile.get_r0();
    let mut src = cpu.regfile.get_r1();
    let mut dst = cpu.regfile.get_r3();
    let len = full_len & 0xFF; // how many bytes to copy this go around.
    
    //OPTIMIZATION POTENTIAL:
    // Add API to bus to get mut slice to a chunk of RAM? (would allow direct memcpy instead of this)
    println!("MOVC3 | Processing: {} bytes | Remainder: {} bytes", len, full_len - len);
    for _ in 0..len {
        let sbyte: u8 = cpu.read_val(src)?;
        println!("I: {} | O: {} | V: {:?}", src, dst, sbyte);

        cpu.write_val(dst, sbyte)?;
        full_len -= 1;
        cpu.regfile.set_r0(full_len);
        src += 1;
        cpu.regfile.set_r1(src);
        dst += 1;
        cpu.regfile.set_r3(dst);
    }

    if full_len == 0 {
        cpu.multi_instr_active = MultiInstruction::None; // Done!
    }

    Ok(())
}