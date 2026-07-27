use crate::{interpreter::{
    bus::Bus,
    pmp::PmpCsrs,
    riscv_core::{ExceptionType, PrivilegeLevel},
    virtual_memory::sv32::{AccessType, PhysicalAddress},
}, peripherals::RTC_BASE};
use bitfield::bitfield;

pub struct ControlAndStatus {
    csrs: [u32; 4096],
    pmp: PmpCsrs,
    // mstatus: MStatus,
    // satp: Satp32,
    minstret_loaded: bool,
    minstret: u64,
    cycle: u64,
}

impl ControlAndStatus {
    /*
     * MACHINE CRS
     */
    // INFORMATION
    const MVENDORID: usize = 0xF11;
    const MARCHID: usize = 0xF12;
    const MIMPID: usize = 0xF13;
    pub const MHARTID: usize = 0xF14;
    const MCONFIGPTR: usize = 0xF15;
    // TRAP SETUP
    pub const MSTATUS: usize = 0x300;
    pub const MISA: usize = 0x301;
    pub const MEDELEG: usize = 0x302;
    pub const MIDELEG: usize = 0x303;
    pub const MIE: usize = 0x304;
    pub const MTVEC: usize = 0x305;
    const MCOUNTEREN: usize = 0x306;
    pub const MSTATUSH: usize = 0x310;
    pub const MEDELEGH: usize = 0x312;
    // TRAP HANDLING
    const MSCRATCH: usize = 0x340;
    pub const MEPC: usize = 0x341;
    pub const MCAUSE: usize = 0x342;
    pub const MTVAL: usize = 0x343;
    pub const MIP: usize = 0x344;
    const MTINST: usize = 0x34A;
    const MTVAL2: usize = 0x34B;
    // COUNTER/TIMERS
    const MCYCLE: usize = 0xB00;
    const MINSTRET: usize = 0xB02;
    const MHPMCOUNTER3: usize = 0xB03; // MAX 31 (-3)
    const MHPMCOUNTER31: usize = 0xB1F;
    const MCYCLEH: usize = 0xB80;
    const MINSTRETH: usize = 0xB82;
    const MHPMCOUNTER3H: usize = 0xB83; // MAX 31 (-3)
    const MHPMCOUNTER31H: usize = 0xB9F; // MAX 31 (-3)

    const MENVCFG: usize = 0x30A;
    const MENVCFGH: usize = 0x31A;

    const MCOUNTINHIBIT: usize = 0x320;

    const PMPCFG0: usize = 0x3A0;
    const PMPCFG15: usize = 0x3AF;
    const PMPADDR0: usize = 0x3B0;
    const PMPADDR63: usize = 0x3EF;

    // DEBUG
    const TSELECT: usize = 0x7A0;

    // MACHINE MASKS
    const MSTATUS_MASK: u32 = 0x007E19AA;
    const MISA_MASK_WRITE: u32 = 0b00000000000101000001000100000001;
    const MIE_MASK: u32 = 0x00002AAA;
    const MSTATUSH_MASK: u32 = 0x6F0;
    const MIP_MASK: u32 = 0x00002AAA;
    const MENVCFG_MASK: u32 = 0x0001;
    const MENVCFGH_MASK: u32 = 0xA000;

    /*
     * SUPERVISOR
     */
    pub const SSTATUS: usize = 0x100;
    pub const SIE: usize = 0x104;
    pub const STVEC: usize = 0x105;
    const SCOUNTEREN: usize = 0x106;
    const SENVCFG: usize = 0x10A;
    const SSCRATCH: usize = 0x140;
    pub const SEPC: usize = 0x141;
    pub const SCAUSE: usize = 0x142;
    pub const STVAL: usize = 0x143;
    pub const SIP: usize = 0x144;

    pub const SATP: usize = 0x180;

    // SUPERVISOR MASKS
    const SSTATUS_MASK: u32 = 0x000C0122;
    const SIE_MASK: u32 = 0xFFFF2222;
    const SIP_MASK: u32 = 0xFFFF2222;
    const SENVCFG_MASK: u32 = 0x0001;

    // UNPRIVILEGED
    const CYCLE: usize = 0xC00;
    const TIME: usize = 0xC01;
    const INSTRET: usize = 0xC02;
    const HPMCOUNTER3: usize = 0xC03;
    const HPMCOUNTER31: usize = 0xC1F;
    const CYCLEH: usize = 0xC80;
    const TIMEH: usize = 0xC81;
    const INSTRETH: usize = 0xC82;
    const HPMCOUNTER3H: usize = 0xC83;
    const HPMCOUNTER31H: usize = 0xC9F;

    pub fn new(hart_id: u32) -> Self {
        let mut csrs = [0u32; 4096];

        let mut misa = 0u32;
        misa |= 0b01 << 30; // 32 bits
        misa |= 1 << 18; // Supervisor ISA
        misa |= 1 << 12; // RV31M
        misa |= 1 << 8; // RV32I
        misa |= 1 << 2; // RVC
        misa |= 1; // RV32A

        csrs[Self::MISA] = misa;

        csrs[Self::MHARTID] = hart_id;

        Self {
            csrs,
            pmp: PmpCsrs::default(),
            minstret_loaded: false,
            minstret: 0,
            cycle: 0
        }
    }

    pub fn read_csr(
        &self,
        bus: &Bus,
        csr: usize,
        priv_level: PrivilegeLevel,
    ) -> Result<u32, ExceptionType> {
        let csr_priv = (csr >> 8) & 0b11;

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(ExceptionType::IllegalInstruction);
        }

        // let val = self.csrs[csr as usize];
        let val = match csr {
            Self::MVENDORID => self.csrs[Self::MVENDORID],
            Self::MARCHID => self.csrs[Self::MARCHID],
            Self::MIMPID => self.csrs[Self::MIMPID],
            Self::MHARTID => self.csrs[Self::MHARTID],

            Self::MSTATUS => self.csrs[Self::MSTATUS] & Self::MSTATUS_MASK,
            Self::MISA => self.csrs[Self::MISA],
            Self::MEDELEG => self.csrs[Self::MEDELEG],
            Self::MIDELEG => self.csrs[Self::MIDELEG],
            Self::MIE => self.csrs[Self::MIE] & Self::MIE_MASK,
            Self::MTVEC => self.csrs[Self::MTVEC],
            Self::MCOUNTEREN => self.csrs[Self::MCOUNTEREN],
            Self::MSTATUSH => self.csrs[Self::MSTATUSH] & Self::MSTATUSH_MASK,
            Self::MEDELEGH => self.csrs[Self::MEDELEGH],

            Self::MSCRATCH => self.csrs[Self::MSCRATCH],
            Self::MEPC => self.csrs[Self::MEPC],
            Self::MCAUSE => self.csrs[Self::MCAUSE],
            Self::MTVAL => self.csrs[Self::MTVAL],
            Self::MIP => self.csrs[Self::MIP] & Self::MIP_MASK,
            Self::MTINST => self.csrs[Self::MTINST],
            Self::MTVAL2 => self.csrs[Self::MTVAL2],

            Self::MENVCFG => self.csrs[csr] & Self::MENVCFG_MASK,
            Self::MENVCFGH => self.csrs[csr] & Self::MENVCFGH_MASK,

            Self::MCOUNTINHIBIT => self.csrs[csr],

            Self::MCYCLE => self.csrs[csr],
            Self::MINSTRET => self.minstret as u32,
            Self::MHPMCOUNTER3..Self::MHPMCOUNTER31 => self.csrs[csr],
            Self::MCYCLEH => self.csrs[csr],
            Self::MINSTRETH => (self.minstret >> 32) as u32,
            Self::MHPMCOUNTER3H..Self::MHPMCOUNTER31H => self.csrs[csr],

            Self::PMPCFG0..=Self::PMPCFG15 => self.pmp.get_pmp_cfg(csr - Self::PMPCFG0),
            Self::PMPADDR0..=Self::PMPADDR63 => self.pmp.get_pmp_addr(csr - Self::PMPADDR0),

            Self::TSELECT => u32::MAX, // TODO Cambiar si se incluye el modo debug

            Self::SSTATUS => self.csrs[Self::MSTATUS] & Self::SSTATUS_MASK,
            Self::SIE => self.csrs[Self::MIE] & Self::SIE_MASK,
            Self::STVEC => self.csrs[Self::STVEC],
            Self::SCOUNTEREN => self.csrs[Self::SCOUNTEREN],
            Self::SENVCFG => self.csrs[Self::SENVCFG],

            Self::SSCRATCH => self.csrs[Self::SSCRATCH],
            Self::SEPC => self.csrs[Self::SEPC],
            Self::SCAUSE => self.csrs[Self::SCAUSE],
            Self::STVAL => self.csrs[Self::STVAL],
            Self::SIP => self.csrs[Self::MIP] & Self::SIP_MASK,

            Self::SATP => {
                let mstatus = self.read_mstatus_unchecked();

                if mstatus.get_tvm() {
                    return Err(ExceptionType::IllegalInstruction);
                }

                self.csrs[Self::SATP]
            }

            Self::CYCLE => self.cycle as u32,
            Self::TIME => bus.read_word(&PhysicalAddress(RTC_BASE as u64)).unwrap(),
            Self::INSTRET => self.minstret as u32,
            Self::HPMCOUNTER3..=Self::HPMCOUNTER31 => {
                let csr = (csr - Self::HPMCOUNTER3) + Self::MHPMCOUNTER3;
                self.csrs[csr]
            },
            Self::CYCLEH => (self.cycle >> 32) as u32,
            Self::TIMEH => bus.read_word(&PhysicalAddress(RTC_BASE as u64 + 4)).unwrap(),
            Self::INSTRETH => (self.minstret >> 32) as u32,
            Self::HPMCOUNTER3H..=Self::HPMCOUNTER31H => self.csrs[csr],

            _ => {
                println!("READ {:03X}", csr);
                return Err(ExceptionType::IllegalInstruction);
            }
        };

        Ok(val)
    }

    pub fn read_misa_unchecked(&self) -> u32 {
        self.csrs[Self::MISA]
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_mstatus_unchecked(&self) -> MStatus {
        MStatus(self.csrs[Self::MSTATUS] & Self::MSTATUS_MASK)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_sstatus_unchecked(&self) -> SStatus {
        SStatus(self.csrs[Self::MSTATUS] & Self::SSTATUS_MASK)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_satp_unchecked(&self) -> Satp32 {
        Satp32(self.csrs[Self::SATP])
    }

    pub fn read_mstatus(
        &self,
        bus: &Bus,
        priv_level: PrivilegeLevel,
    ) -> Result<MStatus, ExceptionType> {
        let csr = self.read_csr(bus, Self::MSTATUS, priv_level)?;

        Ok(MStatus(csr))
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_sstatus(
        &self,
        bus: &Bus,
        priv_level: PrivilegeLevel,
    ) -> Result<SStatus, ExceptionType> {
        let csr = self.read_csr(bus, Self::SSTATUS, priv_level)?;

        Ok(SStatus(csr))
    }

    pub fn read_mip_unchecked(&self) -> u32 {
        self.csrs[Self::MIP]
    }

    pub fn read_mie_unchecked(&self) -> u32 {
        self.csrs[Self::MIE]
    }

    pub fn read_mideleg_unchecked(&self) -> u32 {
        self.csrs[Self::MIDELEG]
    }

    pub fn read_sip_unchecked(&self) -> u32 {
        self.csrs[Self::SIP]
    }

    pub fn read_sie_unchecked(&self) -> u32 {
        self.csrs[Self::SIE]
    }

    pub fn read_medeleg_unchecked(&self) -> u32 {
        self.csrs[Self::MEDELEG]
    }

    pub fn read_medelegh_unchecked(&self) -> u32 {
        self.csrs[Self::MEDELEGH]
    }

    pub fn read_mtvec_unchecked(&self) -> u32 {
        self.csrs[Self::MTVEC]
    }

    pub fn read_stvec_unchecked(&self) -> u32 {
        self.csrs[Self::STVEC]
    }

    pub fn write_csr(
        &mut self,
        csr: usize,
        priv_level: PrivilegeLevel,
        val: u32,
    ) -> Result<(), ExceptionType> {
        let csr_rw = (csr >> 10) & 0b11;
        let csr_priv = (csr >> 8) & 0b11;

        if csr_rw == 0b11 {
            // READ-ONLY, RAISE EXCEPTION
            return Err(ExceptionType::IllegalInstruction);
        }

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(ExceptionType::IllegalInstruction);
        }

        match csr {
            Self::SSTATUS => {
                self.csrs[Self::MSTATUS] =
                    (self.csrs[Self::MSTATUS] & !Self::SSTATUS_MASK) | (val & Self::SSTATUS_MASK)
            }
            Self::STVEC => self.csrs[Self::STVEC] = val,
            Self::SCOUNTEREN => self.csrs[Self::SCOUNTEREN] = val,
            Self::SENVCFG => self.csrs[Self::SENVCFG] = val & Self::SENVCFG_MASK,

            Self::SSCRATCH => self.csrs[Self::SSCRATCH] = val,

            Self::MSTATUS => self.csrs[Self::MSTATUS] = Self::MSTATUS_MASK & val,
            Self::MEDELEG => self.csrs[Self::MEDELEG] = val,
            Self::MIDELEG => self.csrs[Self::MIDELEG] = val,
            Self::MIE => self.csrs[Self::MIE] = val & Self::MIE_MASK,
            Self::MTVEC => self.csrs[Self::MTVEC] = val,
            Self::MSTATUSH => self.csrs[Self::MSTATUSH] = val & Self::MSTATUSH_MASK,
            Self::MEDELEGH => self.csrs[Self::MEDELEGH] = val,

            Self::MIP => self.csrs[Self::MIP] = val & Self::MIP_MASK,

            // Self::MINSTRET | Self::MINSTRETH => {
            //     self.minstret_loaded = true;
            //     self.csrs[csr] = val;
            // }
            Self::MINSTRET => {
                self.minstret_loaded = true;
                self.minstret &= 0xFFFFFFFF00000000;
                self.minstret |= val as u64;
            }
            Self::MINSTRETH => {
                self.minstret_loaded = true;
                self.minstret &= 0xFFFFFFFF00000000;
                self.minstret |= (val as u64) << 32;
            }
            Self::MEPC => self.csrs[Self::MEPC] = val & 0xFFFFFFFE,
            Self::MISA => {
                self.csrs[Self::MISA] =
                    (self.csrs[Self::MISA] & !Self::MISA_MASK_WRITE) | (val & Self::MISA_MASK_WRITE)
            }
            Self::MCAUSE => self.csrs[Self::MCAUSE] = val,
            Self::MTVAL => self.csrs[Self::MTVAL] = val,
            Self::MSCRATCH => self.csrs[Self::MSCRATCH] = val,
            Self::MCOUNTEREN => self.csrs[Self::MCOUNTEREN] = val, // TODO HAY QUE CONTROLAR LOS CONTADORES

            Self::MENVCFG => self.csrs[csr] = val & Self::MENVCFG_MASK,
            Self::MENVCFGH => self.csrs[csr] = val & Self::MENVCFGH_MASK,

            Self::MCOUNTINHIBIT => self.csrs[csr] = val,

            Self::PMPCFG0..=Self::PMPCFG15 => self.pmp.set_pmp_cfg(csr - Self::PMPCFG0, val),
            Self::PMPADDR0..=Self::PMPADDR63 => self.pmp.set_pmp_addr(csr - Self::PMPADDR0, val),

            Self::SEPC => self.csrs[csr] = val,
            Self::SCAUSE => self.csrs[csr] = val,
            Self::STVAL => self.csrs[csr] = val,
            Self::SIP => self.csrs[csr] = val & Self::SIP_MASK,
            Self::SIE => self.csrs[csr] = val & Self::SIE_MASK,

            Self::SATP => {
                let mstatus = self.read_mstatus_unchecked();

                if mstatus.get_tvm() {
                    return Err(ExceptionType::IllegalInstruction);
                }

                self.csrs[Self::SATP] = val;
            }

            _ => {
                println!("WRITE {:03X}", csr);
                return Err(ExceptionType::IllegalInstruction);
            }
        }

        Ok(())
    }

    pub fn increment_minstret(&mut self) {
        if self.minstret_loaded {
            self.minstret_loaded = false;
            return;
        }

        self.minstret = self.minstret.wrapping_add(1);
    }

    pub fn set_mip_bit(&mut self, bit: u32) {
        self.csrs[Self::MIP] |= 1 << bit;
    }

    pub fn check_pmp(
        &self,
        phys_address: PhysicalAddress,
        priv_level: PrivilegeLevel,
        access_type: AccessType,
        access_length: u64,
    ) -> Result<PhysicalAddress, ExceptionType> {
        if self
            .pmp
            .has_access(priv_level, access_type, &phys_address, access_length)
        {
            Ok(phys_address)
        } else {
            Err(access_type.get_access_fault_exception())
        }
    }

    pub fn inc_cycle(&mut self) {
        self.cycle += 1;
    }
}

bitfield! {
    pub struct MStatus(u32);
    _, _: 0; // WPRI 0
    pub get_sie, set_sie: 1;
    _, _: 2; // WPRI 2
    pub get_mie, set_mie: 3;
    _, _: 4; // WPRI 4
    pub get_spie, set_spie: 5;
    pub get_ube, set_ube: 6;
    pub get_mpie, set_mpie: 7;
    pub get_spp, set_spp: 8;
    pub get_vs, set_vs: 10, 9;
    pub get_mpp, set_mpp: 12, 11;
    pub get_fs, set_fs: 14, 13;
    pub get_xs, set_xs: 16, 15;
    pub get_mprv, set_mprv: 17;
    pub get_sum, set_sum: 18;
    pub get_mxr, set_mxr: 19;
    pub get_tvm, set_tvm: 20;
    pub get_tw, set_tw: 21;
    pub get_tsr, set_tsr: 22;
    pub get_spelp, set_spelp: 23;
    pub get_sdt, set_sdt: 24;
    _, _: 30, 25; // WPRI 25-30
    pub get_sd, set_sd: 31;
}

bitfield! {
    pub struct Satp32(u32);
    u32;
    pub get_ppn, set_ppn: 21, 0;
    pub get_asid, set_asid: 30, 22;
    pub get_mode, set_mode: 31
}

bitfield! {
    pub struct SStatus(u32);
    _, _: 0; // WPRI 0
    pub get_sie, set_sie: 1;
    _, _: 4, 2; // WPRI 2
    pub get_spie, set_spie: 5;
    pub get_ube, set_ube: 6;
    _, _: 7; // WPRI 3
    pub get_spp, set_spp: 8;
    pub get_vs, set_vs: 10, 9;
    _, _: 12, 11; // WPRI 4
    pub get_fs, set_fs: 14, 13;
    pub get_xs, set_xs: 16, 15;
    _, _: 17;
    pub get_sum, set_sum: 18;
    pub get_mxr, set_mxr: 19;
    pub _, _: 22, 20;
    pub get_spelp, set_spelp: 23;
    pub get_sdt, set_sdt: 24;
    _, _: 30, 25; // WPRI 25-30
    pub get_sd, set_sd: 31;
}
