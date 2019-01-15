use std::io;
use std::io::prelude::*;
use std::fs::File;

const RAM_SIZE: usize = 4096;

fn main() {
    let cpu = &mut Cpu::new();

    let fname = String::from("./roms/games/Nim [Carmelo Cortez, 1978].ch8");

    cpu.load_progran(fname);
    cpu.run();
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
    
    fn load_progran(&mut self, fname: String) -> io::Result<()> {
            let mut f = File::open(fname)?;
            let mut buffer = Vec::new();

            f.read_to_end(&mut buffer)?;

            for (index, instr) in buffer.iter().enumerate() {
                self.ram[0x200 + index] = *instr;
            }

            Ok(())
    }

    fn display_ram(& self) {
        let mut index = 0x200;
        while self.ram[index] != 0x0 {
            println!("0x{:0x} -> 0x{:0x}", index, self.ram[index]);
            index += 1;
        }
    }

    /* Seperate each instruction (16 bytes) to opcodes */
    fn run(& self) {
        // println!("First instr 1: 0x{:0x}", self.ram[0x200]);
        // println!("First instr 2: 0x{:0x}", self.ram[0x200+1]);
        let opcode1 = (self.ram[0x200] & 0xf0) >> 4;
        let opcode2 = self.ram[0x200] & 0x0f;
        let opcode3 = (self.ram[0x200+1] & 0xf0) >> 4;
        let opcode4 = self.ram[0x200+1] & 0x0f;
        // println!("partial 1 instr: 0x{:0x}", opcode1);
        // println!("Partial 2 instr: 0x{:0x}", opcode2);
        // println!("partial 3 instr: 0x{:0x}", opcode3);
        // println!("Partial 4 instr: 0x{:0x}", opcode4);
    }
}
