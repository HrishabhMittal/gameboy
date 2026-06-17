use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug, Default, Clone, Copy)]
struct Register {
    reg: u16,
}

impl Register {
    fn new() -> Self {
        Register { reg: 0 }
    }
    fn init(val: u16) -> Self {
        Register { reg: val }
    }
    fn get(&self) -> u16 {
        return self.reg;
    }
    fn set(&mut self, value: u16) {
        self.reg = value;
    }
    fn lo(&self) -> u8 {
        (self.reg & 0x00FF) as u8
    }
    fn set_lo(&mut self, value: u8) {
        self.reg = (self.reg & 0xFF00) | (value as u16);
    }
    fn hi(&self) -> u8 {
        (self.reg >> 8) as u8
    }
    fn set_hi(&mut self, value: u8) {
        self.reg = (self.reg & 0x00FF) | ((value as u16) << 8);
    }
}

const AF: usize = 0;
const BC: usize = 1;
const DE: usize = 2;
const HL: usize = 3;

const FLAG_Z: usize = 7;
const FLAG_N: usize = 6;
const FLAG_H: usize = 5;
const FLAG_C: usize = 4;

struct Emulator {
    mem: Vec<u8>,
    screen: [[[u8; 3]; 144]; 160],
    registers: [Register; 4],
    stack: Register,
    pc: u16,
}

impl Emulator {
    fn new() -> Self {
        let mut memory = vec![0; 0x10000];
        memory[0xFF05] = 0x00;
        memory[0xFF06] = 0x00;
        memory[0xFF07] = 0x00;
        memory[0xFF10] = 0x80;
        memory[0xFF11] = 0xBF;
        memory[0xFF12] = 0xF3;
        memory[0xFF14] = 0xBF;
        memory[0xFF16] = 0x3F;
        memory[0xFF17] = 0x00;
        memory[0xFF19] = 0xBF;
        memory[0xFF1A] = 0x7F;
        memory[0xFF1B] = 0xFF;
        memory[0xFF1C] = 0x9F;
        memory[0xFF1E] = 0xBF;
        memory[0xFF20] = 0xFF;
        memory[0xFF21] = 0x00;
        memory[0xFF22] = 0x00;
        memory[0xFF23] = 0xBF;
        memory[0xFF24] = 0x77;
        memory[0xFF25] = 0xF3;
        memory[0xFF26] = 0xF1;
        memory[0xFF40] = 0x91;
        memory[0xFF42] = 0x00;
        memory[0xFF43] = 0x00;
        memory[0xFF45] = 0x00;
        memory[0xFF47] = 0xFC;
        memory[0xFF48] = 0xFF;
        memory[0xFF49] = 0xFF;
        memory[0xFF4A] = 0x00;
        memory[0xFF4B] = 0x00;
        memory[0xFFFF] = 0x00;
        let mut regs = [Register::new(); 4];
        regs[AF] = Register::init(0x01B0);
        regs[BC] = Register::init(0x0013);
        regs[DE] = Register::init(0x00D8);
        regs[HL] = Register::init(0x014D);
        Emulator {
            mem: memory,
            screen: [[[0; 3]; 144]; 160],
            registers: regs,
            pc: 0x0100,
            stack: Register::init(0xFFFE),
        }
    }
    fn read_memory(&mut self, address: u16) -> u8 {
        todo!()
    }
    fn write_memory(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x7FFF => {}

            0xE000..=0xFDFF => {
                self.mem[address as usize] = byte;
                self.write_memory(address - 0x2000, byte);
            }

            0xFEA0..=0xFEFF => {}

            _ => {
                self.mem[address as usize] = byte;
            }
        }
        todo!()
    }
    fn next_op() -> i32 {
        todo!()
    }
}

pub fn emulate(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut emulator = Emulator::new();
    let mut buffer = Vec::new();
    let size = file.read_to_end(&mut buffer)?;

    emulator.mem[..size].copy_from_slice(&buffer[..size]);

    Ok(())
}
