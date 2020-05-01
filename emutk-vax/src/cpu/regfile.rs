use crate::cpu::{
    PSL,
};
use crate::VAXNum;
use bytemuck::{
    cast_slice,
    cast_slice_mut,
};

pub struct VAXRegisterFile {
    gpr: [u32;14],
    stkptrs: [u32;5],

    pc: u32,
    psl: PSL,

    // MMU things.
    /// P0 Base Register
    p0br: u32,
    /// P0 Length Register
    p0lr: u32,
    /// P1 Base Register
    p1br: u32,
    /// P1 Length Register
    p1lr: u32,

    /// System Base Register
    sbr: u32,
    /// System Length Register
    slr: u32,

    /// Process Control Block Base
    pcbb: u32,
    /// System Control Block Base
    scbb: u32,

    /// Memory Management Enable
    mapen: bool,

    /// Software Interrupt Request
    sirr: u32,
    /// Software Interrupt Summary
    sisr: u32,

    /// Translation Buffer Invalidate All
    tbia: u32,
    /// Translation Buffer Invalidate Single
    tbis: u32,
    /// Translation Buffer Check
    tbchk: u32,

}

macro_rules! gpr_funcs {
    (;$($reg:literal, $get_reg:ident, $set_reg:ident);+) => {
        $(
        #[inline]
        pub fn $get_reg(&self) -> u32 {
            self.gpr[$reg]
        }
        #[inline]
        pub fn $set_reg(&mut self, val: u32) {
            self.gpr[$reg] = val;
        }
        )+
    }
}

macro_rules! sp_funcs {
    (;$($reg:literal, $get_reg:ident, $set_reg:ident);+) => {
        $(
        #[inline]
        pub fn $get_reg(&self) -> u32 {
            self.stkptrs[$reg]
        }
        #[inline]
        pub fn $set_reg(&mut self, val: u32) {
            self.stkptrs[$reg] = val;
        }
        )+
    }
}

macro_rules! reg_funcs {
    (;$($reg:ident, $get_reg:ident, $set_reg:ident);+) => {
        $(
        #[inline]
        pub fn $get_reg(&self) -> u32 {
            self.$reg
        }
        #[inline]
        pub fn $set_reg(&mut self, val: u32) {
            self.$reg = val;
        }
        )+
    }
}

impl VAXRegisterFile {
    gpr_funcs!(
        ; 0, get_r0, set_r0
        ; 1, get_r1, set_r1
        ; 2, get_r2, set_r2
        ; 3, get_r3, set_r3
        ; 4, get_r4, set_r4
        ; 5, get_r5, set_r5
        ; 6, get_r6, set_r6
        ; 7, get_r7, set_r7
        ; 8, get_r8, set_r8
        ; 9, get_r9, set_r9
        ; 10, get_r10, set_r10
        ; 11, get_r11, set_r11
        ; 12, get_r12, set_r12
        ; 13, get_r13, set_r13
    );

    sp_funcs!(
        ; 0, get_ksp, set_ksp
        ; 1, get_esp, set_esp
        ; 2, get_ssp, set_ssp
        ; 3, get_usp, set_usp
        ; 4, get_isp, set_isp
    );

    reg_funcs!(
        ; pc, get_pc, set_pc
        ; p0br, get_p0br, set_p0br
        ; p0lr, get_p0lr, set_p0lr
        ; p1br, get_p1br, set_p1br
        ; p1lr, get_p1lr, set_p1lr
        ; sbr, get_sbr, set_sbr
        ; slr, get_slr, set_slr
        ; pcbb, get_pcbb, set_pcbb
        ; scbb, get_scbb, set_scbb
        ; sirr, get_sirr, set_sirr
        ; sisr, get_sisr, set_sisr
        ; tbia, get_tbia, set_tbia
        ; tbis, get_tbis, set_tbis
        ; tbchk, get_tbchk, set_tbchk
    );
    
    pub fn get_mapen(&self) -> bool {
        self.mapen
    }

    pub fn set_mapen(&mut self, val: bool) {
        self.mapen = val
    }

    pub fn get_psl(&self) -> &PSL {
        &self.psl
    }

    pub fn get_psl_mut(&mut self) -> &mut PSL {
        &mut self.psl
    }
}

/// CPU-level register file reads/writes
impl VAXRegisterFile {
    pub fn get_sp(&self) -> u32 {
        if self.psl.get_is() {
            self.get_isp()
        } else {
            let id = self.psl.get_cur_mod();
            self.stkptrs[id as usize]
        }
    }

    pub fn set_sp(&mut self, val: u32) {
        if self.psl.get_is() {
            self.set_isp(val)
        } else {
            let id = self.psl.get_cur_mod();
            self.stkptrs[id as usize] = val;
        }
    }

    pub fn read_gpr(&self, gpr: u8) -> u32 {
        assert!(gpr < 16);
        match gpr {
            14 => {
                self.get_sp()
            }
            15 => {
                self.pc
            }
            16u8..=std::u8::MAX => unreachable!(),
            v => self.gpr[v as usize],
           
        }
    }

    pub fn write_gpr(&mut self, gpr: u8, val: u32) {
        assert!(gpr < 16);
        match gpr {
            14 => {
                self.set_sp(val)
            }
            15 => {
                // Anyone who writes VAX ASM that invokes this 
                // particular set better be ready for it to break.
                self.pc = val 
            }
            16u8..=std::u8::MAX => unreachable!(),
            v => self.gpr[v as usize] = val,   
        }
    }

    pub fn read_gpr_ext<T: VAXNum>(&self, start_gpr: u8) -> T
    {   
        if T::BYTE_LEN <= 4 {
            return T::primitive_from(self.read_gpr(start_gpr));
        } else {
            // Ugh, big number.
            let gpr_count = T::BYTE_LEN / 4;
            if start_gpr as usize + gpr_count - 1 > 13 {
                // Reading back SP and PC in a multi-register read is UB.
                // As such, wimp out and return 0. Trying to return a full u128 or whatever
                // is extra work, for no gain.
                return T::primitive_from(0_u32);
            }
            let a = start_gpr as usize;
            let b = gpr_count as usize + a - 1;

            let regs: &[u32] = &self.gpr[a..b];
            let slice = cast_slice::<_, u8>(regs);
            return T::from_le_bytes(slice);
        }
    }

    pub fn write_gpr_ext<T: VAXNum>(&mut self, start_gpr: u8, value: T)
    {
            if T::BYTE_LEN <= 4 {
                self.write_gpr(start_gpr, value.as_());
            } else {
                let gpr_count = T::BYTE_LEN / 4;
                if start_gpr as usize + gpr_count - 1 > 13 {
                    return; // Abuse UB to be lazy.
                }
                let a = start_gpr as usize;
                let b = gpr_count as usize + a;

                let regs: &mut [u32] = &mut self.gpr[a..b];
                let slice = cast_slice_mut::<_, u8>(regs);
                value.copy_to_le_bytes(slice);
            }
        
    }
}

impl VAXRegisterFile {
    pub fn new() -> Self {
        VAXRegisterFile {
            gpr: [0;14],
            stkptrs: [0;5],
        
            pc: 0,
            psl: PSL(0),
        
            // MMU things.
            /// P0 Base Register
            p0br: 0,
            /// P0 Length Register
            p0lr: 0,
            /// P1 Base Register
            p1br: 0,
            /// P1 Length Register
            p1lr: 0,
        
            /// System Base Register
            sbr: 0,
            /// System Length Register
            slr: 0,
        
            /// Process Control Block Base
            pcbb: 0,
            /// System Control Block Base
            scbb: 0,
        
            /// Memory Management Enable
            mapen: false,
        
            /// Software Interrupt Request
            sirr: 0,
            /// Software Interrupt Summary
            sisr: 0,
        
            /// Translation Buffer Invalidate All
            tbia: 0,
            /// Translation Buffer Invalidate Single
            tbis: 0,
            /// Translation Buffer Check
            tbchk: 0,
        }
    }
}