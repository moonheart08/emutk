use num_derive::*;
use num_traits::{ToPrimitive, FromPrimitive};

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum InstructionType {
    ADAWI = 0x58,

    ADDB2 = 0x80,
    ADDB3 = 0x81,
    ADDW2 = 0xA0,
    ADDW3 = 0xA1,
    ADDL2 = 0xC0,
    ADDL3 = 0xC1,
    #[cfg(feature = "64bit")]
    ADDQ2 = 0x80FD,
    #[cfg(feature = "64bit")]
    ADDQ3 = 0x81FD,
    #[cfg(feature = "128bit")]
    ADDO2 = 0xA0FD,
    #[cfg(feature = "128bit")]
    ADDO3 = 0xA1FD,

    ADWC = 0xD8,

    ASHL = 0x78,
    ASHQ = 0x79,

    BICB2 = 0x8A,
    BICB3 = 0x8B,
    BICW2 = 0xAA,
    BICW3 = 0xAB,
    BICL2 = 0xCA,
    BICL3 = 0xCB,
    #[cfg(feature = "64bit")]
    BICQ2 = 0x8AFD,
    #[cfg(feature = "64bit")]
    BICQ3 = 0x8BFD,
    #[cfg(feature = "128bit")]
    BICO2 = 0xAAFD,
    #[cfg(feature = "128bit")]
    BICO3 = 0xABFD,

    BISB2 = 0x88,
    BISB3 = 0x89,
    BISW2 = 0xA8,
    BISW3 = 0xA9,
    BISL2 = 0xC8,
    BISL3 = 0xC9,
    #[cfg(feature = "64bit")]
    BISQ2 = 0x88FD,
    #[cfg(feature = "64bit")]
    BISQ3 = 0x89FD,
    #[cfg(feature = "128bit")]
    BISO2 = 0xA8FD,
    #[cfg(feature = "128bit")]
    BISO3 = 0xA9FD,

    BITB = 0x93,
    BITW = 0xB3,
    BITL = 0xD3,
    #[cfg(feature = "64bit")]
    BITQ = 0x93FD,
    #[cfg(feature = "128bit")]
    BITO = 0xB3FD,

    CLRB = 0x94,
    CLRW = 0xB4,
    CLRL = 0xD4,
    CLRQ = 0x7C,
    CLRO = 0x7CFD,

    CMPB = 0x91,
    CMPW = 0xB1,
    CMPL = 0xD1,
    #[cfg(feature = "64bit")]
    CMPQ = 0x91FD,
    #[cfg(feature = "128bit")]
    CMPO = 0xB1FD,

    CVTBW = 0x99,
    CVTBL = 0x98,
    CVTWB = 0x33,
    CVTWL = 0x32,
    CVTLB = 0xF6,
    CVTLW = 0xF7,
    //TODO: allocate Quad and Octa conversion instructions.

    DECB = 0x97,
    DECW = 0xB7,
    DECL = 0xD7,
    #[cfg(feature = "64bit")]
    DECQ = 0x97FD,
    #[cfg(feature = "128bit")]
    DECO = 0xB7FD,

    DIVB2 = 0x86,
    DIVB3 = 0x87,
    DIVW2 = 0xA6,
    DIVW3 = 0xA7,
    DIVL2 = 0xC6,
    DIVL3 = 0xC7,
    #[cfg(feature = "64bit")]
    DIVQ2 = 0x86FD,
    #[cfg(feature = "64bit")]
    DIVQ3 = 0x87FD,
    #[cfg(feature = "128bit")]
    DIVO2 = 0xA6FD,
    #[cfg(feature = "128bit")]
    DIVO3 = 0xA7FD,


    EDIV = 0x7B,
    EMUL = 0x7A,

    INCB = 0x96,
    INCW = 0xB6,
    INCL = 0xD6,
    #[cfg(feature = "64bit")]
    INCQ = 0x96FD,
    #[cfg(feature = "128bit")]
    INCO = 0xB6FD,


    MCOMB = 0x92,
    MCOMW = 0xB2,
    MCOML = 0xD2,
    #[cfg(feature = "64bit")]
    MCOMQ = 0x92FD,
    #[cfg(feature = "128bit")]
    MCOMO = 0xB2FD,

    MNEGB = 0x8E,
    MNEGW = 0xAE,
    MNEGL = 0xCE,
    #[cfg(feature = "64bit")]
    MNEGQ = 0x8EFD,
    #[cfg(feature = "128bit")]
    MNEGO = 0xAEFD,

    MOVB = 0x90,
    MOVW = 0xB0,
    MOVL = 0xD0,
    MOVQ = 0x7D,
    MOVO = 0x7DFD,

    MOVZBW = 0x9B,
    MOVZBL = 0x9A,
    MOVZWL = 0x3C,
    //TODO: allocate Qud and Octa zero extend instructions.

    MULB2 = 0x84,
    MULB3 = 0x85,
    MULW2 = 0xA4,
    MULW3 = 0xA5,
    MULL2 = 0xC4,
    MULL3 = 0xC5,
    #[cfg(feature = "64bit")]
    MULQ2 = 0x84FD,
    #[cfg(feature = "64bit")]
    MULQ3 = 0x85FD,
    #[cfg(feature = "128bit")]
    MULO2 = 0xA4FD,
    #[cfg(feature = "128bit")]
    MULO3 = 0xA5FD,

    PUSHL = 0xDD,
    #[cfg(feature = "64bit")]
    PUSHQ = 0xDDFD,

    ROTL = 0x9C,
    #[cfg(feature = "64bit")]
    ROTQ = 0x9CFD,
    
    SBWC = 0xD9,

    SUBB2 = 0x82,
    SUBB3 = 0x83,
    SUBW2 = 0xA2,
    SUBW3 = 0xA3,
    SUBL2 = 0xC2,
    SUBL3 = 0xC3,
    #[cfg(feature = "64bit")]
    SUBQ2 = 0x82FD,
    #[cfg(feature = "64bit")]
    SUBQ3 = 0x83FD,
    #[cfg(feature = "128bit")]
    SUBO2 = 0xA2FD,
    #[cfg(feature = "128bit")]
    SUBO3 = 0xA3FD,

    TSTB = 0x95,
    TSTW = 0xB5,
    TSTL = 0xD5,
    #[cfg(feature = "64bit")]
    TSTQ = 0x95FD,
    #[cfg(feature = "128bit")]
    TSTO = 0xB5FD,
    
    XORB2 = 0x8C,
    XORB3 = 0x8D,
    XORW2 = 0xAC,
    XORW3 = 0xAD,
    XORL2 = 0xCC,
    XORL3 = 0xCD,
    #[cfg(feature = "64bit")]
    XORQ2 = 0x8CFD,
    #[cfg(feature = "64bit")]
    XORQ3 = 0x8DFD,
    #[cfg(feature = "128bit")]
    XORO2 = 0xACFD,
    #[cfg(feature = "128bit")]
    XORO3 = 0xADFD,

    MOVAB = 0x9E,
    MOVAW = 0x3E,
    MOVAL = 0xDE,
    MOVAQ = 0x7E,
    MOVAO = 0x7EFD,

    PUSHAB = 0x9F,
    PUSHAW = 0x3F,
    PUSHAL = 0xDF,
    PUSHAQ = 0x7F,
    PUSHAO = 0x7FFD,

    CMPV = 0xEC,
    CMPZV = 0xED,

    EXTV = 0xEE,
    EXTZV = 0xEF,

    FFC = 0xEB,
    FFS = 0xEA,

    INSV = 0xF0,

    ACBB = 0x9D,
    ACBW = 0x3D,
    ACBL = 0xF1,
    ACBF = 0x4F,
    ACBD = 0x6F, // Unsupported instruction, D_floating
    ACBG = 0x4FFD,
    ACBH = 0x6FFD, // Unsupported instruction, H_floating

    AOBLEQ = 0xF3,

    AOBLSS = 0xF2,

    BGTR = 0x14,
    BLEQ = 0x15,
    BNEQ = 0x12,
    BEQL = 0x13,
    BGEQ = 0x18,
    BLSS = 0x19,
    BGTRU = 0x1A,
    BLEQU = 0x1B,
    BVC = 0x1C,
    BVS = 0x1D,
    BGEQU = 0x1E,
    BLSSU = 0x1F,

    BBS = 0xE0,
    BBC = 0xE1,

    BBSS = 0xE2,
    BBCS = 0xE3,
    BBSC = 0xE4,
    BBCC = 0xE5,

    BBSSI = 0xE6,
    BBCCI = 0xE7,

    BLBS = 0xE8,
    BLBC = 0xE9,

    BRB = 0x11,
    BRW = 0x31,

    BSBB = 0x10,
    BSBW = 0x30,

    CASEB = 0x8F,
    CASEW = 0xAF,
    CASEL = 0xCF,

    JMP = 0x17,

    JSB = 0x16,

    RSB = 0x05,

    SOBGEQ = 0xF4,

    SOBGTR = 0xF5,

    CALLG = 0xFA,
    CALLS = 0xFB,
    RET = 0x04,

    BICPSW = 0xB9,
    BISPSW = 0xB8,

    BPT = 0x03,
    BUGW = 0xFEFF,
    BUGL = 0xFDFF,

    HALT = 0x00,

    INDEX = 0x0A,

    MOVPSL = 0xDC,

    NOP = 0x01,

    POPR = 0xBA,
    PUSHR = 0xBB,

    // Queue instructions

    INSQHI = 0x5C,
    INSQTI = 0x5D,
    INSQUE = 0x0E, // Why are the queue instructions in the major 252 instructions? 
    REMQHI = 0x5E, // They'd be better put in one of the 3 extension prefixes...
    REMQTI = 0x5F,
    REMQUE = 0x0F,

    // Floating point

    ADDF2 = 0x40,
    ADDF3 = 0x41,
    ADDD2 = 0x60, // Unsupported
    ADDD3 = 0x61, // Unsupported
    ADDG2 = 0x40FD,
    ADDG3 = 0x41FD,
    ADDH2 = 0x60FD, // Unsupported
    ADDH3 = 0x61FD, // Unsupported

    CMPF = 0x51,
    CMPD = 0x71, // Unsupported
    CMPG = 0x51FD,
    CMPH = 0x71FD, // Unsupported

    // Floating point convert instructions.

    CVTBF = 0x4C,
    CVTWF = 0x4D,
    CVTLF = 0x4E,

    CVTBD = 0x6C, // Unsupported
    CVTWD = 0x6D, // Unsupported
    CVTLD = 0x6E, // Unsupported

    CVTBG = 0x4CFD,
    CVTWG = 0x4DFD,
    CVTLG = 0x4EFD,

    CVTBH = 0x6CFD, // Unsupported
    CVTWH = 0x6DFD, // Unsupported
    CVTLH = 0x6EFD, // Unsupported

    CVTFB = 0x48,
    CVTFW = 0x49,
    CVTFL = 0x4A,
    CVTRFL = 0x4B,

    CVTDB = 0x68, // Unsupported
    CVTDW = 0x69, // Unsupported
    CVTDL = 0x6A, // Unsupported
    CVTRDL = 0x6B, // Unsupported

    CVTGB = 0x48FD,
    CVTGW = 0x49FD,
    CVTGL = 0x4AFD,
    CVTRGL = 0x4BFD,

    CVTHB = 0x68FD, // Unsupported
    CVTHW = 0x69FD, // Unsupported
    CVTHL = 0x6AFD, // Unsupported
    CVTRHL = 0x6BFD, // Unsupported

    CVTFD = 0x56, // Unsupported
    CVTFG = 0x99FD,
    CVTFH = 0x98FD, // Unsupported

    CVTDF = 0x76, // Unsupported
    CVTDH = 0x32FD, // Unsupported

    CVTGF = 0x33FD,
    CVTGH = 0x56FD, // Unsupported

    CVTHF = 0xF6FD, // Unsupported
    CVTHD = 0xF7FD, // Unsupported
    CVTHG = 0x76FD, // Unsupported

    // Rest of floating point.

    DIVF2 = 0x46,
    DIVF3 = 0x47,
    DIVD2 = 0x66, // Unsupported
    DIVD3 = 0x67, // Unsupported
    DIVG2 = 0x46FD,
    DIVG3 = 0x47FD,
    DIVH2 = 0x66FD, // Unsupported
    DIVH3 = 0x67FD, // Unsupported

    EMODF = 0x54,
    EMODD = 0x74, // Unsupported
    EMODG = 0x54FD,
    EMODH = 0x74FD, // Unsupported

    MNEGF = 0x52,
    MNEGD = 0x72, // Unsupported
    MNEGG = 0x52FD,
    MNEGH = 0x72FD, // Unsupported

    MOVF = 0x50,
    MOVD = 0x70, // Unsupported
    MOVG = 0x50FD,
    MOVH = 0x70FD, // Unsupported

    MULF2 = 0x44,
    MULF3 = 0x45,
    MULD2 = 0x64, // Unsupported
    MULD3 = 0x65, // Unsupported
    MULG2 = 0x44FD,
    MULG3 = 0x45FD,
    MULH2 = 0x64FD, // Unsupported
    MULH3 = 0x65FD, // Unsupported

    POLYF = 0x55, // All POLY instructions are unimplemented for now. Not top priority.
    POLYD = 0x75, // Unsupported
    POLYG = 0x55FD,
    POLYH = 0x75FD, // Unsupported

    SUBF2 = 0x42,
    SUBF3 = 0x43,
    SUBD2 = 0x62, // Unsupported
    SUBD3 = 0x63, // Unsupported
    SUBG2 = 0x42FD,
    SUBG3 = 0x43FD,
    SUBH2 = 0x62FD, // Unsupported
    SUBH3 = 0x63FD, // Unsupported

    TSTF = 0x53,
    TSTD = 0x73,
    TSTG = 0x53FD,
    TSTH = 0x73FD,
    
    // Instructions for exceptions and interrupts.
    
    REI = 0x02,

    CHMK = 0xBC,
    CHME = 0xBD,
    CHMS = 0xBE,
    CHMU = 0xBF,

    // Instructions for processes.

    LDPCTX = 0x06,
    SVPCTX = 0x07,

    //

    MTPR = 0xDA,
    MFPR = 0xDB,
}

impl InstructionType {
    pub fn from_instrid(bytes: [u8; 2]) -> Option<Self> 
    {
        match bytes[0] {
            0xFD | 0xFE | 0xFF => {
                InstructionType::from_u16(u16::from_le_bytes(bytes))
            }
            v => InstructionType::from_u8(v),
        }
    }

    pub fn opcode_len(self) -> usize {
        let dat = self.to_u16().unwrap();
        if  dat > 0xFF {
            2
        } else {
            1
        }
    }

}