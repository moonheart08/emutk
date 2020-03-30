use crate::{
    cpu::*,
    instructiontypes::*,
    operands::*,
    DataSize,
};
use emutk_core::{
    cycles::Cycles,
};

#[derive(Copy, Clone, Debug)]
pub enum FetAndParErr {
    FetchError,
    ParseError,
}

#[inline]
pub fn fetch_and_parse_op(pc: &mut Wrapping<u32>, cpu: &mut VAXCPU) -> (Cycles, Result<OperandMode, FetAndParErr>) {
    let (cycles, src_bytes) = cpu.read_val::<[u8;2]>(pc.0 as usize);
    if src_bytes == Err(()) {
        return (cycles, Err(FetAndParErr::FetchError));
    }
    let src = OperandMode::identify_operand(src_bytes.unwrap());
    if src == None {
        return (cycles, Err(FetAndParErr::ParseError));
    }

    *pc += Wrapping(src.unwrap().pc_add_amnt(DataSize::Longword) as u32);

    return (cycles, Ok(src.unwrap()));
}


impl VAXCPU {
    /// Runs a undefined number of cycles, usually how long it takes to parse and execute the next instruction.
    pub fn run_tick(&mut self) -> Cycles {
        let mut cycle_count = Cycles(0);
        // Handling for string instructions might go here.

        // Execute a new instruction
        let (cycles, bytes) = self.read_val::<[u8;2]>(self.pc().0 as usize);
        cycle_count += cycles;
        if let Ok(x) = bytes {
                let instr = InstructionType::from_instrid(x);
                if let Some(i) = instr {
                    let cycles = self.execute_instr(i, self.pc());

                    cycle_count += cycles;
                } else {
                    unimplemented!("Instruction decoding failed and no handler implemented.");
                }
        } else {
            unimplemented!("Fetching left opcode byte failed and no handler implemented.");
        }

        return cycle_count;
    }

    pub fn execute_instr(&mut self, instr: InstructionType, pc: Wrapping<u32>) -> Cycles {
        let saved_pc = pc;
        let mut pc = saved_pc + Wrapping(instr.instr_len() as u32);
        let mut gpr = self.gpr;
        let mut psl = self.psl.clone();

        let cycles = match instr {
            InstructionType::HALT => {
                // TODO: Check privilege level
                self.halted = true; // Halt the CPU
                Cycles(1)
            },
            InstructionType::NOP => {
                Cycles(1)
            }
            InstructionType::MOVL => {
                let mut cycle_count = Cycles(1);
                let (cycles, src) = fetch_and_parse_op(&mut pc, self);
                cycle_count += cycles;

                if let Err(_) = src {
                    unimplemented!("No handling for failed opcode fetch implemented.");
                }

                let (cycles, val) = src.unwrap().load_u32(&mut pc, &mut gpr, self);
                cycle_count += cycles;

                if val == Err(()) {
                    unimplemented!("No fetch failure handlign implemented yet.");
                }

                let (cycles, dst) = fetch_and_parse_op(&mut pc, self);
                cycle_count += cycles;

                if let Err(_) = dst {
                    unimplemented!("No handling for failed opcode fetch implemented.");
                }

                let (cycles, res) = dst.unwrap().store_u32(&mut pc, &mut gpr, self, val.unwrap());
                cycle_count += cycles;
                
                if res == Err(()) {
                    unimplemented!("No store failure handling implemented yet.");
                }
                cycle_count
            }
            _ => {
                unimplemented!("Not all instructions implemented yet, and no error handler implemented.");
            }
        };

        self.set_gpr(gpr); // Commit GPR
        self.set_pc(pc); // Commit PC
        self.set_psl(psl); // Commit PSL
        

        return cycles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::VAXBus;
    use test::Bencher;

    #[test]
    fn exectest() {
        let mut cpu = VAXCPU::new(VAXBus::new(1024, 2048, 0, 1024));
        let mut rom = cpu.bus.rom_mut();
        let bytes = &[0xD0, 0x8F, 0x01, 0x00, 0x00, 0x00, 0x50];
        (*rom)[0..7].copy_from_slice(bytes);
        (*rom)[8..15].copy_from_slice(bytes);

        while cpu.halted == false {
            cpu.run_tick();
        }
        assert_eq!(cpu.gpr()[0].0, 1);
    }

    #[bench]
    fn execbench(b: &mut Bencher) {
        let mut cpu = VAXCPU::new(VAXBus::new(1024, 2048, 0, 1024));
        let mut rom = cpu.bus.rom_mut();
        let bytes = &[0xD0, 0x8F, 0x01, 0x00, 0x00, 0x00, 0x50];
        rom[..7*32].chunks_mut(7).for_each(|ch| ch.copy_from_slice(bytes));

        b.iter(|| {
            cpu.set_pc(Wrapping(0));
            cpu.halted = false;

            test::black_box(while cpu.halted == false {
                cpu.run_tick();
            });
            
        });
    }
}