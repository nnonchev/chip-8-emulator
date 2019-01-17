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
        let mut i: usize = 0x200;
        loop {
            println!("0x{:0x}+1 -> 0x{:0x}{:0x}", i, self.ram[i], self.ram[i+1]);

//            let mut addr: u16 = (op2 as u16) << 0x08;
//            addr += (op3 as u16) << 0x04;
//            addr += op4 as u16;

//            let op1 = (self.ram[i] & 0xf0) >> 4;
//            let op2 = self.ram[i] & 0x0f;
//            let op3 = (self.ram[i+1] & 0xf0) >> 4;
//            let op4 = self.ram[i+1] & 0x0f;

            println!("op1: {:0x}, op2: {:0x}, op3: {:0x}, op4: {:0x}", op1, op2, op3, op4);

//            self._parse_instr(op1, op2, op3, op4);

            i += 2;

            if (i+1) >= RAM_SIZE { break; }
        }
    }

    fn _parse_instr(& self, op1: u8, op2: u8, op3: u8, op4: u8) {
        match op1 {
            0x0 => {
                let mut sub_op: u16 = (op2 as u16) << 0x08;
                sub_op += (op3 as u16) << 0x04;
                sub_op += op4 as u16;

                match sub_op {
                    0x0e0 => {
                        println!("CLS");
                    },
                    0x0ee => {
                        println!("RET");
                    },
                    _ => {
                        println!("SYS 0x{:0x}", sub_op);
                    }
                }
            },
            0x1 => {
                let mut addr: u16 = (op2 as u16) << 0x08;
                addr += (op3 as u16) << 0x04;
                addr += op4 as u16;

                println!("JP 0x{:0x}", addr);
            },
            0x2 => {
                let mut addr: u16 = (op2 as u16) << 0x08;
                addr += (op3 as u16) << 0x04;
                addr += op4 as u16;

                println!("CALL 0x{:0x}", addr);
            },
            0x3 => {
                let mut byte: u8 = op3 << 4;
                byte += op4;

                println!("SE V{:0x}, 0x{:0x}", op2, byte);
            },
            0x4 => {
                let mut byte: u8 = op3 << 4;
                byte += op4;

                println!("SNE V{:0x}, 0x{:0x}", op2, byte);
            },
            0x5 => {
                println!("SE V{:0x}, V{:0x}", op2, op3);
            },
            0x6 => {
                let mut byte: u8 = op3 << 4;
                byte += op4;

                println!("LD V{:0x}, 0x{:0x}", op2, byte);
            },
            0x7 => {
                let mut byte: u8 = op3 << 4;
                byte += op4;

                println!("ADD V{:0x}, 0x{:0x}", op2, byte);
            },
            0x8 => {
                match op4 {
                    0x0 => {
                        println!("LD V{:0x}, V{:0x}", op2, op3);
                    },
                    0x1 => {
                        println!("OR V{:0x}, V{:0x}", op2, op3);
                    },
                    0x2 => {
                        println!("AND V{:0x}, V{:0x}", op2, op3);
                    },
                    0x3 => {
                        println!("XOR V{:0x}, V{:0x}", op2, op3);
                    },
                    0x4 => {
                        println!("ADD V{:0x}, V{:0x}", op2, op3);
                    },
                    0x5 => {
                        println!("SUB V{:0x}, V{:0x}", op2, op3);
                    },
                    0x6 => {
                        println!("SHR V{:0x}, [V{:0x}]", op2, op3);
                    },
                    0x7 => {
                        println!("SUBN V{:0x}, V{:0x}", op2, op3);
                    },
                    0xe => {
                        println!("SHL V{:0x}, V{:0x}", op2, op3);
                    },
                    _ => println!("Error in sub instruction 8xy{}", op4),
                }
            },
            0x9 => {
                println!("SNE V{:0x}, V{:0x}", op2, op3);
            },
            0xa => {
                let mut addr: u16 = (op2 as u16) << 0x08;
                addr += (op3 as u16) << 0x04;
                addr += op4 as u16;

                println!("LD I, 0x{:0x}", addr);
            },
            0xb => {
                let mut addr: u16 = (op2 as u16) << 0x08;
                addr += (op3 as u16) << 0x04;
                addr += op4 as u16;

                println!("JP V0, 0x{:0x}", addr);
            },
            0xc => {
                let mut byte: u8 = op3 << 4;
                byte += op4;

                println!("RND V{:0x}, 0x{:0x}", op2, byte);
            },
            0xd => {
                println!("DRW V{:0x}, V{:0x}, 0x{:0x}", op2, op3, op4);
            },
            0xe => {
                let mut sub_op: u8 = op3 << 4;
                sub_op += op4;

                match sub_op {
                    0x9e => {
                        println!("SKP V{:0x}", op2);
                    },
                    0xa1 => {
                        println!("SKNP V{:0x}", op2);
                    },
                    _ => println!("Error in sub instruction ex{}", sub_op),
                }
            },
            0xf => {
                let mut sub_op: u8 = op3 << 4;
                sub_op += op4;

                match sub_op {
                    0x07 => {
                        println!("LD V{:0x}, DT", op2);
                    },
                    0x0a => {
                        println!("LD V{:0x}, K", op2);
                    },
                    0x15 => {
                        println!("LD DT, V{:0x}", op2);
                    },
                    0x18 => {
                        println!("LD ST, V{:0x}", op2);
                    },
                    0x1e => {
                        println!("ADD I, V{:0x}", op2);
                    },
                    0x29 => {
                        println!("LD F, V{:0x}", op2);
                    },
                    0x33 => {
                        println!("LD B, V{:0x}", op2);
                    },
                    0x55 => {
                        println!("LD [I], V{:0x}", op2);
                    },
                    0x65 => {
                        println!("LD V{:0x}, [I]", op2);
                    }
                    _ => println!("Error in sub instruction fx{}", sub_op),
                }
            },
            _ => println!("Error opcode not found: 0x{:0x}", op1)
        }
    }
}
