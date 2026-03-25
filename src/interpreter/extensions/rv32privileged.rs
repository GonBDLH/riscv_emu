use crate::interpreter::{
    bus::Bus, csr::{MEPC, MSTATUS, SEPC, SSTATUS}, riscv_core::{Exception, ExceptionType, IInstruction, PrivilegeLevel, RVCore}
};

pub fn ecall(_: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    match core.privilege_level {
        PrivilegeLevel::Machine => Err(Exception::new(ExceptionType::EnviromentCallFromMMode, 0)),
        PrivilegeLevel::Supervisor => Err(Exception::new(ExceptionType::EnviromentCallFromSMode, 0)),
        PrivilegeLevel::User => Err(Exception::new(ExceptionType::EnviromentCallFromUMode, 0)),
    }
}

pub fn ebreak(_: &IInstruction, _: &mut Bus, _: &mut RVCore) -> Result<(), Exception> {
    Err(Exception::new(ExceptionType::Breakpoint, 0))
}

pub fn mret(_: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    core.pc = core.control_and_status.read_csr(MEPC, core.privilege_level)?.wrapping_sub(4);

    let mut mstatus = core
        .control_and_status
        .read_mstatus(core.privilege_level)?;

    let mpp = mstatus.get_mpp();
    let mpie = mstatus.get_mpie();
    let mpp_y = PrivilegeLevel::new(mpp);

    mstatus.set_mie(mpie);
    mstatus.set_mpie(true);
    mstatus.set_mpp(0b00);

    if mpp_y != PrivilegeLevel::Machine {
        mstatus.set_mprv(false);
    }
    core.control_and_status.write_csr(MSTATUS, core.privilege_level, mstatus.0)?;

    core.privilege_level = mpp_y;

    Ok(())
}

// TODO SRET
pub fn sret(instr: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    core.pc = core.control_and_status.read_csr(SEPC, core.privilege_level)?.wrapping_sub(4);

    let mut sstatus = core.control_and_status.read_sstatus(core.privilege_level)?;
    let mut mstatus = core
        .control_and_status
        .read_mstatus_unchecked();

    if mstatus.get_tsr() && core.privilege_level == PrivilegeLevel::Supervisor {
        return Err(Exception::new(ExceptionType::IllegalInstruction, instr.data));
    }

    let spp = sstatus.get_spp();
    let spie = sstatus.get_spie();
    // TODO
    let spp_y = PrivilegeLevel::new(spp as u32);

    sstatus.set_sie(spie);
    sstatus.set_spie(true);
    sstatus.set_spp(false);
    
    if spp_y != PrivilegeLevel::Machine {
        mstatus.set_mprv(false);
    }
    core.control_and_status.write_csr(SSTATUS, core.privilege_level, sstatus.0)?;
    // Puede que un poco hacky
    core.control_and_status.write_csr(MSTATUS, PrivilegeLevel::Machine, mstatus.0).unwrap();

    core.privilege_level = spp_y;

    Ok(())
}

pub fn sfence_vma(instr: &IInstruction, _: &mut Bus, core: &mut RVCore)  -> Result<(), Exception> { 
    let mstatus = core.control_and_status.read_mstatus(core.privilege_level).expect("Puede que esto no vaya asi");

    if mstatus.get_tvm() {
        return Err(Exception::new(ExceptionType::IllegalInstruction, instr.data));
    }

    Ok(())
}