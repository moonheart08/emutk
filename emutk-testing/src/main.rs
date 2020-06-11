const BOOTLOADER: &'static [u8] = include_bytes!("../../emutk-vax/vsrc/bootrom/bootloader.bin");

pub mod mcbus;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use emutk_vax::cpu::VAXCPU;
fn main() {
    println!("Attempting to run bootrom!\n");
    let mut cpu = VAXCPU::new();
    let mut bus = mcbus::VirtVAXBus::new(BOOTLOADER, 8388608 );

    cpu.regfile.set_pc(0x1000_0000);

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
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    continue
                }
                let mut c = line.chars();
                match c.next().unwrap() {
                    'l' => {
                        c.next(); 
                        let idx_str: String = c.collect();
                        let idx = u32::from_str_radix(&idx_str, 16).unwrap();
                        match cpu.read_val::<u32>(idx) {
                            Ok(v) => {
                                println!("Value: {:01$x}", v, 8);
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                continue
                            },
                        }
                    },
                    _ => {
                        println!("Invalid command.");
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
