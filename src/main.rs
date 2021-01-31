#[macro_use]
use chippers8::{cpu::CPU, *};
use std::{fs::File, io::Read};
fn main() -> Result<()> {
    let mut cpu = {
        let mut rom = Vec::<u8>::new();
        File::open("./c8_test.c8")?.read_to_end(&mut rom)?;
        CPU::with_rom(&rom)
        //CPU::new()
    };
    cpu.run()?;
    Ok(())
}
