
const BOOTLOADER: &'static [u8] = include_bytes!("../../emutk-vax/vsrc/bootrom/bootloader.bin");

pub mod mcbus;

use emutk_vax::cpu::VAXCPU;
use emutk_vax::bus::{
    MicroVAX3100Bus,
    RAMSize,
};

fn main() {
    println!("Attempting to run bootrom!\n");
    let mut cpu = VAXCPU::new();
    let mut bus = MicroVAX3100Bus::new(BOOTLOADER, RAMSize::Size32MB);

    cpu.prepare_as_microvax();

    cpu.give_bus(&mut bus);
    let mut icount = 0;
    while !cpu.halted() {
        match cpu.run_tick() {
            Ok(()) => {
                icount+=1;
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    println!("Cycles: {} | ICount: {}", cpu.cur_cycle(), icount);
}
