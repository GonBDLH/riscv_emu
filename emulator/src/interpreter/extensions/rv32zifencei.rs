use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, IInstruction, RVCore},
};

pub fn fence_i(_: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    // TODO
    core.inc_pc(4);
    Ok(())
}
