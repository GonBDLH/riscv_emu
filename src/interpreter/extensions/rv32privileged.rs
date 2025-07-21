use crate::interpreter::{mmu::Mmu, riscv_core::{Exception, IInstruction, PrivilegeLevel, RVCore}};


pub fn ecall(_: &IInstruction, _: &Mmu, core: &mut RVCore) -> Result<(), Exception> {
    match core.privilege_level {
        PrivilegeLevel::Machine => Err(Exception::EnviromentCallFromMMode),
        PrivilegeLevel::Supervisor => Err(Exception::EnviromentCallFromSMode),
        PrivilegeLevel::User => Err(Exception::EnviromentCallFromUMode),
    }
}

pub fn ebreak(_: &IInstruction, _: &Mmu, _: &mut RVCore) -> Result<(), Exception> {
    Err(Exception::Breakpoint)
}

pub fn mret(_: &IInstruction, _: &Mmu, core: &mut RVCore) -> Result<(), Exception> {
    let mstatus = core.control_and_status.get_mstatus_mut_ref(core.privilege_level)?;

    let mpp = mstatus.get_mpp();
    let mpie = mstatus.get_mpie();
    let mpp_y = PrivilegeLevel::new(mpp);

    mstatus.set_mie(mpie);
    core.privilege_level = mpp_y;
    mstatus.set_mpie(true);
    mstatus.set_mpp(0b00);

    if mpp_y != PrivilegeLevel::Machine {
        mstatus.set_mprv(false);
    }

    Ok(())
}

// TODO SRET
pub fn sret(_: &IInstruction, _: &Mmu, _: &mut RVCore) -> Result<(), Exception> {
    Ok(())
}