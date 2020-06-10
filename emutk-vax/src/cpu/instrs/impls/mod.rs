mod arith;
mod misc;
mod control;
mod util;
mod bitfield;
mod convert;
#[cfg(test)]
mod tests;

use crate::cpu::VAXCPU;
use crate::cpu::instrs::InstructionType;
use tablegen_proc::gen_instr_table;
use crate::bus::VAXBus;
use crate::Error;
use control::BranchCondition;
use emutk_core::{
    cycles::Cycles,
};

/*
// Instruction form:
fn instr_name(&mut VAXCPU, &mut Cycles)

*/
use InstructionType::*;

impl<T: VAXBus> VAXCPU<'_, T> {
    pub(crate) fn setup_instr_table(&mut self) {
        let itable: [Option<fn(&mut VAXCPU<'_, T> , &mut Cycles) -> Result<(), Error>>; 1280]
         = gen_instr_table!{
            VAX_INSTR_MAP_TABLE; 1280; Option<fn(&mut VAXCPU<'_, T> , &mut Cycles) -> Result<(), Error>>; None => {
                0x00 => HALT, Some(misc::instr_halt);
                0x01 => NOP, Some(misc::instr_nop);
                0x02 => REI, Some(misc::instr_noimpl);
                0x03 => BPT, Some(misc::instr_noimpl);
                0x04 => RET, Some(misc::instr_noimpl);
                0x05 => RSB, Some(control::instr_rsb);
                0x06 => LDPCTX, Some(misc::instr_noimpl);
                0x07 => SVPCTX, Some(misc::instr_noimpl);
                0x08 => CVTPS, Some(misc::instr_noimpl);
                0x09 => CVTSP, Some(misc::instr_noimpl);
                0x0A => INDEX, Some(misc::instr_noimpl);
                0x0B => CRC, Some(misc::instr_noimpl);
                0x0C => PROBER, Some(misc::instr_noimpl);
                0x0D => PROBEW, Some(misc::instr_noimpl);
                0x0E => INSQUE, Some(misc::instr_noimpl);
                0x0F => REMQUE, Some(misc::instr_noimpl);
                0x10 => BSBB, Some(control::instr_branch_subroutine_byte);
                0x11 => BRB, Some(control::instr_branch_byte);
                0x12 => BNEQ, Some(control::instr_branch_cond_zus);
                0x13 => BEQL, Some(control::instr_branch_cond_zs);
                0x14 => BGTR, Some(control::instr_branch_cond_norzus);
                0x15 => BLEQ, Some(control::instr_branch_cond_norzs);
                0x16 => JSB, Some(control::instr_jsb);
                0x17 => JMP, Some(control::instr_jmp);
                0x18 => BGEQ, Some(control::instr_branch_cond_nus);
                0x19 => BLSS, Some(control::instr_branch_cond_ns);
                0x1A => BGTRU, Some(control::instr_branch_cond_corzus);
                0x1B => BLEQU, Some(control::instr_branch_cond_corzs);
                0x1C => BVC, Some(control::instr_branch_cond_vus);
                0x1D => BVS, Some(control::instr_branch_cond_vs);
                0x1E => BGEQU, Some(control::instr_branch_cond_cus);
                0x1F => BLSSU, Some(control::instr_branch_cond_cs);
                0x20 => ADDP4, Some(misc::instr_noimpl);
                0x21 => ADDP6, Some(misc::instr_noimpl);
                0x22 => SUBP4, Some(misc::instr_noimpl);
                0x23 => SUBP6, Some(misc::instr_noimpl);
                0x24 => CVTPT, Some(misc::instr_noimpl);
                0x25 => MULP, Some(misc::instr_noimpl);
                0x26 => CVTTP, Some(misc::instr_noimpl);
                0x27 => DIVP, Some(misc::instr_noimpl);
                0x28 => MOVC3, Some(misc::instr_noimpl);
                0x29 => CMPC3, Some(misc::instr_noimpl);
                0x2A => SCANC, Some(misc::instr_noimpl);
                0x2B => SPANC, Some(misc::instr_noimpl);
                0x2C => MOVC5, Some(misc::instr_noimpl);
                0x2D => CMPC5, Some(misc::instr_noimpl);
                0x2E => MOVTC, Some(misc::instr_noimpl);
                0x2F => MOVTUC, Some(misc::instr_noimpl);
                0x30 => BSBW, Some(control::instr_branch_subroutine_word);
                0x31 => BRW, Some(control::instr_branch_word);
                0x32 => CVTWL, Some(convert::instr_cvtwl);
                0x33 => CVTWB, Some(convert::instr_cvtwb);
                0x34 => MOVP, Some(misc::instr_noimpl);
                0x35 => CMPP3, Some(misc::instr_noimpl);
                0x36 => CVTPL, Some(misc::instr_noimpl);
                0x37 => CMPP4, Some(misc::instr_noimpl);
                0x38 => EDITPC, Some(misc::instr_noimpl);
                0x39 => MATCHC, Some(misc::instr_noimpl);
                0x3A => LOCC, Some(misc::instr_noimpl);
                0x3B => SKPC, Some(misc::instr_noimpl);
                0x3C => MOVZWL, Some(convert::instr_movzwl);
                0x3D => ACBW, Some(misc::instr_noimpl);
                0x3E => MOVAW, Some(misc::instr_mova::<_, u16>);
                0x3F => PUSHAW, Some(misc::instr_pusha::<_, u16>);
                0x40 => ADDF2, Some(misc::instr_noimpl);
                0x41 => ADDF3, Some(misc::instr_noimpl);
                0x42 => SUBF2, Some(misc::instr_noimpl);
                0x43 => SUBF3, Some(misc::instr_noimpl);
                0x44 => MULF2, Some(misc::instr_noimpl);
                0x45 => MULF3, Some(misc::instr_noimpl);
                0x46 => DIVF2, Some(misc::instr_noimpl);
                0x47 => DIVF3, Some(misc::instr_noimpl);
                0x48 => CVTFB, Some(misc::instr_noimpl);
                0x49 => CVTFW, Some(misc::instr_noimpl);
                0x4A => CVTFL, Some(misc::instr_noimpl);
                0x4B => CVTRFL, Some(misc::instr_noimpl);
                0x4C => CVTBF, Some(misc::instr_noimpl);
                0x4D => CVTWF, Some(misc::instr_noimpl);
                0x4E => CVTLF, Some(misc::instr_noimpl);
                0x4F => ACBF, Some(misc::instr_noimpl);
                0x50 => MOVF, Some(misc::instr_noimpl);
                0x51 => CMPF, Some(misc::instr_noimpl);
                0x52 => MNEGF, Some(misc::instr_noimpl);
                0x53 => TSTF, Some(misc::instr_noimpl);
                0x54 => EMODF, Some(misc::instr_noimpl);
                0x55 => POLYF, Some(misc::instr_noimpl);
                0x56 => CVTFD, Some(misc::instr_noimpl);
                //0x57
                0x58 => ADAWI, Some(arith::instr_add2::<_, u32>);
                //0x59
                //0x5A
                //0x5B
                0x5C => INSQHI, Some(misc::instr_noimpl);
                0x5D => INSQTI, Some(misc::instr_noimpl);
                0x5E => REMQHI, Some(misc::instr_noimpl);
                0x5F => REMQTI, Some(misc::instr_noimpl);
                0x60 => ADDD2, Some(misc::instr_noimpl);
                0x61 => ADDD3, Some(misc::instr_noimpl);
                0x62 => SUBD2, Some(misc::instr_noimpl);
                0x63 => SUBD3, Some(misc::instr_noimpl);
                0x64 => MULD2, Some(misc::instr_noimpl);
                0x65 => MULD3, Some(misc::instr_noimpl);
                0x66 => DIVD2, Some(misc::instr_noimpl);
                0x67 => DIVD3, Some(misc::instr_noimpl);
                0x68 => CVTDB, Some(misc::instr_noimpl);
                0x69 => CVTDW, Some(misc::instr_noimpl);
                0x6A => CVTDL, Some(misc::instr_noimpl);
                0x6B => CVTRDL, Some(misc::instr_noimpl);
                0x6C => CVTBD, Some(misc::instr_noimpl);
                0x6D => CVTWD, Some(misc::instr_noimpl);
                0x6E => CVTLD, Some(misc::instr_noimpl);
                0x6F => ACBD, Some(misc::instr_noimpl);
                0x70 => MOVD, Some(misc::instr_noimpl);
                0x71 => CMPD, Some(misc::instr_noimpl);
                0x72 => MNEGD, Some(misc::instr_noimpl);
                0x73 => TSTD, Some(misc::instr_noimpl);
                0x74 => EMODD, Some(misc::instr_noimpl);
                0x75 => POLYD, Some(misc::instr_noimpl);
                0x76 => CVTDF, Some(misc::instr_noimpl);
                // 0x77
                0x78 => ASHL, Some(arith::instr_ash::<_, u32>);
                0x79 => ASHQ, Some(arith::instr_ash::<_, u64>);
                0x7A => EMUL, Some(arith::instr_emul);
                0x7B => EDIV, Some(misc::instr_noimpl);
                0x7C => CLRQ, Some(misc::instr_clr::<_, u64>);
                0x7D => MOVQ, Some(arith::instr_mov::<_, u64>);
                0x7E => MOVAQ, Some(misc::instr_mova::<_, u64>);
                0x7F => PUSHAQ, Some(misc::instr_pusha::<_, u64>);
                0x80 => ADDB2, Some(arith::instr_add2::<_, u8>);
                0x81 => ADDB2, Some(arith::instr_add3::<_, u8>);
                0x82 => SUBB2, Some(arith::instr_sub2::<_, u8>);
                0x83 => SUBB3, Some(arith::instr_sub3::<_, u8>);
                0x84 => MULB2, Some(arith::instr_mul2::<_, u8>);
                0x85 => MULB3, Some(arith::instr_mul3::<_, u8>);
                0x86 => DIVB2, Some(arith::instr_div2::<_, u8>);
                0x87 => DIVB3, Some(arith::instr_div3::<_, u8>);
                0x88 => BISB2, Some(arith::instr_bis2::<_, u8>);
                0x89 => BISB3, Some(arith::instr_bis3::<_, u8>);
                0x8A => BICB2, Some(arith::instr_bic2::<_, u8>);
                0x8B => BICB3, Some(arith::instr_bic3::<_, u8>);
                0x8C => XORB2, Some(arith::instr_xor2::<_, u8>);
                0x8D => XORB3, Some(arith::instr_xor3::<_, u8>);
                0x8E => MNEGB, Some(arith::instr_mneg::<_, u8>);
                0x8F => CASEB, Some(control::instr_case::<u8, _>);
                0x90 => MOVB, Some(arith::instr_mov::<_, u8>);
                0x91 => CMPB, Some(arith::instr_cmp::<_, u8>);
                0x92 => MCOMB, Some(arith::instr_mcom::<_, u8>);
                0x93 => BITB, Some(arith::instr_bit::<_, u8>);
                0x94 => CLRB, Some(misc::instr_clr::<_, u8>);
                0x95 => TSTB, Some(arith::instr_tst::<_, u8>);
                0x96 => INCB, Some(arith::instr_inc::<_, u8>);
                0x97 => DECB, Some(arith::instr_dec::<_, u8>);
                0x98 => CVTBL, Some(convert::instr_cvtbl);
                0x99 => CVTBW, Some(convert::instr_cvtbw);
                0x9A => MOVZBL, Some(convert::instr_movzbl);
                0x9B => MOVZBW, Some(convert::instr_movzbw);
                0x9C => ROTL, Some(misc::instr_noimpl);
                0x9D => ACBB, Some(misc::instr_noimpl);
                0x9E => MOVAB, Some(misc::instr_mova::<_, u8>);
                0x9F => PUSHAB, Some(misc::instr_pusha::<_, u8>);
                0xA0 => ADDW2, Some(arith::instr_add2::<_, u16>);
                0xA1 => ADDW3, Some(arith::instr_add3::<_, u16>);
                0xA2 => SUBW2, Some(arith::instr_sub2::<_, u16>);
                0xA3 => SUBW3, Some(arith::instr_sub3::<_, u16>);
                0xA4 => MULW2, Some(arith::instr_mul2::<_, u16>);
                0xA5 => MULW3, Some(arith::instr_mul3::<_, u16>);
                0xA6 => DIVW2, Some(arith::instr_div2::<_, u16>);
                0xA7 => DIVW3, Some(arith::instr_div3::<_, u16>);
                0xA8 => BISW2, Some(arith::instr_bic2::<_, u16>);
                0xA9 => BISW3, Some(arith::instr_bic3::<_, u16>);
                0xAA => BICW2, Some(arith::instr_bic2::<_, u16>);
                0xAB => BICW3, Some(arith::instr_bic3::<_, u16>);
                0xAC => XORW2, Some(arith::instr_xor2::<_, u16>);
                0xAD => XORW3, Some(arith::instr_xor2::<_, u16>);
                0xAE => MNEGW, Some(arith::instr_mneg::<_, u16>);
                0xAF => CASEW, Some(control::instr_case::<u16, _>);
                0xB0 => MOVW, Some(arith::instr_mov::<_, u16>);
                0xB1 => CMPW, Some(arith::instr_cmp::<_, u16>);
                0xB2 => MCOMW, Some(arith::instr_mcom::<_, u16>);
                0xB3 => BITW, Some(arith::instr_bit::<_, u16>);
                0xB4 => CLRW, Some(misc::instr_clr::<_, u16>);
                0xB5 => TSTW, Some(arith::instr_tst::<_, u16>);
                0xB6 => INCW, Some(arith::instr_inc::<_, u16>);
                0xB7 => DECW, Some(arith::instr_dec::<_, u16>);
                0xB8 => BISPSW, Some(misc::instr_bispsw);
                0xB9 => BICPSW, Some(misc::instr_bicpsw);
                0xBA => POPR, Some(misc::instr_popr);
                0xBB => PUSHR, Some(misc::instr_pushr);
                0xBC => CHMK, Some(misc::instr_noimpl);
                0xBD => CHME, Some(misc::instr_noimpl);
                0xBE => CHMS, Some(misc::instr_noimpl);
                0xBF => CHMU, Some(misc::instr_noimpl);
                0xC0 => ADDL2, Some(arith::instr_add2::<_, u32>);
                0xC1 => ADDL3, Some(arith::instr_add3::<_, u32>);
                0xC2 => SUBL2, Some(arith::instr_sub2::<_, u32>);
                0xC3 => SUBL3, Some(arith::instr_sub3::<_, u32>);
                0xC4 => MULL2, Some(arith::instr_mul2::<_, u32>);
                0xC5 => MULL3, Some(arith::instr_mul3::<_, u32>);
                0xC6 => DIVL2, Some(arith::instr_div2::<_, u32>);
                0xC7 => DIVL3, Some(arith::instr_div3::<_, u32>);
                0xC8 => BISL2, Some(arith::instr_bis2::<_, u32>);
                0xC9 => BISL3, Some(arith::instr_bis3::<_, u32>);
                0xCA => BICL2, Some(arith::instr_bic2::<_, u32>);
                0xCB => BICL3, Some(arith::instr_bic3::<_, u32>);
                0xCC => XORL2, Some(arith::instr_xor2::<_, u32>);
                0xCD => XORL3, Some(arith::instr_xor2::<_, u32>);
                0xCE => MNEGL, Some(arith::instr_mneg::<_, u32>);
                0xCF => CASEL, Some(control::instr_case::<u32, _>);
                0xD0 => MOVL, Some(arith::instr_mov::<_, u32>);
                0xD1 => CMPL, Some(arith::instr_cmp::<_, u32>);
                0xD2 => MCOML, Some(arith::instr_mcom::<_, u32>);
                0xD3 => BITL, Some(arith::instr_bit::<_, u32>);
                0xD4 => CLRL, Some(misc::instr_clr::<_, u32>);
                0xD5 => TSTL, Some(arith::instr_tst::<_, u32>);
                0xD6 => INCL, Some(arith::instr_inc::<_, u32>);
                0xD7 => DECL, Some(arith::instr_dec::<_, u32>);
                0xD8 => ADWC, Some(arith::instr_adwc::<_, u16>);
                0xD9 => SBWC, Some(misc::instr_noimpl);
                0xDA => MTPR, Some(misc::instr_mtpr);
                0xDB => MFPR, Some(misc::instr_mfpr);
                0xDC => MOVPSL, Some(misc::instr_movpsl);
                0xDD => PUSHL, Some(misc::instr_pushl);
                0xDE => MOVAL, Some(misc::instr_mova::<_, u32>);
                0xDF => PUSHAL, Some(misc::instr_pusha::<_, u32>);
                0xE0 => BBS, Some(misc::instr_noimpl);
                0xE1 => BBC, Some(misc::instr_noimpl);
                0xE2 => BBSS, Some(misc::instr_noimpl);
                0xE3 => BBCS, Some(misc::instr_noimpl);
                0xE4 => BBSC, Some(misc::instr_noimpl);
                0xE5 => BBCC, Some(misc::instr_noimpl);
                0xE6 => BBSSI, Some(misc::instr_noimpl);
                0xE7 => BBCCI, Some(misc::instr_noimpl);
                0xE8 => BLBS, Some(control::instr_branch_low_bit_true);
                0xE9 => BLBC, Some(control::instr_branch_low_bit_false);
                0xEA => FFS, Some(misc::instr_noimpl);
                0xEB => FFC, Some(misc::instr_noimpl);
                0xEC => CMPV, Some(misc::instr_noimpl);
                0xED => CMPZV, Some(misc::instr_noimpl);
                0xEE => EXTV, Some(misc::instr_noimpl);
                0xEF => EXTZV, Some(misc::instr_noimpl);
                0xF0 => INSV, Some(misc::instr_noimpl);
                0xF1 => ACBL, Some(misc::instr_noimpl);
                0xF2 => AOBLSS, Some(misc::instr_noimpl);
                0xF3 => AOBLEQ, Some(misc::instr_noimpl);
                0xF4 => SOBGEQ, Some(misc::instr_noimpl);
                0xF5 => SOBGTR, Some(misc::instr_noimpl);
                0xF6 => CVTLB, Some(convert::instr_cvtlb);
                0xF7 => CVTLW, Some(convert::instr_cvtlw);
                0xF8 => ASHP, Some(misc::instr_noimpl);
                0xF9 => CVTLP, Some(misc::instr_noimpl);
                0xFA => CALLG, Some(misc::instr_noimpl);
                0xFB => CALLS, Some(misc::instr_noimpl);
                // 0xFC PREFIX
                // 0xFD PREFIX
                // 0xFE PREFIX
                // 0xFF PREFIX
                
                0x202 => WAIT, Some(misc::instr_noimpl);

                0x231 => MFVP, Some(misc::instr_noimpl);
                0x232 => CVTDH, Some(misc::instr_noimpl);
                0x233 => CVTGH, Some(misc::instr_noimpl);

                0x235 => VGATHL, Some(misc::instr_noimpl);

                0x237 => VGATHQ, Some(misc::instr_noimpl);

                0x240 => ADDG2, Some(misc::instr_noimpl);
                0x241 => ADDG3, Some(misc::instr_noimpl);
                0x242 => SUBG2, Some(misc::instr_noimpl);
                0x243 => SUBG3, Some(misc::instr_noimpl);
                0x244 => MULG2, Some(misc::instr_noimpl);
                0x245 => MULG3, Some(misc::instr_noimpl);
                0x246 => DIVG2, Some(misc::instr_noimpl);
                0x247 => DIVG3, Some(misc::instr_noimpl);
                0x248 => CVTGB, Some(misc::instr_noimpl);
                0x249 => CVTGW, Some(misc::instr_noimpl);
                0x24A => CVTGL, Some(misc::instr_noimpl);
                0x24B => CVTRGL, Some(misc::instr_noimpl);
                0x24C => CVTBG, Some(misc::instr_noimpl);
                0x24D => CVTWG, Some(misc::instr_noimpl);
                0x24E => CVTLG, Some(misc::instr_noimpl);
                0x24F => ACBG, Some(misc::instr_noimpl);
                0x250 => MOVG, Some(misc::instr_noimpl);
                0x251 => CMPG, Some(misc::instr_noimpl);
                0x252 => MNEGG, Some(misc::instr_noimpl);
                0x253 => TSTG, Some(misc::instr_noimpl);
                0x254 => EMODG, Some(misc::instr_noimpl);
                0x255 => POLYG, Some(misc::instr_noimpl);
                0x256 => CVTGH, Some(misc::instr_noimpl);

                0x260 => ADDH2, Some(misc::instr_noimpl);
                0x261 => ADDH3, Some(misc::instr_noimpl);
                0x262 => SUBH2, Some(misc::instr_noimpl);
                0x263 => SUBH3, Some(misc::instr_noimpl);
                0x264 => MULH2, Some(misc::instr_noimpl);
                0x265 => MULH3, Some(misc::instr_noimpl);
                0x266 => DIVH2, Some(misc::instr_noimpl);
                0x267 => DIVH3, Some(misc::instr_noimpl);
                0x268 => CVTHB, Some(misc::instr_noimpl);
                0x269 => CVTHW, Some(misc::instr_noimpl);
                0x26A => CVTHL, Some(misc::instr_noimpl);
                0x26B => CVTRHL, Some(misc::instr_noimpl);
                0x26C => CVTBH, Some(misc::instr_noimpl);
                0x26D => CVTWH, Some(misc::instr_noimpl);
                0x26E => CVTLH, Some(misc::instr_noimpl);
                0x26F => ACBH, Some(misc::instr_noimpl);
                0x270 => MOVH, Some(misc::instr_noimpl);
                0x271 => CMPH, Some(misc::instr_noimpl);
                0x272 => MNEGH, Some(misc::instr_noimpl);
                0x273 => TSTH, Some(misc::instr_noimpl);
                0x274 => EMODH, Some(misc::instr_noimpl);
                0x275 => POLYH, Some(misc::instr_noimpl);
                0x276 => CVTHG, Some(misc::instr_noimpl);

                0x27D => CLRO, Some(misc::instr_clr::<_, u128>);
                0x27D => MOVO, Some(arith::instr_mov::<_, u128>);
                0x27E => MOVAO, Some(misc::instr_mova::<_, u128>);
                0x27F => PUSHAO, Some(misc::instr_pusha::<_, u128>);
                0x280 => VVADDL, Some(misc::instr_noimpl);
                0x281 => VSADDL, Some(misc::instr_noimpl);
                0x282 => VVADDG, Some(misc::instr_noimpl);
                0x283 => VSADDG, Some(misc::instr_noimpl);
                0x284 => VVADDF, Some(misc::instr_noimpl);
                0x285 => VSADDF, Some(misc::instr_noimpl);
                0x286 => VVADDD, Some(misc::instr_noimpl);
                0x287 => VSADDD, Some(misc::instr_noimpl);
                0x288 => VVSUBL, Some(misc::instr_noimpl);
                0x289 => VSSUBL, Some(misc::instr_noimpl);
                0x28A => VVSUBG, Some(misc::instr_noimpl);
                0x28B => VSSUBG, Some(misc::instr_noimpl);
                0x28C => VVSUBF, Some(misc::instr_noimpl);
                0x28D => VSSUBF, Some(misc::instr_noimpl);
                0x28E => VVSUBD, Some(misc::instr_noimpl);
                0x28F => VSSUBD, Some(misc::instr_noimpl);
                
                0x298 => CVTFH, Some(misc::instr_noimpl);
                0x299 => CVTFG, Some(misc::instr_noimpl);
                0x29A => PROBEVMR, Some(misc::instr_noimpl);
                0x29B => PROBEVMW, Some(misc::instr_noimpl);
                0x29C => VSTL, Some(misc::instr_noimpl);
                0x29D => VSCATL, Some(misc::instr_noimpl);
                0x29E => VSTQ, Some(misc::instr_noimpl);
                0x29F => VSCATQ, Some(misc::instr_noimpl);
                0x2A0 => VVMULL, Some(misc::instr_noimpl);
                0x2A1 => VSMULL, Some(misc::instr_noimpl);
                0x2A2 => VVMULG, Some(misc::instr_noimpl);
                0x2A3 => VSMULG, Some(misc::instr_noimpl);
                0x2A4 => VVMULF, Some(misc::instr_noimpl);
                0x2A5 => VSMULF, Some(misc::instr_noimpl);
                0x2A6 => VVMULD, Some(misc::instr_noimpl);
                0x2A7 => VSMULD, Some(misc::instr_noimpl);
                0x2A8 => VSYNC, Some(misc::instr_noimpl);
                0x2A9 => MTVP, Some(misc::instr_noimpl);
                0x2AA => VVDIVG, Some(misc::instr_noimpl);
                0x2AB => VSDIVG, Some(misc::instr_noimpl);
                0x2AC => VVDIVF, Some(misc::instr_noimpl);
                0x2AD => VSDIVF, Some(misc::instr_noimpl);
                0x2AE => VVDIVD, Some(misc::instr_noimpl);
                0x2AF => VSDIVD, Some(misc::instr_noimpl);
                0x2C0 => VVCMPL, Some(misc::instr_noimpl);
                0x2C1 => VSCMPL, Some(misc::instr_noimpl);
                0x2C2 => VVCMPG, Some(misc::instr_noimpl);
                0x2C3 => VSCMPG, Some(misc::instr_noimpl);
                0x2C4 => VVCMPF, Some(misc::instr_noimpl);
                0x2C5 => VSCMPF, Some(misc::instr_noimpl);
                0x2C6 => VVCMPD, Some(misc::instr_noimpl);
                0x2C7 => VSCMPD, Some(misc::instr_noimpl);
                0x2C8 => VVBISL, Some(misc::instr_noimpl);
                0x2C9 => VSBISL, Some(misc::instr_noimpl);
                0x2CC => VVBICL, Some(misc::instr_noimpl);
                0x2CD => VSBICL, Some(misc::instr_noimpl);
                0x2E0 => VVSRLL, Some(misc::instr_noimpl);
                0x2E1 => VSSRLL, Some(misc::instr_noimpl);
                0x2E4 => VVSLLL, Some(misc::instr_noimpl);
                0x2E5 => VSSLLL, Some(misc::instr_noimpl);
                0x2E8 => VVXORL, Some(misc::instr_noimpl);
                0x2E9 => VSXORL, Some(misc::instr_noimpl);
                0x2EC => VVCVT, Some(misc::instr_noimpl);
                0x2ED => IOTA, Some(misc::instr_noimpl);
                0x2EE => VVMERGE, Some(misc::instr_noimpl);
                0x2EF => VSMERGE, Some(misc::instr_noimpl);
                0x2F6 => CVTHF, Some(misc::instr_noimpl);
                0x2F7 => CVTHD, Some(misc::instr_noimpl);
            }
        };
        
        self.itable = Some(itable);
    }
}

pub fn execute_instr<T: VAXBus>
    (
        instr_bytes: [u8; 2], 
        cpu: &mut VAXCPU<T>, 
        cycle_count: &mut Cycles
    )
    -> Result<(), Error>
{
    let pc = cpu.regfile.get_pc();
    let itable = cpu.itable.as_ref().expect("Itable missing!");
    let maybe_opfn = if instr_bytes[0] < 0xFC {
        cpu.regfile.set_pc(pc + 1);
        itable[instr_bytes[0] as usize]
    } else {
        let idx_adj = (instr_bytes[0] & 0x03) as usize * 256 + 256;
        cpu.regfile.set_pc(pc + 2);
        itable[instr_bytes[1] as usize + idx_adj]
    };

    if let Some(opfn) = maybe_opfn {
        opfn(cpu, cycle_count)
    } else {
        println!("{:?}", instr_bytes);
        Err(Error::new_reserved_instruction_fault())
    }
}

