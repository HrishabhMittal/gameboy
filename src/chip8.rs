use crate::rng::RNG;
use std::time::{SystemTime, UNIX_EPOCH};
struct Emulator {
    game_memory: [u8; 0xFFF],
    registers: [u8; 16],
    i: u16,
    pc: usize,
    sp: usize,
    rng: RNG,
    stack: [u16; 16],
    screen: [u8; 32 * 64],
}

impl Emulator {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Emulator {
            game_memory: [0; 0xFFF],
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            rng: RNG::new(nanos),
            screen: [0; 32 * 64],
        }
    }
    fn get_opcode(&mut self) -> u16 {
        let mut res = self.game_memory[self.pc] as u16;
        res <<= 8;
        res |= self.game_memory[self.pc + 1] as u16;
        self.pc += 2;
        res
    }
    fn next(&mut self) {
        let opcode = self.get_opcode();
        let n1 = opcode & 0xF000 >> 12;
        let n2 = opcode & 0x0F00 >> 8;
        let n3 = opcode & 0x00F0 >> 4;
        let n4 = opcode & 0x000F;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let x = n2 as usize;
        let y = n3 as usize;
        match (n1, n2, n3, n4) {
            (0, 0, 0xE, 0) => {
                self.screen.fill(0);
            }
            (0, 0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            }
            (1, _, _, _) => {
                self.pc = nnn as usize;
            }
            (2, _, _, _) => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
            }
            (3, _, _, _) => {
                if self.registers[x] == nn {
                    self.pc += 1;
                }
            }
            (4, _, _, _) => {
                if self.registers[x] != nn {
                    self.pc += 1;
                }
            }
            (5, _, _, 0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 1;
                }
            }
            (6, _, _, _) => {
                self.registers[x] = nn;
            }
            (7, _, _, _) => {
                self.registers[x] += nn;
            }
            (8, _, _, 0) => {
                self.registers[x] = self.registers[y];
            }
            (8, _, _, 1) => {
                self.registers[x] |= self.registers[y];
            }
            (8, _, _, 2) => {
                self.registers[x] &= self.registers[y];
            }
            (8, _, _, 3) => {
                self.registers[x] ^= self.registers[y];
            }
            (8, _, _, 4) => {
                self.registers[x] += self.registers[y];
                todo!("add vf updates in all instructions");
            }
            (8, _, _, 5) => {
                self.registers[x] -= self.registers[y];
            }
            (8, _, _, 6) => {
                self.registers[x] >>= self.registers[y];
            }
            (8, _, _, 7) => {
                self.registers[x] = self.registers[y] - self.registers[x];
            }
            (8, _, _, 8) => {
                self.registers[x] = self.registers[y] - self.registers[x];
            }
            (8, _, _, 0xE) => {
                self.registers[x] <<= self.registers[y];
            }
            (9, _, _, 0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 1;
                }
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = (nnn + self.registers[0] as u16) as usize;
            }
            (0xC, _, _, _) => {
                self.registers[x] = self.rng.range(0, 255) as u8 & nn;
            }
            _ => {}
        }
    }
}
