use crate::interpreter::csr::ControlAndStatus;
use crate::interpreter::csr::MCAUSE;
use crate::interpreter::csr::MEDELEG;
use crate::interpreter::csr::MEPC;
use crate::interpreter::csr::MHARTID;
use crate::interpreter::csr::MTVAL;
use crate::interpreter::csr::MTVEC;
use crate::interpreter::extensions::rv32a::*;
use crate::interpreter::extensions::rv32i::*;
use crate::interpreter::extensions::rv32m::*;
use crate::interpreter::extensions::rv32privileged::*;
use crate::interpreter::extensions::rv32zicrs::*;
use crate::interpreter::mmu::Mmu;

pub struct RVCore {
    // x0/zero -> Siempre 0
    registers: [u32; 31],
    pub pc: u32,

    // csrs: [u8; 4096],
    pub control_and_status: ControlAndStatus,

    // pub id: usize,
    pub privilege_level: PrivilegeLevel,
}

impl Default for RVCore {
    fn default() -> Self {
        Self {
            registers: [0u32; 31],
            pc: 0x80000000,
            control_and_status: ControlAndStatus::new(),

            privilege_level: PrivilegeLevel::Machine,
        }
    }
}

impl RVCore {
    pub fn decode(&mut self, instr: u32) -> Option<InstructionType> {
        let opcode = instr & 0x7F;

        match opcode {
            0b0000011 => self.try_decode_load_instr(instr).map(InstructionType::I),
            0b0010011 => self.try_decode_i_instr(instr).map(InstructionType::I),
            0b0010111 => self.try_decode_auipc_instr(instr).map(InstructionType::U),
            0b0100011 => self.try_decode_s_instr(instr).map(InstructionType::S),
            0b0101111 => self
                .try_decode_atomic_instr(instr)
                .map(InstructionType::Atomic),
            0b0110011 => self.try_decode_r_instr(instr).map(InstructionType::R),
            0b0110111 => self.try_decode_lui_instr(instr).map(InstructionType::U),
            0b1100011 => self.try_decode_b_instr(instr).map(InstructionType::B),
            0b1100111 => self.try_decode_jalr_instr(instr).map(InstructionType::I),
            0b1101111 => self.try_decode_j_instr(instr).map(InstructionType::J),
            0b1110011 => self.try_decode_system_instr(instr).map(InstructionType::I),
            0b0001111 => self.try_decode_fence_instr(instr).map(InstructionType::I),

            _ => None,
        }
    }

    pub fn read_reg(&self, reg: u32) -> u32 {
        if reg == 0 {
            0
        } else if reg < 32 {
            self.registers[reg as usize - 1]
        } else {
            unreachable!("MAL REGISTRO")
        }
    }

    pub fn write_reg(&mut self, reg: u32, val: u32) {
        if reg == 0 {
        } else if reg < 32 {
            self.registers[reg as usize - 1] = val;
        } else {
            unreachable!("MAL REGISTRO")
        }
    }

    pub fn read_hartid(&self) -> Result<usize, Exception> {
        self.control_and_status
            .read_csr(MHARTID, self.privilege_level)
            .map(|x| x as usize)
    }

    fn try_decode_r_instr(&mut self, instr: u32) -> Option<RInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let funct7 = (instr >> 25) & 0x7F;

        match (funct3, funct7) {
            // ADD
            (0x0, 0x00) => Some(RInstruction::new(rs1, rs2, rd, add)),
            // MUL
            (0x0, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mul)),
            // SUB
            (0x0, 0x20) => Some(RInstruction::new(rs1, rs2, rd, sub)),
            // MULH
            (0x1, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulh)),
            // MULSU
            (0x2, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulhsu)),
            // MULSU
            (0x3, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulhu)),
            // XOR
            (0x4, 0x00) => Some(RInstruction::new(rs1, rs2, rd, xor)),
            // DIV
            (0x4, 0x01) => Some(RInstruction::new(rs1, rs2, rd, div)),
            // DIVU
            (0x5, 0x01) => Some(RInstruction::new(rs1, rs2, rd, divu)),
            // OR
            (0x6, 0x00) => Some(RInstruction::new(rs1, rs2, rd, or)),
            // REM
            (0x6, 0x01) => Some(RInstruction::new(rs1, rs2, rd, rem)),
            // AND
            (0x7, 0x00) => Some(RInstruction::new(rs1, rs2, rd, and)),
            // REMU
            (0x7, 0x01) => Some(RInstruction::new(rs1, rs2, rd, remu)),
            // SLL
            (0x1, 0x00) => Some(RInstruction::new(rs1, rs2, rd, sll)),
            // SRL
            (0x5, 0x00) => Some(RInstruction::new(rs1, rs2, rd, srl)),
            // SRA
            (0x5, 0x20) => Some(RInstruction::new(rs1, rs2, rd, sra)),
            // SLT
            (0x2, 0x00) => Some(RInstruction::new(rs1, rs2, rd, slt)),
            // SLTU
            (0x3, 0x00) => Some(RInstruction::new(rs1, rs2, rd, sltu)),

            _ => None,
        }
    }

    fn try_decode_i_instr(&mut self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm_val = get_i_imm_val(instr);

        match funct3 {
            // ADDI
            0x0 => Some(IInstruction::new(rs1, imm_val, rd, addi)),
            // XORI
            0x4 => Some(IInstruction::new(rs1, imm_val, rd, xori)),
            // ORI
            0x6 => Some(IInstruction::new(rs1, imm_val, rd, ori)),
            // ANDI
            0x7 => Some(IInstruction::new(rs1, imm_val, rd, andi)),
            // SLLI
            0x1 => {
                if (imm_val >> 5) & 0x7F == 0x00 {
                    Some(IInstruction::new(rs1, imm_val & 0x1F, rd, slli))
                } else {
                    None
                }
            }
            0x5 => {
                if (imm_val >> 5) & 0x7F == 0x00 {
                    // SRLI
                    Some(IInstruction::new(rs1, imm_val & 0x1F, rd, srli))
                } else if (imm_val >> 5) & 0x7F == 0x20 {
                    // SRAI
                    Some(IInstruction::new(rs1, imm_val & 0x1F, rd, srai))
                } else {
                    None
                }
            }
            // SLTI
            0x2 => Some(IInstruction::new(rs1, imm_val, rd, slti)),
            // SLTIU
            0x3 => Some(IInstruction::new(rs1, imm_val, rd, sltui)),

            _ => None,
        }
    }

    fn try_decode_load_instr(&mut self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm = (instr >> 20) & 0xFFF;

        let imm_val = get_i_imm_val(imm);

        match funct3 {
            0x0 => Some(IInstruction::new(rs1, imm_val, rd, lb)),
            0x1 => Some(IInstruction::new(rs1, imm_val, rd, lh)),
            0x2 => Some(IInstruction::new(rs1, imm_val, rd, lw)),
            0x4 => Some(IInstruction::new(rs1, imm_val, rd, lbu)),
            0x5 => Some(IInstruction::new(rs1, imm_val, rd, lhu)),

            _ => None,
        }
    }

    fn try_decode_s_instr(&mut self, instr: u32) -> Option<SInstruction> {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;

        let imm_val = get_s_imm_val(instr);

        match funct3 {
            0x0 => Some(SInstruction::new(rs1, rs2, imm_val, sb)),
            0x1 => Some(SInstruction::new(rs1, rs2, imm_val, sh)),
            0x2 => Some(SInstruction::new(rs1, rs2, imm_val, sw)),

            _ => None,
        }
    }

    fn try_decode_b_instr(&mut self, instr: u32) -> Option<BInstruction> {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;

        let imm_val = get_b_imm_val(instr);

        match funct3 {
            0x0 => Some(BInstruction::new(rs1, rs2, imm_val, beq)),
            0x1 => Some(BInstruction::new(rs1, rs2, imm_val, bne)),
            0x4 => Some(BInstruction::new(rs1, rs2, imm_val, blt)),
            0x5 => Some(BInstruction::new(rs1, rs2, imm_val, bge)),
            0x6 => Some(BInstruction::new(rs1, rs2, imm_val, bltu)),
            0x7 => Some(BInstruction::new(rs1, rs2, imm_val, bgeu)),

            _ => None,
        }
    }

    fn try_decode_j_instr(&mut self, instr: u32) -> Option<JInstruction> {
        let rd = (instr >> 7) & 0x1F;

        let imm_val = get_j_imm_val(instr);

        Some(JInstruction::new(imm_val, rd, jal))
    }

    fn try_decode_jalr_instr(&mut self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm = (instr >> 20) & 0xFFF;

        let imm_val = get_i_imm_val(imm);

        if funct3 == 0x0 {
            Some(IInstruction::new(rs1, imm_val, rd, jarl))
        } else {
            None
        }
    }

    fn try_decode_lui_instr(&mut self, instr: u32) -> Option<UInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let imm_val = get_u_imm_val(instr);

        Some(UInstruction::new(imm_val, rd, lui))
    }

    fn try_decode_auipc_instr(&mut self, instr: u32) -> Option<UInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let imm_val = get_u_imm_val(instr);

        Some(UInstruction::new(imm_val, rd, auipc))
    }

    fn try_decode_atomic_instr(&mut self, instr: u32) -> Option<AtomicInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let constraint_bits = (instr >> 25) & 0b11;
        let funct5 = (instr >> 27) & 0x1F;

        match (funct5, funct3) {
            (0b00010, 0x2) => Some(AtomicInstruction::new(constraint_bits, rs1, rs2, rd, lr_w)),
            (0b00011, 0x2) => Some(AtomicInstruction::new(constraint_bits, rs1, rs2, rd, sc_w)),
            (0b00001, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoswap_w,
            )),
            (0b00000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoadd_w,
            )),
            (0b00100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoxor_w,
            )),
            (0b01100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoand_w,
            )),
            (0b01000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoor_w,
            )),
            (0b10000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomin_w,
            )),
            (0b10100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomax_w,
            )),
            (0b11000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amominu_w,
            )),
            (0b11100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomaxu_w,
            )),
            _ => None,
        }
    }

    fn try_decode_system_instr(&mut self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let csr = get_csr(instr);

        match funct3 {
            // 0x1 => todo!("CSRRW"),
            0x0 => match (csr, rs1, rd) {
                (0, 0, 0) => Some(IInstruction::new(rs1, csr, rd, ecall)),
                (1, 0, 0) => Some(IInstruction::new(rs1, csr, rd, ebreak)),

                // Tecnicamente son instrucciones R, pero asi me simplifica la vida
                (0x102, 0, 0) => Some(IInstruction::new(rs1, csr, rd, sret)),
                (0x302, 0, 0) => Some(IInstruction::new(rs1, csr, rd, mret)),
                _ => None,
            },
            0x1 => Some(IInstruction::new(rs1, csr, rd, csrrw)),
            0x2 => Some(IInstruction::new(rs1, csr, rd, csrrs)),
            0x3 => Some(IInstruction::new(rs1, csr, rd, csrrc)),
            0x5 => Some(IInstruction::new(rs1, csr, rd, csrrwi)),
            0x6 => Some(IInstruction::new(rs1, csr, rd, csrrsi)),
            0x7 => Some(IInstruction::new(rs1, csr, rd, csrrci)),

            _ => None,
        }
    }

    fn try_decode_fence_instr(
        &mut self,
        instr: u32,
    ) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm_val = get_i_imm_val(instr);

        if funct3 == 0 {
            Some(IInstruction::new(rs1, imm_val, rd, fence))
        } else {
            None
        }
    }
}

fn get_i_imm_val(instr: u32) -> u32 {
    let imm_31_11 = 0xFFFFF800 * (instr >> 31 & 1);
    let imm_10_0 = (instr >> 20) & 0x7FF;

    imm_31_11 | imm_10_0
}

fn get_s_imm_val(instr: u32) -> u32 {
    let imm_31_11 = 0xFFFFF800 * (instr >> 31 & 1);
    let imm_10_5 = (instr >> 25) & 0x3F;
    let imm_4_0 = (instr >> 7) & 0x1F;

    imm_31_11 | imm_10_5 | imm_4_0
}

fn get_b_imm_val(instr: u32) -> u32 {
    let imm_31_12 = 0xFFFFF000 * (instr >> 31);
    let imm_11 = (instr & (1 << 7)) << 4;
    let imm_10_5 = (instr & (0b0111111 << 25)) >> 20;
    let imm_4_1 = (instr & (0b1111 << 8)) >> 7;

    imm_31_12 | imm_11 | imm_10_5 | imm_4_1
}

fn get_j_imm_val(instr: u32) -> u32 {
    let imm_31_20 = 0xFFF00000 * (instr >> 31);
    let imm_19_12 = instr & (0b11111111 << 12);
    let imm_11 = (instr & (1 << 20)) >> 9;
    let imm_10_5 = (instr & (0b111111 << 25)) >> 20;
    let imm_4_1 = (instr & (0b1111 << 21)) >> 20;

    imm_31_20 | imm_19_12 | imm_11 | imm_10_5 | imm_4_1
}

fn get_u_imm_val(instr: u32) -> u32 {
    instr & 0xFFFFF000
}

fn get_csr(instr: u32) -> u32 {
    (instr & 0xFFF00000) >> 20
}
pub enum InstructionType {
    R(RInstruction),
    I(IInstruction),
    S(SInstruction),
    B(BInstruction),
    J(JInstruction),
    U(UInstruction),
    Atomic(AtomicInstruction),
}

impl InstructionType {
    pub fn execute(&mut self, mmu: &mut Mmu, core: &mut RVCore) -> Result<(), Exception> {
        match self {
            Self::R(instr) => instr.execute(core),
            Self::I(instr) => instr.execute(mmu, core),
            Self::S(instr) => instr.execute(mmu, core),
            Self::B(instr) => instr.execute(core),
            Self::J(instr) => instr.execute(core),
            Self::U(instr) => instr.execute(core),
            Self::Atomic(instr) => instr.execute(mmu, core),
        }
    }
}

pub struct RInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl RInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        rd: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            rd,
            function,
        }
    }

    fn execute(&mut self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct IInstruction {
    pub rs1: u32,
    pub imm: u32,
    pub rd: u32,

    function: fn(&IInstruction, &Mmu, &mut RVCore) -> Result<(), Exception>,
}

impl IInstruction {
    pub fn new(
        rs1: u32,
        imm: u32,
        rd: u32,
        function: fn(&IInstruction, &Mmu, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            imm,
            rd,
            function,
        }
    }

    fn execute(&mut self, mmu: &mut Mmu, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct SInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,

    function: fn(&Self, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
}

impl SInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        imm: u32,
        function: fn(&Self, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            imm,
            function,
        }
    }

    pub fn execute(&mut self, mmu: &mut Mmu, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct BInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl BInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        imm: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            imm,
            function,
        }
    }

    pub fn execute(&mut self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct JInstruction {
    pub imm: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl JInstruction {
    pub fn new(
        imm: u32,
        rd: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            imm,
            rd,
            function,
        }
    }

    pub fn execute(&mut self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct UInstruction {
    pub imm: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl UInstruction {
    pub fn new(
        imm: u32,
        rd: u32,

        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self { imm, rd, function }
    }

    pub fn execute(&mut self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(&self, core)
    }
}

pub struct AtomicInstruction {
    pub _constraint_bits: u32,
    pub rs2: u32,
    pub rs1: u32,
    pub rd: u32,
    // pub mmu: &'a mut Mmu,
    // pub core: &'a mut RVCore,

    // function: fn(u32, u32, u32, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
    function: fn(&Self, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
}

impl AtomicInstruction {
    pub fn new(
        constraint_bits: u32,
        rs1: u32,
        rs2: u32,
        rd: u32,
        // mmu: &'a mut Mmu,
        // core: &'a mut RVCore,
        // function: fn(u32, u32, u32, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
        function: fn(&Self, &mut Mmu, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            _constraint_bits: constraint_bits,
            rs2,
            rs1,
            rd,
            // mmu,
            // core,
            function,
        }
    }

    pub fn execute(&mut self, mmu: &mut Mmu, core: &mut RVCore) -> Result<(), Exception> {
        // (self.function)(self.rs1, self.rs2, self.rd, self.mmu, self.core)
        (self.function)(self, mmu, core)
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum PrivilegeLevel {
    User = 0x00,
    Supervisor = 0x01,
    #[default]
    Machine = 0x03,
}

impl PrivilegeLevel {
    pub fn new(val: u32) -> Self {
        match val {
            0x00 => Self::User,
            0x01 => Self::Supervisor,
            0x03 => Self::Machine,
            _ => Self::Machine, // TODO
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction(u32),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAmoAddressMisaligned,
    StoreAmoAccessFault,
    EnviromentCallFromUMode,
    EnviromentCallFromSMode,
    EnviromentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StoreAmoPageFault,
    DoubleTrap,
    SoftwareCheck,
    HardwareError,
}

impl Exception {
    fn get_cause(&self) -> u32 {
        match self {
            Self::InstructionAddressMisaligned => 0,
            Self::InstructionAccessFault => 1,
            Self::IllegalInstruction(_) => 2,
            Self::Breakpoint => 3,
            Self::LoadAddressMisaligned => 4,
            Self::LoadAccessFault => 5,
            Self::StoreAmoAddressMisaligned => 6,
            Self::StoreAmoAccessFault => 7,
            Self::EnviromentCallFromUMode => 8,
            Self::EnviromentCallFromSMode => 9,
            Self::EnviromentCallFromMMode => 11,
            Self::InstructionPageFault => 12,
            Self::LoadPageFault => 13,
            Self::StoreAmoPageFault => 15,
            Self::DoubleTrap => 16,
            Self::SoftwareCheck => 18,
            Self::HardwareError => 19,
        }
    }

    pub fn trap(&self, core: &mut RVCore) -> Trap {
        let prev_priv_level = core.privilege_level;
        let medeleg = core.control_and_status.read_csr_unchecked(MEDELEG);
        let cause = self.get_cause();

        // if prev_priv_level as u32 <= PrivilegeLevel::Supervisor as u32 && ((medeleg >> cause) & 1) > 0 {
        //     todo!()
        // } else {
        core.privilege_level = PrivilegeLevel::Machine;

        core.control_and_status.write_csr_unchecked(MEPC, core.pc);
        core.control_and_status.write_csr_unchecked(MCAUSE, cause);
        core.control_and_status.write_csr_unchecked(MTVAL, 0); // TODO

        let mstatus = core
            .control_and_status
            .get_mstatus_mut_ref(core.privilege_level)
            .unwrap();
        mstatus.set_mpp(prev_priv_level as u32);
        mstatus.set_mpie(mstatus.get_mie());
        mstatus.set_mie(false);

        let mtvec = core.control_and_status.read_csr_unchecked(MTVEC);
        let base = mtvec & 0xFFFFFFFC;
        if mtvec & 0b11 == 0 {
            core.pc = base;
        } else {
            core.pc = base + 4 * cause;
        }
        // }

        // TODO
        Trap::Requested
    }
}

pub enum Trap {
    Contained,
    Requested,
    Invisible,
    Fatal,
}
