#[macro_export]
macro_rules! if_ {
    ($cond:expr => $x:expr) => {
        if $cond {
            $x
        }
    };
}
#[macro_use]
extern crate thiserror;
pub use anyhow::Result;
pub mod cpu;
pub mod util;
#[derive(Error, Debug)]
enum EmulatorError {
    #[error("No ROM loaded")]
    NoROM,
    #[error("No font loaded")]
    NoFont,
    #[error("Invalid opcode: {opcode} at program counter {pc}")]
    InvalidOpcode { pc: usize, opcode: u16 },
    #[error("Invalid register {0}")]
    InvalidRegister(u8),
}
