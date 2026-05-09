use crate::interpreter::riscv_core::{Exception, ExceptionType};

pub fn semihosting(op_number: u32, parameter: u32) -> Result<(), Exception> {
    match op_number {
        0x18 => Err(Exception::new(ExceptionType::ExitException, parameter)),

        _ => Ok(()),
    }
}
