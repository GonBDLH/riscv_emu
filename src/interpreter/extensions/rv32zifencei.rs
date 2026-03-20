use crate::interpreter::{bus::Bus, riscv_core::{Exception, IInstruction, RVCore}};

pub fn fence_i(_: &IInstruction, _: &mut Bus, _: &mut RVCore) -> Result<(), Exception> {
    // TODO
    Ok(())
}