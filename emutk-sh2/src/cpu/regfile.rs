

pub struct SH2RegisterFile {
    gpr: [u32;16],

    sr: u32,
    gbr: u32,
    vbr: u32,

    mac: u64,
    pr: u32,
    pc: u32,
}