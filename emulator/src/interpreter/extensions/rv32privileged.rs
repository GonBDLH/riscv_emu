use crate::interpreter::{
    bus::Bus,
    csr::ControlAndStatus,
    riscv_core::{
        Exception, ExceptionType, IInstruction,
        PrivilegeLevel::{self, Machine},
        RVCore, WithErrVal,
    },
};

pub fn ecall(_: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    match core.privilege_level {
        PrivilegeLevel::Machine => Err(Exception::new(ExceptionType::EnviromentCallFromMMode, 0)),
        PrivilegeLevel::Supervisor => {
            Err(Exception::new(ExceptionType::EnviromentCallFromSMode, 0))
        }
        PrivilegeLevel::User => Err(Exception::new(ExceptionType::EnviromentCallFromUMode, 0)),
    }
}

pub fn ebreak(_: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    Err(Exception::new(ExceptionType::Breakpoint, core.get_pc()))
}

pub fn mret(instr: &IInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let mepc = core
        .control_and_status
        .read_csr(bus, ControlAndStatus::MEPC, core.privilege_level)
        .with_err_val(instr.data)?;
    let mut mstatus = core
        .control_and_status
        .read_mstatus(bus, core.privilege_level)
        .with_err_val(instr.data)?;

    let mpp = mstatus.get_mpp();
    let mpie = mstatus.get_mpie();
    let mpp_y = PrivilegeLevel::new(mpp);

    if mepc == 0x90809000 || mepc == 0x7fc0296c {
        println!(
            "MRET: mepc={:#010x} mpp={} new_priv={:?} mstatus={:#010x}",
            mepc,
            mstatus.get_mpp(),
            mpp_y,
            mstatus.0
        );
    }

    mstatus.set_mie(mpie);
    mstatus.set_mpie(true);
    mstatus.set_mpp(0b00);

    if mpp_y != PrivilegeLevel::Machine {
        mstatus.set_mprv(false);
    }
    core.control_and_status
        .write_csr(ControlAndStatus::MSTATUS, core.privilege_level, mstatus.0)
        .with_err_val(instr.data)?;

    core.privilege_level = mpp_y;
    core.set_pc(mepc);

    Ok(())
}

pub fn sret(instr: &IInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let mut mstatus = core.control_and_status.read_mstatus_unchecked();

    if mstatus.get_tsr() && core.privilege_level == PrivilegeLevel::Supervisor {
        return Err(Exception::new(
            ExceptionType::IllegalInstruction,
            instr.data,
        ));
    }

    let sepc = core
        .control_and_status
        .read_csr(bus, ControlAndStatus::SEPC, core.privilege_level)
        .with_err_val(instr.data)?;

    let spp = mstatus.get_spp();
    let spie = mstatus.get_spie();

    let spp_y = PrivilegeLevel::new(spp as u32);

    mstatus.set_sie(spie);
    mstatus.set_spie(true);
    mstatus.set_spp(false);

    if spp_y != PrivilegeLevel::Machine {
        mstatus.set_mprv(false);
    }
    core.control_and_status
        .write_csr(
            ControlAndStatus::MSTATUS,
            PrivilegeLevel::Machine,
            mstatus.0,
        )
        .with_err_val(core.get_pc())?;

    core.privilege_level = spp_y;
    core.set_pc(sepc);

    Ok(())
}

pub fn sfence_vma(instr: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let mstatus = core.control_and_status.read_mstatus_unchecked();

    if mstatus.get_tvm() {
        return Err(Exception::new(
            ExceptionType::IllegalInstruction,
            instr.data,
        ));
    }

    core.inc_pc(4);

    Ok(())
}

pub fn wfi(instr: &IInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let mstatus = core.control_and_status.read_mstatus_unchecked();
    if mstatus.get_tw() && core.privilege_level != PrivilegeLevel::Machine {
        return Err(Exception::new(
            ExceptionType::IllegalInstruction,
            instr.data,
        ));
    }

    core.inc_pc(4);

    // core.stalled = true;

    Ok(())
}
