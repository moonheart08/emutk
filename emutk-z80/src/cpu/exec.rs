/*use emutk_core::{
    bus::{
        Bus,
        BusError,
    },
    cycles::Cycles,
    ByteRepr,
};

use num_derive::*;
use num_traits::{ToPrimitive, FromPrimitive};

use crate::bus::Z80BusError;
use std::num::Wrapping;
use crate::cpu::*;

impl<T: Bus<Z80BusError>> Z80CPU<T> {
    fn read_byte_from_pc(&mut self, cycles: &mut Cycles) -> u8 {
        let (cy, res) = self.read_val::<u8>(self.pc.0);
        *cycles += cy;
        self.pc += Wrapping(1);
        res
    }

    fn read_word_from_pc(&mut self, cycles: &mut Cycles) -> u16 {
        let (cy, res) = self.read_val::<u16>(self.pc.0);
        *cycles += cy;
        self.pc += Wrapping(2);
        res
    }

    fn finalize(&mut self) {
        self.pc += Wrapping(1);
    }

    fn read_8b_reg(&mut self, reg: u8, cycles: &mut Cycles) -> u8 {
        assert!(reg < 8);
        match reg {
            0 => self.bc.split_le()[0],
            1 => self.bc.split_le()[1],
            2 => self.de.split_le()[0],
            3 => self.de.split_le()[1],
            4 => self.hl.split_le()[0],
            5 => self.hl.split_le()[1],
            6 => { // Read (HL)
                let (cy, res) = self.read_val::<u8>(self.hl);
                *cycles += cy;
                res
            }
            7 => self.af.split_le()[0],
            _ => unreachable!(),
        }
    }

    fn write_8b_reg(&mut self, reg: u8, val: u8, cycles: &mut Cycles) {
        assert!(reg < 8);
        match reg {
            0 => self.bc.modify_lower_le(val),
            1 => self.bc.modify_upper_le(val),
            2 => self.de.modify_lower_le(val),
            3 => self.de.modify_upper_le(val),
            4 => self.hl.modify_lower_le(val),
            5 => self.hl.modify_upper_le(val),
            6 => {
                *cycles += self.write_val::<u8>(self.hl, val);
            },
            7 => self.af.modify_lower_le(val),
            _ => unreachable!(),
        }
    }

    fn read_16b_reg_1(&mut self, reg: u8) -> u16 {
        assert!(reg < 4);
        match reg {
            0 => self.bc,
            1 => self.de,
            2 => self.hl,
            3 => self.sp,
            _ => unreachable!(),
        }
    }



    fn write_16b_reg_1(&mut self, reg: u8, val: u16){
        assert!(reg < 4);
        match reg {
            0 => self.bc = val,
            1 => self.de = val,
            2 => self.hl = val,
            3 => self.sp = val,
            _ => unreachable!(),
        }
    }

    fn displace_pc(&mut self, amount: i8) {
        self.pc += Wrapping(amount as u16);
    }

    fn read_disp(&mut self, cycles: &mut Cycles) -> i8 {
        self.read_byte_from_pc(cycles) as i8
    }

    fn check_jmp_condition(&mut self, cc: u8) -> bool {
        assert!(cc < 8);
        match cc {
            0 => !self.z(),
            1 => self.z(),
            2 => !self.c(),
            3 => self.c(),
            4 => !self.v(),
            5 => self.v(),
            6 => !self.s(),
            7 => self.s(),
            _ => unreachable!()
        }
    }

    // Runs through a single instruction.
    pub fn run_step(&mut self) -> Cycles {
        let mut cycle_count = Cycles(0);

        let instrbyte = self.read_byte_from_pc(&mut cycle_count);
        
        let x = (0b11_000_000 & instrbyte) >> 6;
        let y = (0b00_111_000 & instrbyte) >> 3;
        let z = (0b00_000_111 & instrbyte) >> 0;
        let p = (0b00_110_000 & instrbyte) >> 4;
        let q = (0b00_001_000 & instrbyte) != 0;

        // Credit to http://z80.info/decoding.htm for making the decoder so clean to write.
        match x {
            0b00 =>  {
                match z {
                    0b000 => {
                        match y {
                            // NOP
                            0 => {},
                            // EX AF, AF'
                            1 => std::mem::swap(&mut self.af, &mut self.af_bk),
                            // DJNZ
                            2 => { 
                                let disp = self.read_disp(&mut cycle_count);
                                if self.bc.split_le()[0].wrapping_sub(1) != 0 {
                                    self.displace_pc(disp);
                                }
                            }
                            // JR d
                            3 => {
                                let disp = self.read_disp(&mut cycle_count);
                                self.displace_pc(disp)
                            },
                            // Jcc d
                            v => { 
                                let v = v - 4;
                                let cond = self.check_jmp_condition(v);
                                let disp = self.read_disp(&mut cycle_count);
                                if cond { self.displace_pc(disp) }
                            }
                        }
                    }
                    0b001 => {
                        match q {
                            // LD rp[p], nn
                            false => {
                                let val = self.read_word_from_pc(&mut cycle_count);
                                self.write_16b_reg_1(p, val);
                            }
                            // ADD HL, rp[p]
                            true => {
                                todo!()
                            }
                        }
                    }
                    _ => todo!()
                }
                
            }
            _ => todo!()
        }

        self.finalize();

        cycle_count
    }


}
*/