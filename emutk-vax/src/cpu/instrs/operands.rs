
use crate::{
    Error,
    VAXNum,
    cpu::{
        VAXCPU,
    },
};
use emutk_core::{
    ByteReprNum,
};
use crate::bus::VAXBus;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperandMode {
    Literal(u8), // 0..=3
    // 4
    IndexedRegisterDeferred(u8, u8),
    IndexedAutodecrement(u8, u8),
    IndexedAutoincrement(u8, u8),
    IndexedAutoincrementDeferred(u8, u8),
    IndexedByteDisplacement(u8, u8),
    IndexedByteDisplacementDeferred(u8, u8),
    IndexedWordDisplacement(u8, u8),
    IndexedWordDisplacementDeferred(u8, u8),
    IndexedLongwordDisplacement(u8, u8),
    IndexedLongwordDisplacementDeferred(u8, u8),

    Register(u8), // 5
    RegisterDeferred(u8), // 6
    Autodecrement(u8), // 7
    Autoincrement(u8), // 8
    AutoincrementDeferred(u8), // 9
    ByteDisplacement(u8), // 10
    ByteDisplacementDeferred(u8), // 11
    WordDisplacement(u8), // 12
    WordDisplacementDeferred(u8), // 13
    LongwordDisplacement(u8), // 14
    LongwordDisplacementDeferred(u8), // 15
}

impl OperandMode {
    pub fn identify_operand(op_head: [u8; 2]) -> Result<OperandMode, Error> {
        let reg = op_head[0] & 0xF;
        
        Ok(match (op_head[0] & 0xF0) >> 4 {
            0..=3 => OperandMode::Literal(op_head[0] & !0b1100_0000),
            4 => {
                let indexed_reg = op_head[1] & 0xF;
                let indexed_op = (op_head[1] & 0xF0) >> 4;
                match indexed_op {
                    6 => OperandMode::IndexedRegisterDeferred(reg, indexed_reg),
                    7 => OperandMode::IndexedAutodecrement(reg, indexed_reg),
                    8 => OperandMode::IndexedAutoincrement(reg, indexed_reg),
                    9 => OperandMode::IndexedAutoincrementDeferred(reg, indexed_reg),
                    10 => OperandMode::IndexedByteDisplacement(reg, indexed_reg),
                    11 => OperandMode::IndexedByteDisplacementDeferred(reg, indexed_reg),
                    12 => OperandMode::IndexedWordDisplacement(reg, indexed_reg),
                    13 => OperandMode::IndexedWordDisplacementDeferred(reg, indexed_reg),
                    14 => OperandMode::IndexedLongwordDisplacement(reg, indexed_reg),
                    15 => OperandMode::IndexedLongwordDisplacementDeferred(reg, indexed_reg),
                    _ => return Err(Error::new_address_mode_fault()),
                }
            },
            5 => OperandMode::Register(reg),
            6 => OperandMode::RegisterDeferred(reg),
            7 => OperandMode::Autodecrement(reg),
            8 => OperandMode::Autoincrement(reg),
            9 => OperandMode::AutoincrementDeferred(reg),
            10 => OperandMode::ByteDisplacement(reg),
            11 => OperandMode::ByteDisplacementDeferred(reg),
            12 => OperandMode::WordDisplacement(reg),
            13 => OperandMode::WordDisplacementDeferred(reg),
            14 => OperandMode::LongwordDisplacement(reg),
            15 => OperandMode::LongwordDisplacementDeferred(reg),
            16..=std::u8::MAX => unreachable!(),
        })
    }

    pub fn byte_size<T: ByteReprNum>(self) -> usize {
        use OperandMode::*;
        match self {
            Literal(_) | Register(_) | RegisterDeferred(_) |
            Autodecrement(_) | Autoincrement(_) | AutoincrementDeferred(_) =>
                1, // returns an Option. always Some.
            ByteDisplacement(_) | ByteDisplacementDeferred(_) =>
                2,
            WordDisplacement(_) | WordDisplacementDeferred(_) =>
                3,
            LongwordDisplacement(_) | LongwordDisplacementDeferred(_) =>
                5,
            
            IndexedRegisterDeferred(_, _) | IndexedAutodecrement(_, _) |
            IndexedAutoincrement(_, _) | IndexedAutoincrementDeferred(_, _) 
            => 2,

            IndexedByteDisplacement(_, _) | IndexedByteDisplacementDeferred(_, _)
                => 3,
            IndexedWordDisplacement(_, _) | IndexedWordDisplacementDeferred(_, _)
                => 4,
            IndexedLongwordDisplacement(_, _) | IndexedLongwordDisplacementDeferred(_, _)
                => 6,
        }
    }

    /// NOTE ON SIDE EFFECTS:
    /// Autoincrement and autodecrement operations *update registers*. Do *not* call out of order.
    pub fn create_unresolved_read<B: VAXBus, T: VAXNum>
        (&self, cpu: &mut VAXCPU<B>) -> Result<UnresolvedOperandRead<T>, Error>
    {
        use OperandMode::*;
        let pc = cpu.regfile.get_pc();
        match self {
            IndexedRegisterDeferred(_, _) => todo!(),
            IndexedAutodecrement(_, _) => todo!(),
            IndexedAutoincrement(_, _) => todo!(),
            IndexedAutoincrementDeferred(_, _) => todo!(),
            IndexedByteDisplacement(_, _) => todo!(),
            IndexedByteDisplacementDeferred(_, _) => todo!(),
            IndexedWordDisplacement(_, _) => todo!(),
            IndexedWordDisplacementDeferred(_, _) => todo!(),
            IndexedLongwordDisplacement(_, _) => todo!(),
            IndexedLongwordDisplacementDeferred(_, _) => todo!(),
            Literal(v) => Ok(UnresolvedOperandRead::Value(T::primitive_from(*v))),
            Register(r) => {
                let v = cpu.regfile.read_gpr_ext(*r);
                Ok(UnresolvedOperandRead::Value(v))
            },
            RegisterDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                Ok(UnresolvedOperandRead::MemRead(v))
            },
            Autoincrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandRead::MemRead(v))
            },
            Autodecrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_sub(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandRead::MemRead(v))
            },
            AutoincrementDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandRead::DeferredMemRead(v))
            },
            ByteDisplacement(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandRead::MemRead(adj_addr))
            }

            ByteDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandRead::DeferredMemRead(adj_addr))
            }

            WordDisplacement(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandRead::MemRead(adj_addr))
            }

            WordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandRead::DeferredMemRead(adj_addr))
            }

            LongwordDisplacement(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandRead::MemRead(adj_addr))
            }

            LongwordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandRead::DeferredMemRead(adj_addr))
            }
        }
    }

    pub fn create_unresolved_write<B: VAXBus, T: VAXNum>
        (&self, cpu: &mut VAXCPU<B>) -> Result<UnresolvedOperandWrite<T>, Error>
    {
        use OperandMode::*;
        let pc = cpu.regfile.get_pc();
        match self {
            IndexedRegisterDeferred(_, _) => todo!(),
            IndexedAutodecrement(_, _) => todo!(),
            IndexedAutoincrement(_, _) => todo!(),
            IndexedAutoincrementDeferred(_, _) => todo!(),
            IndexedByteDisplacement(_, _) => todo!(),
            IndexedByteDisplacementDeferred(_, _) => todo!(),
            IndexedWordDisplacement(_, _) => todo!(),
            IndexedWordDisplacementDeferred(_, _) => todo!(),
            IndexedLongwordDisplacement(_, _) => todo!(),
            IndexedLongwordDisplacementDeferred(_, _) => todo!(),
            Literal(_) => Err(Error::new_address_mode_fault()),
            Register(r) => {
                Ok(UnresolvedOperandWrite::RegWrite(*r))
            },
            RegisterDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                Ok(UnresolvedOperandWrite::MemWrite(v))
            },
            // increment/decrement handled in .finalize()
            Autoincrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandWrite::MemWrite(v))
            },
            Autodecrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_sub(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandWrite::MemWrite(v))
            },
            AutoincrementDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandWrite::DeferredMemWrite(v))
            },
            ByteDisplacement(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandWrite::MemWrite(adj_addr))
            }

            ByteDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandWrite::DeferredMemWrite(adj_addr))
            }

            WordDisplacement(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandWrite::MemWrite(adj_addr))
            }

            WordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandWrite::DeferredMemWrite(adj_addr))
            }

            LongwordDisplacement(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandWrite::MemWrite(adj_addr))
            }

            LongwordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandWrite::DeferredMemWrite(adj_addr))
            }
        }
    }
    pub fn create_unresolved_modify<B: VAXBus, T: VAXNum>
        (&self, cpu: &mut VAXCPU<B>) -> Result<UnresolvedOperandModify<T>, Error>
    {
        use OperandMode::*;
        let pc = cpu.regfile.get_pc();
        match self {
            IndexedRegisterDeferred(_, _) => todo!(),
            IndexedAutodecrement(_, _) => todo!(),
            IndexedAutoincrement(_, _) => todo!(),
            IndexedAutoincrementDeferred(_, _) => todo!(),
            IndexedByteDisplacement(_, _) => todo!(),
            IndexedByteDisplacementDeferred(_, _) => todo!(),
            IndexedWordDisplacement(_, _) => todo!(),
            IndexedWordDisplacementDeferred(_, _) => todo!(),
            IndexedLongwordDisplacement(_, _) => todo!(),
            IndexedLongwordDisplacementDeferred(_, _) => todo!(),
            Literal(_) => Err(Error::new_address_mode_fault()),
            Register(r) => {
                let v = cpu.regfile.read_gpr_ext(*r);
                Ok(UnresolvedOperandModify::Value(v, *r))
            },
            RegisterDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                Ok(UnresolvedOperandModify::Mem(v))
            },
            // increment/decrement handled in .finalize()
            Autoincrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandModify::Mem(v))
            },
            Autodecrement(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_sub(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandModify::Mem(v))
            },
            AutoincrementDeferred(r) => {
                let v = cpu.regfile.read_gpr(*r);
                cpu.regfile.write_gpr(*r, v.wrapping_add(T::BYTE_LEN as u32));
                Ok(UnresolvedOperandModify::DeferredMem(v))
            },
            ByteDisplacement(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandModify::Mem(adj_addr))
            }

            ByteDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i8>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandModify::DeferredMem(adj_addr))
            }

            WordDisplacement(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandModify::Mem(adj_addr))
            }

            WordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i16>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp as i32) as u32;
                Ok(UnresolvedOperandModify::DeferredMem(adj_addr))
            }

            LongwordDisplacement(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandModify::Mem(adj_addr))
            }

            LongwordDisplacementDeferred(r) => {
                let disp = cpu.read_val::<i32>(pc)?;
                let addr = cpu.regfile.read_gpr(*r);
                let adj_addr = (addr as i32).wrapping_add(disp) as u32;
                Ok(UnresolvedOperandModify::DeferredMem(adj_addr))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnresolvedOperandRead<T: VAXNum> {
    MemRead(u32),
    DeferredMemRead(u32),
    Value(T),
}

impl<T: VAXNum> UnresolvedOperandRead<T> {
    // side effect note: Replaces DeferredAddress with Address
    pub fn validate<B: VAXBus>(&mut self, cpu: &mut VAXCPU<B>) -> Result<(), Error> 
    {
        match self {
            UnresolvedOperandRead::MemRead(addr) => {
                cpu.can_read_val::<T>(*addr)
            }
            UnresolvedOperandRead::DeferredMemRead(addr) => {
                let addr2 = cpu.read_val::<u32>(*addr)?;
                cpu.can_read_val::<T>(addr2)?;    
                *self = UnresolvedOperandRead::MemRead(addr2);
                Ok(())
            }
            UnresolvedOperandRead::Value(_) => Ok(()),
        }
    }

    pub fn execute<B: VAXBus>(self, cpu: &mut VAXCPU<B>) -> T
    {
        match self {
            UnresolvedOperandRead::MemRead(addr) => {
                cpu.read_val(addr).unwrap()
            }
            UnresolvedOperandRead::Value(v) => v,
            _ => unreachable!(), // Reaching here means someone forgot to validate ):
        }
    }

    pub fn read<B: VAXBus>(mut self, cpu: &mut VAXCPU<B>) -> Result<T, Error>
    {
        self.validate(cpu)?;
        Ok(self.execute(cpu))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnresolvedOperandWrite<T: VAXNum> {
    MemWrite(u32),
    DeferredMemWrite(u32), // read address to get destination address
    RegWrite(u8),
    #[doc(hidden)]
    __Phantom(std::marker::PhantomData<T>)
}

impl<T: VAXNum>  UnresolvedOperandWrite<T> {
    pub fn validate<B: VAXBus>(&mut self, cpu: &mut VAXCPU<B>) -> Result<(), Error> 
    {
        match self {
            UnresolvedOperandWrite::MemWrite(addr) => {
                cpu.can_write_val::<T>(*addr)
            }
            UnresolvedOperandWrite::DeferredMemWrite(addr) => {
                let addr2 = cpu.read_val::<u32>(*addr)?;
                cpu.can_write_val::<T>(addr2)?;    
                *self = UnresolvedOperandWrite::MemWrite(addr2);
                Ok(())
            }
            UnresolvedOperandWrite::RegWrite(_) => Ok(()),
            _ => unreachable!(),
        }
    }

    pub fn execute<B: VAXBus>(&mut self, cpu: &mut VAXCPU<B>, value: T) 
    {
        match self {
            UnresolvedOperandWrite::RegWrite(r) => {
                cpu.regfile.write_gpr_ext::<T>(*r, value);
            }
            UnresolvedOperandWrite::MemWrite(addr) => {
                cpu.write_val(*addr, value).unwrap();
            }
            _ => {},
        }
    }

    pub fn write<B: VAXBus>(mut self, cpu: &mut VAXCPU<B>, value: T) -> Result<(), Error>
    {
        self.validate(cpu)?;
        self.execute(cpu, value);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnresolvedOperandModify<T: VAXNum> {
    Mem(u32),
    DeferredMem(u32),
    Value(T, u8),
}

impl<T: VAXNum>  UnresolvedOperandModify<T> {
    pub fn validate<B: VAXBus>(&mut self, cpu: &mut VAXCPU<B>) -> Result<(), Error> 
    {
        match self {
            UnresolvedOperandModify::Mem(addr) => {
                cpu.can_write_val::<T>(*addr)
            }
            UnresolvedOperandModify::DeferredMem(addr) => {
                let addr2 = cpu.read_val::<u32>(*addr)?;
                cpu.can_write_val::<T>(addr2)?;    
                *self = UnresolvedOperandModify::Mem(addr2);
                Ok(())
            }
            UnresolvedOperandModify::Value(_,_) => Ok(()),
        }
    }

    pub fn execute_write<B: VAXBus>(&mut self, cpu: &mut VAXCPU<B>, value: T) 
    {
        match self {
            UnresolvedOperandModify::Value(_, r) => {
                cpu.regfile.write_gpr_ext::<T>(*r, value);
            }
            UnresolvedOperandModify::Mem(addr) => {
                cpu.write_val(*addr, value).unwrap();
            }
            _ => {},
        }
    }

    pub fn write<B: VAXBus>(mut self, cpu: &mut VAXCPU<B>, value: T) -> Result<(), Error>
    {
        self.validate(cpu)?;
        self.execute_write(cpu, value);
        Ok(())
    }

    pub fn execute_read<B: VAXBus>(self, cpu: &mut VAXCPU<B>) -> T
    {
        match self {
            UnresolvedOperandModify::Mem(addr) => {
                cpu.read_val(addr).unwrap()
            }
            UnresolvedOperandModify::Value(v, _) => v,
            _ => unreachable!(), // Reaching here means someone forgot to validate ):
        }
    }

    pub fn read<B: VAXBus>(mut self, cpu: &mut VAXCPU<B>) -> Result<T, Error>
    {
        self.validate(cpu)?;
        Ok(self.execute_read(cpu))
    }
}