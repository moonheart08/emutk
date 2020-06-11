use crate::cpu::VAXCPU;

use crate::bus::VAXBus;
#[cfg(test)]
use crate::bus::RAMBus;
use crate::Error;

use crate::cpu::instrs::InstructionType;
use crate::cpu::instrs::execute_instr;
use crate::cpu::instrs::MultiInstruction;
use crate::cpu::instrs::exec_multi_instructions;

use emutk_core::cycles::Cycles;

impl<B: VAXBus> VAXCPU<'_, B> {
    pub fn run_tick(&mut self) -> Result<(), Error> {
        if self.halted == true {
            return Ok(());
        }

        if self.multi_instr_active != MultiInstruction::None {
            let mut cyc = Cycles(0);
            exec_multi_instructions(self, &mut cyc)?;
            self.cur_cycle += cyc;
            Ok(())
        } else {
            let pc = self.regfile.get_pc();
            let instr = self.read_val(pc)?;

            //println!("{:?}", InstructionType::from_instrid(instr));
            //println!("{:01$x}", pc, 8);

            let mut cyc = Cycles(0);
            execute_instr(instr, self, &mut cyc)?;
            self.cur_cycle += cyc;
            if self.halted {
                println!("HALT: {:01$x}", pc, 8);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
pub fn simple_test_cpu_with_data(dat: &[u8]) -> (VAXCPU<RAMBus>, RAMBus) {
    let cpu = VAXCPU::new();
    let mut bus = RAMBus::new(dat.len()+1024);
    let buf = bus.ram_mut();
    buf[0..dat.len()].copy_from_slice(dat);
    (cpu, bus)
}

#[cfg(test)]
pub fn simple_test_cpu<'a>() -> (VAXCPU<'a, RAMBus>, RAMBus) {
    let cpu = VAXCPU::new();
    let bus = RAMBus::new(8192);
    (cpu, bus)
}


#[cfg(test)]
pub fn run_sample_to_done<'a>(dat: &'_ [u8]) -> (VAXCPU<'a, RAMBus>, RAMBus) {
    let mut cpu = VAXCPU::new();
    let mut bus = RAMBus::new(dat.len()+1024);
    let buf = bus.ram_mut();
    buf[0..dat.len()].copy_from_slice(dat);
    while cpu.halted == false {
        match cpu.run_tick() {
            Ok(()) => {},
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    (cpu, bus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::RAMBus;
    use test::Bencher;

    #[test]
    fn exectest() {
        let dat = &[0xD0, 0x8F, 0x01, 0x00, 0x00, 0x00, 0x50];
        let (mut cpu, mut bus) = simple_test_cpu_with_data(dat);
        //(*rom)[8..15].copy_from_slice(bytes);

        cpu.give_bus(&mut bus);

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
    fn loop_test() {
        let dat = &[0x96, 0x8f, 0x00, 0x12, 0xfb, 0x00];
        let (mut cpu, mut bus) = simple_test_cpu_with_data(dat);
        //(*rom)[8..15].copy_from_slice(bytes);

        cpu.give_bus(&mut bus);

        let mut ticks_left = 1024;

        while cpu.halted == false {
            if ticks_left == 0 {
                panic!("Loop never exited!");
            }
            match cpu.run_tick() {
                Ok(()) => {
                    ticks_left -= 1;
                },
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        }

        if ticks_left > 1000 {
            panic!("Loop didn't even work!");
        }
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
        rom[0..bytes.len()].copy_from_slice(bytes);
        //(*rom)[8..15].copy_from_slice(bytes);

        cpu.give_bus(&mut bus);

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
        cpu.give_bus(&mut bus);

        b.iter(|| {
            cpu.halted = false;
            cpu.regfile.set_pc(0);

            test::black_box(while !cpu.halted() {
                match cpu.run_tick() {
                    Ok(_) => {
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
                
            });

        });
    }
}