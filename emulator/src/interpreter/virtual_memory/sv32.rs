use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, ExceptionType, PrivilegeLevel, RVCore, WithErrVal},
};

use bitfield::bitfield;

pub const PAGESIZE: u32 = 2u32.pow(12);
pub const LEVELS: u32 = 2;
pub const PTESIZE: u32 = 4;

#[derive(PartialEq, Clone, Copy)]
pub enum AccessType {
    Load,
    StoreAmo,
    Execute,
}

impl AccessType {
    fn get_page_fault_exception(&self) -> ExceptionType {
        match self {
            AccessType::Load => ExceptionType::LoadPageFault,
            AccessType::StoreAmo => ExceptionType::StoreAmoPageFault,
            AccessType::Execute => ExceptionType::InstructionPageFault,
        }
    }

    pub fn get_access_fault_exception(&self) -> ExceptionType {
        match self {
            AccessType::Load => ExceptionType::LoadAccessFault,
            AccessType::StoreAmo => ExceptionType::StoreAmoAccessFault,
            AccessType::Execute => ExceptionType::InstructionAccessFault,
        }
    }

    pub fn get_address_missaligned_exception(&self) -> ExceptionType {
        match self {
            AccessType::Load => ExceptionType::LoadAddressMisaligned,
            AccessType::StoreAmo => ExceptionType::StoreAmoAddressMisaligned,
            AccessType::Execute => ExceptionType::InstructionAddressMisaligned,
        }
    }
}

bitfield! {
    pub struct VirtAddress(u32);
    pub get_vpn1, set_vpn1: 31, 22;
    pub get_vpn0, set_vpn0: 21, 12;
    pub get_page_offset, set_page_offset: 11, 0;
}

bitfield! {
    #[derive(Debug)]
    pub struct PhysicalAddress(u64);
    pub get_ppn1, set_ppn1: 33, 22;
    pub get_ppn0, set_ppn0: 21, 12;
    pub get_ppn, set_ppn: 33, 12;
    pub get_page_offset, set_page_offset: 11, 0;
    pub _, _: 63, 34; // UNUSED
}

impl PhysicalAddress {
    pub const fn wrapping_add(&self, val: u64) -> Self {
        PhysicalAddress(self.0.wrapping_add(val))
    }
}

bitfield! {
    pub struct PageTableEntry(u32);
    pub get_ppn1, set_ppn1: 31, 20;
    pub get_ppn0, set_ppn0: 19, 10;
    pub get_ppn, set_ppn: 31, 10;
    pub get_rsw, set_rsw: 9, 8;
    pub get_d, set_d: 7;
    pub get_a, set_a: 6;
    pub get_g, set_g: 5;
    pub get_u, set_u: 4;
    pub get_x, set_x: 3;
    pub get_w, set_w: 2;
    pub get_r, set_r: 1;
    pub get_v, set_v: 0;
}

fn check_access(
    core: &mut RVCore,
    pte: &PageTableEntry,
    access_type: &AccessType,
    effective_priv: PrivilegeLevel,
) -> bool {
    match effective_priv {
        PrivilegeLevel::User => {
            if !pte.get_u() {
                return false;
            }
        }
        PrivilegeLevel::Supervisor => {
            let sstatus = core.control_and_status.read_sstatus_unchecked();

            if pte.get_u() && !sstatus.get_sum() && *access_type != AccessType::Execute {
                return false;
            }
        }
        _ => {}
    }

    match access_type {
        AccessType::Execute => pte.get_x(),
        AccessType::Load => {
            let mstatus = core.control_and_status.read_mstatus_unchecked();
            let mxr = mstatus.get_mxr();

            if mxr {
                pte.get_r() || pte.get_x()
            } else {
                pte.get_r()
            }
        }
        AccessType::StoreAmo => pte.get_w(),
    }
}

pub fn translate_address(
    core: &mut RVCore,
    bus: &mut Bus,
    virt_address: u32,
    access_type: AccessType,
    access_length: u64,
) -> Result<PhysicalAddress, Exception> {
    if access_type != AccessType::Execute && virt_address % access_length as u32 != 0 {
        return Err(Exception::new(
            access_type.get_address_missaligned_exception(),
            virt_address,
        ));
    }

    let mstatus = core.control_and_status.read_mstatus_unchecked();

    if core.privilege_level == PrivilegeLevel::Machine {
        if mstatus.get_mprv()
            && (access_type == AccessType::Load || access_type == AccessType::StoreAmo)
        {
            let mpp = mstatus.get_mpp();
            let effective_priv = PrivilegeLevel::new(mpp);
            translate(
                core,
                bus,
                virt_address,
                access_type,
                effective_priv,
                access_length,
            )
        } else {
            core.check_pmp(
                PhysicalAddress(virt_address as u64),
                core.privilege_level,
                access_type,
                access_length,
            )
            .with_err_val(virt_address)
        }
    } else {
        translate(
            core,
            bus,
            virt_address,
            access_type,
            core.privilege_level,
            access_length,
        )
    }
}

fn translate(
    core: &mut RVCore,
    bus: &mut Bus,
    virt_address: u32,
    access_type: AccessType,
    effective_priv: PrivilegeLevel,
    access_length: u64,
) -> Result<PhysicalAddress, Exception> {
    let satp = core.control_and_status.read_satp_unchecked();

    if effective_priv == PrivilegeLevel::Machine {
        return Ok(PhysicalAddress(virt_address as u64));
    }

    if effective_priv != PrivilegeLevel::Machine && !satp.get_mode() {
        return Ok(PhysicalAddress(virt_address as u64));
    }

    let va = VirtAddress(virt_address);

    let mut a = satp.get_ppn() * PAGESIZE;
    let mut i = LEVELS as i32 - 1;

    while i >= 0 {
        let pte_addr = if i == 1 {
            a + va.get_vpn1() * PTESIZE
        } else {
            a + va.get_vpn0() * PTESIZE
        };

        let pte_phys_address = core
            .check_pmp(
                PhysicalAddress(pte_addr as u64),
                PrivilegeLevel::Supervisor,
                access_type,
                4,
            )
            .with_err_val(virt_address)?;
        if !bus.check_pma(&pte_phys_address, access_type) {
            return Err(Exception::new(
                access_type.get_page_fault_exception(),
                virt_address,
            ));
        }

        let pte = PageTableEntry(
            bus.read_aligned_word(&pte_phys_address)
                .with_err_val(virt_address)?,
        );

        if !pte.get_v() || (!pte.get_r() && pte.get_w()) {
            return Err(Exception::new(
                access_type.get_page_fault_exception(),
                virt_address,
            ));
        }

        if pte.get_r() || pte.get_x() {
            if i > 0 && (pte.get_ppn0() != 0) {
                return Err(Exception::new(
                    access_type.get_page_fault_exception(),
                    virt_address,
                ));
            }

            // TODO PASO 7
            // Determine if the requested memory access is allowed by the pte.r, pte.w, and pte.x bits, given the
            // Shadow Stack Memory Protection rules. If not, stop and raise an access-fault exception

            if !check_access(core, &pte, &access_type, effective_priv) {
                return Err(Exception::new(
                    access_type.get_page_fault_exception(),
                    virt_address,
                ));
            }

            if !pte.get_a() || (access_type == AccessType::StoreAmo && !pte.get_d()) {
                let new_pte_phys_address = core
                    .check_pmp(
                        PhysicalAddress(pte_addr as u64),
                        PrivilegeLevel::Supervisor,
                        access_type,
                        4,
                    )
                    .with_err_val(virt_address)?;
                if !bus.check_pma(&new_pte_phys_address, access_type) {
                    return Err(Exception::new(
                        access_type.get_access_fault_exception(),
                        virt_address,
                    ));
                }

                let mut new_pte = PageTableEntry(
                    bus.read_aligned_word(&new_pte_phys_address)
                        .with_err_val(virt_address)?,
                );

                if new_pte.0 == pte.0 {
                    new_pte.set_a(true);
                    if access_type == AccessType::StoreAmo {
                        new_pte.set_d(true);
                    }

                    bus.write_aligned_word(&PhysicalAddress(pte_addr as u64), new_pte.0)
                        .unwrap();
                } else {
                    continue;
                }
            }

            let mut phys_address = PhysicalAddress(0);
            phys_address.set_page_offset(va.get_page_offset() as u64);
            if i > 0 {
                phys_address.set_ppn0(va.get_vpn0() as u64);
                phys_address.set_ppn1(pte.get_ppn1() as u64);
            } else {
                phys_address.set_ppn(pte.get_ppn() as u64);
            }

            return core
                .check_pmp(phys_address, effective_priv, access_type, access_length)
                .with_err_val(virt_address);
        }

        i -= 1;
        a = pte.get_ppn() * PAGESIZE;
    }

    Err(Exception::new(
        access_type.get_page_fault_exception(),
        virt_address,
    ))
}
