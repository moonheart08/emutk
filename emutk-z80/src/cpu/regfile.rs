use emutk_core::{
    ByteRepr,
    split::*,
};

pub struct Z80Registers {
    pc: u16,
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,

    ix: u16,
    iy: u16,

    i: u8,
    r: u8,

    af_bk: u16,
    bc_bk: u16,
    de_bk: u16,
    hl_bk: u16,
}

macro_rules! flag_funcs {
    ($var_get:ident, $var_set:ident ; $($get:ident,$set:ident,$bit:literal);+) => {
        $(
        pub fn $get(&self) -> bool {
            self.$var_get() & $bit != 0
        }

        pub fn $set(&mut self, val: bool) {
            let mut v = self.$var_get() & !$bit;
            if val { v |= $bit; }
            self.$var_set(v);
        }
        )+
    };
}

macro_rules! reg_funcs {
    ($($pair:ident, $get_pair:ident, $set_pair:ident, $get_upper:ident, $set_upper:ident, $get_lower:ident, $set_lower:ident);+) => {
        $(
        #[inline]
        pub fn $get_pair(&self) -> u16 {
            self.$pair
        }
        #[inline]
        pub fn $set_pair(&mut self, val: u16) {
            self.$pair = val;
        }
        #[inline]
        pub fn $get_lower(&self) -> u8 {
            self.$pair.split_le()[0]
        }
        #[inline]
        pub fn $set_lower(&mut self, val: u8) {
            self.$pair.modify_lower_le(val);
        }
        #[inline]
        pub fn $get_upper(&self) -> u8 {
            self.$pair.split_le()[1]
        }
        #[inline]
        pub fn $set_upper(&mut self, val: u8) {
            self.$pair.modify_upper_le(val);
        }
        )+
    }
}

impl Z80Registers {
    #[inline]
    pub fn set_reg_f(&mut self, val: u8) {
        self.af.modify_upper_le(val);
    }

    reg_funcs!(
          af, get_af, set_af, get_a, set_a, get_f, set_f
        ; bc, get_bc, set_bc, get_b, set_b, get_c, set_c
        ; de, get_de, set_de, get_d, set_d, get_e, set_e
        ; hl, get_hl, set_hl, get_h, set_h, get_l, set_l
        ; ix, get_ix, set_ix, get_ixh, set_ixh, get_ixl, set_ixl
        ; iy, get_iy, set_iy, get_iyh, set_iyh, get_iyl, set_iyl
    );

    #[inline]
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    #[inline]
    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    #[inline]
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    flag_funcs!(get_f, set_f
        ; flag_c, set_flag_c, 0x01
        ; flag_n, set_flag_n, 0x02
        ; flag_v, set_flag_v, 0x04
        ; flag_f3, set_flag_f3, 0x08
        ; flag_h, set_flag_h, 0x10
        ; flag_f5, set_flag_f5, 0x20
        ; flag_z, set_flag_z, 0x40
        ; flag_s, set_flag_s, 0x80
    );

    pub fn new() -> Self {
        Z80Registers {
            pc: 0,
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,

            ix: 0,
            iy: 0,

            i: 0,
            r: 0,

            af_bk: 0,
            bc_bk: 0,
            de_bk: 0,
            hl_bk: 0,
        }
    }
}