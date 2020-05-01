use crate::cpu::VAXCPU;

use crate::bus::VAXBus;
use crate::Error;

use crate::cpu::instrs::InstructionType;
use crate::cpu::instrs::execute_instr;

use emutk_core::cycles::Cycles;

impl<B: VAXBus> VAXCPU<B> {
    pub fn run_tick(&mut self) -> Result<(), Error> {
        if self.halted == true {
            return Ok(());
        }

        let pc = self.regfile.get_pc();
        let instr = {
            let by: [u8; 2] = self.read_val(pc)?;
            InstructionType::from_instrid(by)
        };

        if let Some(i) = instr {
            self.regfile.set_pc(pc + i.opcode_len() as u32);
        }
        let mut cyc = Cycles(0);
        execute_instr(instr, self, &mut cyc)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::RAMBus;
    use test::Bencher;

    #[test]
    fn exectest() {
        let mut cpu = VAXCPU::new();
        let mut bus = RAMBus::new(8192);
        let rom = bus.ram_mut();
        let bytes = &[0xD0, 0x8F, 0x01, 0x00, 0x00, 0x00, 0x50];
        (*rom)[0..7].copy_from_slice(bytes);
        //(*rom)[8..15].copy_from_slice(bytes);

        cpu.give_bus(bus);

        while cpu.halted == false {
            match cpu.run_tick() {
                Ok(()) => {},
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        assert_eq!(cpu.regfile.get_r0(), 1);
    }

    #[test]
    fn exectest2() {
        let mut cpu = VAXCPU::new();
        let mut bus = RAMBus::new(8192);
        let rom = bus.ram_mut();
        let bytes = &[
            // MOVL $5, R0
            0xD0, 0x05, 0x50,
            // MULL2 R0, R0
            0xC4, 0x50, 0x50,
        ];
        (*rom)[0..bytes.len()].copy_from_slice(bytes);
        //(*rom)[8..15].copy_from_slice(bytes);

        cpu.give_bus(bus);

        while cpu.halted == false {
            match cpu.run_tick() {
                Ok(()) => {},
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        assert_eq!(cpu.regfile.get_r0(), 25);
    }

    #[bench]
    fn execbench(b: &mut Bencher) {
        let mut cpu = VAXCPU::new();
        let mut bus = RAMBus::new(8192);
        let rom = bus.ram_mut();
        let bytes = &[
                // MOVL $5, R0
                0xD0, 0x05, 0x50,
                // MULL2 R0, R0
                0xC4, 0x50, 0x50,
                // NOP
                0x01,
                // MOVO $9, R1:R5
                0xFD, 0x7D, 0x09, 0x51,
                // ADDW2 1024, R0
                0xA0, 0x8F, 0x00, 0x0F, 0x50,
                // MNEGL R1, (R0)+
                0xCE, 0x51, 0x80,
                // CLRL -(R0)
                0xD4, 0x70,
            ];
        rom[..(bytes.len())*32].chunks_mut(bytes.len()).for_each(|ch| ch.copy_from_slice(bytes));
        cpu.give_bus(bus);

        b.iter(|| {
            cpu.halted = false;
            cpu.regfile.set_pc(0);

            test::black_box(while !cpu.halted() {
                match cpu.run_tick() {
                    Ok(()) => {
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
                
            })
        });
    }
}