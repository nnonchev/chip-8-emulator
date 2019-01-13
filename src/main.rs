use std::io;
use std::io::prelude::*;
use std::fs::File;

const RAM_SIZE: usize = 4096;

fn main() {
    // let cpu = &mut Cpu::new();
    // let program = vec![0xaa; 20];
    // cpu.load_program(program);


    let fname = String::from("./roms/games/Nim [Carmelo Cortez, 1978].ch8");
    open_program(fname);
}

// TODO Finish method
fn open_program(fname: String)-> io::Result<()> {
    println!("Opening file: {}...", fname);

    let mut f = File::open(fname)?;
    let mut buffer = [0; 10];
    
    f.read(&mut buffer);

    for byte in buffer.iter() {
        println!("Byte: 0x{:0x}", byte);
    }

    Ok(())
}

struct Cpu {
    ram: [u8; RAM_SIZE],
    opcode: u16,        // Stores the current operation code
    V: [u8; 16],        // 16 8-bit General purpose registers
    I: u16,             // 1 16-bit register used for memory addresses
    ST: u8,             // Sound timer
    DT: u8,             // Delay timer
    stack: [u16; 16],   // The stack, used for subroutine calls
    PC: u16,            // Program counter
    SP: u8              // Stacl pointer
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            ram: [0; RAM_SIZE],
            opcode: 0,
            V: [0; 16],
            I: 0x200,       // The program starts at address 0x200
            ST: 0,
            DT: 0,
            stack: [0; 16],
            PC: 0x200,      // The program starts at address 0x200 
            SP: 0
        }
    }
    
    fn load_program(&mut self, program: Vec<u8>){
        for (index, instr) in program.iter().enumerate() {
            self.ram[0x200 + index] = *instr;
        }

        for i in 0x200..(0x200+20) {
            println!("Instr: {:0x}", self.ram[i]);
        }
    }
}
