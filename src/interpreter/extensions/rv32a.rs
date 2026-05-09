use crate::interpreter::{
    bus::Bus,
    riscv_core::{AtomicInstruction, Exception, ExceptionType, RVCore, Trap, WithErrVal},
    virtual_memory::sv32::{AccessType, translate_address},
};

pub fn lr_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::Load, 4)?;

    let val = bus
        .read_aligned_word(&phys_address)
        .with_err_val(address)
        .map_err(|exc| Exception::new(ExceptionType::LoadAccessFault, exc.get_val()))?;
    core.write_reg(instr.rd, val);
    bus.reserve_address(
        core.get_hartid(),
        phys_address.0 as usize,
        phys_address.0 as usize + 4,
    );

    Ok(())
}

fn sc_w_inner(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    let rs2_val = core.read_reg(instr.rs2);

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if bus.is_address_reserved(core.get_hartid(), phys_address.0 as usize, phys_address.0 as usize + 3) {
        bus.write_aligned_word(&phys_address, rs2_val)
            .with_err_val(address)
            .map_err(|exc| Exception::new(ExceptionType::StoreAmoAccessFault, exc.get_val()))?;
        core.write_reg(instr.rd, 0);
    } else {
        core.write_reg(instr.rd, 1);
    }

    Ok(())
}

pub fn sc_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let res = sc_w_inner(instr, bus, core);
    bus.invalidate_reserved_address(core.get_hartid());

    res
}

pub fn amoswap_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, core.read_reg(instr.rs2))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoadd_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp.wrapping_add(core.read_reg(instr.rs2)))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoand_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp & core.read_reg(instr.rs2))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoor_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp | core.read_reg(instr.rs2))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoxor_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp ^ core.read_reg(instr.rs2))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomax_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(
        &phys_address,
        (tmp as i32).max(core.read_reg(instr.rs2) as i32) as u32,
    )
    .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomin_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(
        &phys_address,
        (tmp as i32).min(core.read_reg(instr.rs2) as i32) as u32,
    )
    .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomaxu_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp.max(core.read_reg(instr.rs2)))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amominu_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    if phys_address.0 % 4 != 0 {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    if !bus.check_pma(&phys_address, AccessType::StoreAmo) {
        return Err(Exception::new(ExceptionType::StoreAmoAccessFault, address));
    }

    let tmp = bus.read_word(&phys_address).with_err_val(address)?;

    bus.write_aligned_word(&phys_address, tmp.min(core.read_reg(instr.rs2)))
        .with_err_val(address)?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}
