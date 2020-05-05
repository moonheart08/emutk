const BOOTLOADER: &[u8] = include_bytes!("../../emutk-vax/vsrc/bootrom/bootloader.bin");

use emutk_vax::cpu::VAXCPU;
use emutk_vax::bus::RAMBus;

fn simple_test_cpu_with_data(dat: &[u8]) -> (VAXCPU<RAMBus>, RAMBus) {
    let cpu = VAXCPU::new();
    let mut bus = RAMBus::new(dat.len()+1024);
    let buf = bus.ram_mut();
    buf[0..dat.len()].copy_from_slice(dat);
    (cpu, bus)
}

fn main() {
    println!("Executing test VAX program...\n");

    let (mut cpu, mut bus) = simple_test_cpu_with_data(BOOTLOADER);
    cpu.give_bus(&mut bus);

    while !cpu.halted() {
        match cpu.run_tick() {
            Ok(()) => {},
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    println!("");
}
