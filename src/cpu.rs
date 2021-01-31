use crate::{util::*, EmulatorError, Result};

#[derive(Debug, Default)]
pub struct CPU {
    registers: Registers,
    memory: Memory,
    pc: usize,
    sp: usize,
    i: u16,
    timers: Timers,
    display: Display,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            pc: 0,
            sp: 0,
            i: 0,
            timers: Timers::new(),
            display: Display::new(),
        }
    }
    pub fn with_rom(rom: &[u8]) -> Self {
        let mut cpu = Self::new();
        cpu.load_rom(rom);
        cpu
    }
    pub fn load_rom(&mut self, rom: &[u8]) {
        if_! { self.loaded() => self.reset() }
        self.memory.0[512..(512 + rom.len())].copy_from_slice(rom);
        self.pc = 512;
    }
    pub fn run(&mut self) -> Result<()> {
        if self.loaded() {
            loop {
                if let Some(_interrupt) = self.execute()? {
                    break;
                }
            }
            Ok(())
        } else {
            Err(EmulatorError::NoROM.into())
        }
    }
    fn reset(&mut self) {
        self.registers.reset();
        self.memory.clear();
        self.pc = 0;
        self.sp = 0;
        self.i = 0;
        self.timers.reset();
        self.display.clear();
    }
    pub fn loaded(&self) -> bool {
        self.rom_loaded() && self.font_loaded()
    }
    fn rom_loaded(&self) -> bool {
        !self.memory.0[512..].iter().all(|&i| i == 0)
    }
    fn font_loaded(&self) -> bool {
        !self.memory.0[..512].iter().all(|&i| i == 0)
    }
    fn execute(&mut self) -> Result<Option<u16>> {
        use std::convert::TryInto;
        let opcode = u8_2_to_nibbles(
            self.memory.0[self.pc..self.pc + 2]
                .try_into()
                .expect("Invalid program counter"),
        );
        let mut interrupt = None;
        #[allow(unused_variables)]
        match opcode {
            [0x0, 0x0, 0xE, 0x0] => {
                self.display.clear();
            } // cls
            [0x0, 0x0, 0xE, 0xE] => (), // return
            [0x0, n, nn, nnn] => {
                let n = nibbles_to_u16(n, nn, nnn);
                interrupt = Some(n);
            } // interrupt
            [0x1, n, nn, nnn] => {
                let n = nibbles_to_u16(n, nn, nnn);
            } // jump
            [0x2, n, nn, nnn] => {
                let n = nibbles_to_u16(n, nn, nnn);
            } // call
            [0x3, x, n, nn] => {
                nibbles_to_u8(n, nn);
            } // skip x == nn
            [0x4, x, n, nn] => {
                nibbles_to_u8(n, nn);
            } // skip x != nn
            [0x5, x, y, 0x0] => {
                if self.registers.get(x)? == self.registers.get(x)? {
                    self.pc += 2
                }
            } // skip x == y
            [0x6, x, n, nn] => {
                nibbles_to_u8(n, nn);
            } // x = nn
            [0x7, x, n, nn] => {
                nibbles_to_u8(n, nn);
            } // x += nn
            [0x8, x, y, 0x0] => {
                let y = self.registers.get(y)?;
                self.registers.set(x, y)?;
            } // x =  y
            [0x8, x, y, 0x1] => {
                self.registers
                    .set(x, self.registers.get(x)? | self.registers.get(y)?)?;
            } // x =| y
            [0x8, x, y, 0x2] => {
                self.registers
                    .set(x, self.registers.get(x)? & self.registers.get(y)?)?;
            } // x =& y
            [0x8, x, y, 0x3] => (),     // x =^ y
            [0x8, x, y, 0x4] => (),     // x += y
            [0x8, x, y, 0x5] => (),     // x -= y
            [0x8, x, y, 0x6] => (),     // x>>1
            [0x8, x, y, 0x7] => (),     // x = x-y
            [0x8, x, y, 0xE] => (),     // x<<1
            [0x9, x, y, 0x0] => (),     // skip x!=y
            [0xA, n, nn, nnn] => {
                let n = nibbles_to_u16(n, nn, nnn);
            } // set I = nnn
            [0xB, n, nn, nnn] => {
                let n = nibbles_to_u16(n, nn, nnn);
            } // jump nnn+V0
            [0xC, x, nn, nnn] => (),    // x = rand()&nn
            [0xD, x, y, n] => (),       // draw sprite at (x, y)
            [0xE, x, 0x9, 0xE] => (),   // skip if x is pressed
            [0xE, x, 0xA, 0x1] => (),   // skip if x  isn't pressed
            [0xF, x, 0x0, 0x7] => (),   // get delay
            [0xF, x, 0x0, 0xA] => (),   // wait for keypress
            [0xF, x, 0x1, 0x5] => (),   // set delay
            [0xF, x, 0x1, 0x8] => (),   // set sound
            [0xF, x, 0x1, 0xE] => (),   // I += x
            [0xF, x, 0x2, 0x9] => (),   // I = char *x
            [0xF, x, 0x3, 0x3] => (),   // store bcd x
            [0xF, x, 0x5, 0x5] => (),   // store Vall at &I
            [0xF, x, 0x6, 0x5] => (),   // load vall from I
            [_, _, _, _] => {
                return Err(EmulatorError::InvalidOpcode {
                    pc: self.pc,
                    opcode: u16::from_be_bytes(self.memory.0[self.pc..self.pc + 2].try_into()?),
                }
                .into())
            } // invalid opcode
        };
        self.pc += 2;
        Ok(interrupt)
    }
}

#[derive(Debug, Default)]
struct Registers([u8; 16]);
impl Registers {
    const fn new() -> Self {
        Self([0; 16])
    }
    fn reset(&mut self) {
        for x in self.0.iter_mut() {
            *x = 0
        }
    }
    fn get(&self, x: u8) -> Result<u8> {
        if Self::index_valid(x) {
            Ok(self.0[x as usize])
        } else {
            Err(EmulatorError::InvalidRegister(x).into())
        }
    }
    fn set(&mut self, x: u8, v: u8) -> Result<()> {
        if Self::index_valid(x) {
            self.0[x as usize] = v;
            Ok(())
        } else {
            Err(EmulatorError::InvalidRegister(x).into())
        }
    }
    fn index_valid(x: u8) -> bool {
        matches!(x, 0..=0xF)
    }
}

#[derive(Debug, Default)]
struct Timers {
    delay: u8,
    sound: u8,
}
impl Timers {
    const fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }
    fn reset(&mut self) {
        self.delay = 0;
        self.sound = 0;
    }
}

#[derive(Debug)]
struct Display([bool; 64 * 32]);
impl Display {
    const fn new() -> Self {
        Self([false; 64 * 32])
    }
    fn clear(&mut self) {
        for x in self.0.iter_mut() {
            *x = false
        }
    }
}
impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Memory([u8; 4096]);
impl Memory {
    const fn new() -> Self {
        Self([0; 4096])
    }
    fn clear(&mut self) {
        for x in self.0.iter_mut() {
            *x = 0
        }
    }
}
impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
