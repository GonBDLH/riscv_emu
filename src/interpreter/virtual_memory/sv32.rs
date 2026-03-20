use crate::interpreter::{
    bus::Bus, csr::MStatus, riscv_core::{Exception, PrivilegeLevel, RVCore}
};

use bitfield::bitfield;

const PAGESIZE: u32 = 2u32.pow(12);
const LEVELS: u32 = 2;
const PTESIZE: u32 = 4;

#[derive(PartialEq)]
pub enum AccessType {
    Load,
    StoreAmo,
    Execute
}

impl AccessType {
    fn get_page_fault_exception(&self) -> Exception {
        match self {
            AccessType::Load => Exception::LoadPageFault,
            AccessType::StoreAmo => Exception::StoreAmoPageFault,
            AccessType::Execute => Exception::InstructionPageFault
        }
    }

    fn get_access_fault_exception(&self) -> Exception {
        match self {
            AccessType::Load => Exception::LoadAccessFault,
            AccessType::StoreAmo => Exception::StoreAmoAccessFault,
            AccessType::Execute => Exception::InstructionAccessFault
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
    pub struct PhysicalAddress(u64);
    pub get_ppn1, set_ppn1: 33, 22;
    pub get_ppn0, set_ppn0: 21, 12;
    pub get_ppn, set_ppn: 33, 12;
    pub get_page_offset, set_page_offset: 11, 0;
    pub _, _: 63, 30; // UNUSED
}

impl PhysicalAddress {
    pub const fn wrapping_add(&self, val: u64) -> Self {
        PhysicalAddress(self.0.wrapping_add(val))
    }
}

bitfield! {
    pub struct PageTableEntry(u32);
    pub get_ppn0, set_ppn0: 31, 20;
    pub get_ppn1, set_ppn1: 19, 10;
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

fn check_access(pte: &PageTableEntry, access_type: &AccessType, mstatus: &MStatus) -> bool {
    if !mstatus.get_sum() && pte.get_u() {
        false;
    }

    match access_type {
        AccessType::Execute => {
            pte.get_x()
        },
        AccessType::Load => {
            mstatus.get_mxr() && pte.get_x() || pte.get_r()
        },
        AccessType::StoreAmo => {
            pte.get_w()
        }
    }
}

pub fn translate_address(
    core: &mut RVCore,
    bus: &mut Bus,
    virt_address: u32,
    access_type: AccessType
) -> Result<PhysicalAddress, Exception> {
    let satp = core.control_and_status.get_satp_ref_unchecked();
    let mstatus = core.control_and_status.get_mstatus_ref_unchecked();
        
    if !satp.get_mode() {
        return Ok(PhysicalAddress(virt_address as u64));
    }
    
    if core.privilege_level == PrivilegeLevel::Machine {
        if mstatus.get_mprv() {
            if access_type == AccessType::Execute {
                return Ok(PhysicalAddress(virt_address as u64));
            }
        } else {
            return Ok(PhysicalAddress(virt_address as u64));
        }
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

        let pte = PageTableEntry(bus.read_word(&PhysicalAddress(pte_addr as u64))?);

        if !pte.get_v() || (!pte.get_r() && pte.get_w()) {
            return Err(access_type.get_page_fault_exception());
        }

        if pte.get_r() || pte.get_x() {
            if i > 0 && (pte.get_ppn0() != 0) {
                return Err(access_type.get_page_fault_exception());
            }

            // TODO PASO 7
            // Determine if the requested memory access is allowed by the pte.r, pte.w, and pte.x bits, given the
            // Shadow Stack Memory Protection rules. If not, stop and raise an access-fault exception

            if !check_access(&pte, &access_type, &mstatus) {
                return Err(access_type.get_page_fault_exception());
            }

            if pte.get_a() || (access_type == AccessType::StoreAmo && pte.get_d()) {
                let mut new_pte = PageTableEntry(bus.read_word(&PhysicalAddress(pte_addr as u64))?);

                if new_pte.0 == pte.0 {
                    new_pte.set_a(true);
                    if access_type == AccessType::StoreAmo {
                        new_pte.set_d(true);
                    }

                    let _ = bus.write_aligned_word(&PhysicalAddress(pte_addr as u64), new_pte.0);
                } else {
                    continue;
                }

                let _ = bus.write_aligned_word(&PhysicalAddress(pte_addr as u64), pte.0);
            }

            let mut phys_addres = PhysicalAddress(0);
            phys_addres.set_page_offset(va.get_page_offset() as u64);
            if i > 0 {
                phys_addres.set_ppn0(va.get_vpn0() as u64);
                phys_addres.set_ppn1(pte.get_ppn1() as u64);
            } else {
                phys_addres.set_ppn(pte.get_ppn() as u64);
            }
        }

        i -= 1;
        a = pte.get_ppn() * PAGESIZE;
    }

    Err(access_type.get_access_fault_exception())
}
