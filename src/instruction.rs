use crate::cpu;

#[derive(Debug, PartialEq)]
pub enum FlagValue {
    NO_CHANGE,
    CHANGE,
    FORCE_TRUE,
    FORCE_FALSE,
}

pub struct Flags {
    pub zero: FlagValue,
    pub subtract: FlagValue,
    pub half_carry: FlagValue,
    pub carry: FlagValue,
}

#[derive(Debug, PartialEq)]
pub enum DEC_Arg_0 {
    B,
    BC,
    C,
    D,
    DE,
    E,
    H,
    HL,
    L,
    Indirect_HL,
    SP,
    A,
}

impl DEC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            DEC_Arg_0::B => cpu.registers.b as u16,
            DEC_Arg_0::BC => cpu.registers.get_bc(),
            DEC_Arg_0::C => cpu.registers.c as u16,
            DEC_Arg_0::D => cpu.registers.d as u16,
            DEC_Arg_0::DE => cpu.registers.get_de(),
            DEC_Arg_0::E => cpu.registers.e as u16,
            DEC_Arg_0::H => cpu.registers.h as u16,
            DEC_Arg_0::HL => cpu.registers.get_hl(),
            DEC_Arg_0::L => cpu.registers.l as u16,
            DEC_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            DEC_Arg_0::SP => cpu.sp,
            DEC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            DEC_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::BC => {
                cpu.registers.set_bc(value);
                value
            }
            DEC_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::DE => {
                cpu.registers.set_de(value);
                value
            }
            DEC_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            DEC_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            DEC_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            DEC_Arg_0::SP => {
                cpu.sp = value;
                value
            }
            DEC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JP_Arg_0 {
    NZ,
    a16,
    Z,
    NC,
    C,
    HL,
}

impl JP_Arg_0 {
    pub fn condition(&self, cpu: &cpu::CPU) -> bool {
        match *self {
            JP_Arg_0::NZ => !cpu.registers.f.zero,
            JP_Arg_0::a16 => true,
            JP_Arg_0::Z => cpu.registers.f.zero,
            JP_Arg_0::NC => !cpu.registers.f.carry,
            JP_Arg_0::C => cpu.registers.f.carry,
            JP_Arg_0::HL => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JP_Arg_1 {
    NONE,
    a16,
}

impl JP_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            JP_Arg_1::NONE => panic!("can not call!"),
            JP_Arg_1::a16 => cpu.read_next_word(),
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            JP_Arg_1::NONE => panic!("can not call!"),
            JP_Arg_1::a16 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SBC_Arg_0 {
    A,
}

impl SBC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SBC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SBC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SBC_Arg_1 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl SBC_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SBC_Arg_1::B => cpu.registers.b as u16,
            SBC_Arg_1::C => cpu.registers.c as u16,
            SBC_Arg_1::D => cpu.registers.d as u16,
            SBC_Arg_1::E => cpu.registers.e as u16,
            SBC_Arg_1::H => cpu.registers.h as u16,
            SBC_Arg_1::L => cpu.registers.l as u16,
            SBC_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SBC_Arg_1::A => cpu.registers.a as u16,
            SBC_Arg_1::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SBC_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SBC_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            SBC_Arg_1::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SWAP_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl SWAP_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SWAP_Arg_0::B => cpu.registers.b as u16,
            SWAP_Arg_0::C => cpu.registers.c as u16,
            SWAP_Arg_0::D => cpu.registers.d as u16,
            SWAP_Arg_0::E => cpu.registers.e as u16,
            SWAP_Arg_0::H => cpu.registers.h as u16,
            SWAP_Arg_0::L => cpu.registers.l as u16,
            SWAP_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SWAP_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SWAP_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SWAP_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SWAP_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SUB_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl SUB_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SUB_Arg_0::B => cpu.registers.b as u16,
            SUB_Arg_0::C => cpu.registers.c as u16,
            SUB_Arg_0::D => cpu.registers.d as u16,
            SUB_Arg_0::E => cpu.registers.e as u16,
            SUB_Arg_0::H => cpu.registers.h as u16,
            SUB_Arg_0::L => cpu.registers.l as u16,
            SUB_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SUB_Arg_0::A => cpu.registers.a as u16,
            SUB_Arg_0::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SUB_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SUB_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            SUB_Arg_0::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CALL_Arg_0 {
    NZ,
    Z,
    a16,
    NC,
    C,
}

impl CALL_Arg_0 {
    pub fn condition(&self, cpu: &cpu::CPU) -> bool {
        match *self {
            CALL_Arg_0::NZ => !cpu.registers.f.zero,
            CALL_Arg_0::Z => cpu.registers.f.zero,
            CALL_Arg_0::a16 => true,
            CALL_Arg_0::NC => !cpu.registers.f.carry,
            CALL_Arg_0::C => cpu.registers.f.carry,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CALL_Arg_1 {
    NONE,
    a16,
}

impl CALL_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            CALL_Arg_1::NONE => panic!("can not call!"),
            CALL_Arg_1::a16 => cpu.read_next_word(),
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            CALL_Arg_1::NONE => panic!("can not call!"),
            CALL_Arg_1::a16 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CP_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl CP_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            CP_Arg_0::B => cpu.registers.b as u16,
            CP_Arg_0::C => cpu.registers.c as u16,
            CP_Arg_0::D => cpu.registers.d as u16,
            CP_Arg_0::E => cpu.registers.e as u16,
            CP_Arg_0::H => cpu.registers.h as u16,
            CP_Arg_0::L => cpu.registers.l as u16,
            CP_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            CP_Arg_0::A => cpu.registers.a as u16,
            CP_Arg_0::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            CP_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            CP_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            CP_Arg_0::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RET_Arg_0 {
    NONE,
    NZ,
    Z,
    NC,
    C,
}

impl RET_Arg_0 {
    pub fn condition(&self, cpu: &cpu::CPU) -> bool {
        match *self {
            RET_Arg_0::NONE => true,
            RET_Arg_0::NZ => !cpu.registers.f.zero,
            RET_Arg_0::Z => cpu.registers.f.zero,
            RET_Arg_0::NC => !cpu.registers.f.carry,
            RET_Arg_0::C => cpu.registers.f.carry,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SLA_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl SLA_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SLA_Arg_0::B => cpu.registers.b as u16,
            SLA_Arg_0::C => cpu.registers.c as u16,
            SLA_Arg_0::D => cpu.registers.d as u16,
            SLA_Arg_0::E => cpu.registers.e as u16,
            SLA_Arg_0::H => cpu.registers.h as u16,
            SLA_Arg_0::L => cpu.registers.l as u16,
            SLA_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SLA_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SLA_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SLA_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SLA_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JR_Arg_0 {
    r8,
    NZ,
    Z,
    NC,
    C,
}

impl JR_Arg_0 {
    pub fn condition(&self, cpu: &cpu::CPU) -> bool {
        match *self {
            JR_Arg_0::r8 => true,
            JR_Arg_0::NZ => !cpu.registers.f.zero,
            JR_Arg_0::Z => cpu.registers.f.zero,
            JR_Arg_0::NC => !cpu.registers.f.carry,
            JR_Arg_0::C => cpu.registers.f.carry,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JR_Arg_1 {
    NONE,
    r8,
}

impl JR_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            JR_Arg_1::NONE => panic!("can not call!"),
            JR_Arg_1::r8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            JR_Arg_1::NONE => panic!("can not call!"),
            JR_Arg_1::r8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PREFIX_Arg_0 {
    CB,
}

impl PREFIX_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum SET_Arg_0 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl SET_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum SET_Arg_1 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl SET_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SET_Arg_1::B => cpu.registers.b as u16,
            SET_Arg_1::C => cpu.registers.c as u16,
            SET_Arg_1::D => cpu.registers.d as u16,
            SET_Arg_1::E => cpu.registers.e as u16,
            SET_Arg_1::H => cpu.registers.h as u16,
            SET_Arg_1::L => cpu.registers.l as u16,
            SET_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SET_Arg_1::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SET_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SET_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SET_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RRC_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl RRC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            RRC_Arg_0::B => cpu.registers.b as u16,
            RRC_Arg_0::C => cpu.registers.c as u16,
            RRC_Arg_0::D => cpu.registers.d as u16,
            RRC_Arg_0::E => cpu.registers.e as u16,
            RRC_Arg_0::H => cpu.registers.h as u16,
            RRC_Arg_0::L => cpu.registers.l as u16,
            RRC_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            RRC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            RRC_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            RRC_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            RRC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum INC_Arg_0 {
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
    Indirect_HL,
    A,
}

impl INC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            INC_Arg_0::BC => cpu.registers.get_bc(),
            INC_Arg_0::B => cpu.registers.b as u16,
            INC_Arg_0::C => cpu.registers.c as u16,
            INC_Arg_0::DE => cpu.registers.get_de(),
            INC_Arg_0::D => cpu.registers.d as u16,
            INC_Arg_0::E => cpu.registers.e as u16,
            INC_Arg_0::HL => cpu.registers.get_hl(),
            INC_Arg_0::H => cpu.registers.h as u16,
            INC_Arg_0::L => cpu.registers.l as u16,
            INC_Arg_0::SP => cpu.sp,
            INC_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            INC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            INC_Arg_0::BC => {
                cpu.registers.set_bc(value);
                value
            }
            INC_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::DE => {
                cpu.registers.set_de(value);
                value
            }
            INC_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            INC_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            INC_Arg_0::SP => {
                cpu.sp = value;
                value
            }
            INC_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            INC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RST_Arg_0 {
    _00H,
    _08H,
    _10H,
    _18H,
    _20H,
    _28H,
    _30H,
    _38H,
}

impl RST_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum RES_Arg_0 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl RES_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum RES_Arg_1 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl RES_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            RES_Arg_1::B => cpu.registers.b as u16,
            RES_Arg_1::C => cpu.registers.c as u16,
            RES_Arg_1::D => cpu.registers.d as u16,
            RES_Arg_1::E => cpu.registers.e as u16,
            RES_Arg_1::H => cpu.registers.h as u16,
            RES_Arg_1::L => cpu.registers.l as u16,
            RES_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            RES_Arg_1::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            RES_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            RES_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            RES_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AND_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl AND_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            AND_Arg_0::B => cpu.registers.b as u16,
            AND_Arg_0::C => cpu.registers.c as u16,
            AND_Arg_0::D => cpu.registers.d as u16,
            AND_Arg_0::E => cpu.registers.e as u16,
            AND_Arg_0::H => cpu.registers.h as u16,
            AND_Arg_0::L => cpu.registers.l as u16,
            AND_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            AND_Arg_0::A => cpu.registers.a as u16,
            AND_Arg_0::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            AND_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            AND_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            AND_Arg_0::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PUSH_Arg_0 {
    BC,
    DE,
    HL,
    AF,
}

impl PUSH_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            PUSH_Arg_0::BC => cpu.registers.get_bc(),
            PUSH_Arg_0::DE => cpu.registers.get_de(),
            PUSH_Arg_0::HL => cpu.registers.get_hl(),
            PUSH_Arg_0::AF => cpu.registers.get_af(),
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            PUSH_Arg_0::BC => {
                cpu.registers.set_bc(value);
                value
            }
            PUSH_Arg_0::DE => {
                cpu.registers.set_de(value);
                value
            }
            PUSH_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            PUSH_Arg_0::AF => {
                cpu.registers.set_af(value);
                value
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum XOR_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl XOR_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            XOR_Arg_0::B => cpu.registers.b as u16,
            XOR_Arg_0::C => cpu.registers.c as u16,
            XOR_Arg_0::D => cpu.registers.d as u16,
            XOR_Arg_0::E => cpu.registers.e as u16,
            XOR_Arg_0::H => cpu.registers.h as u16,
            XOR_Arg_0::L => cpu.registers.l as u16,
            XOR_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            XOR_Arg_0::A => cpu.registers.a as u16,
            XOR_Arg_0::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            XOR_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            XOR_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            XOR_Arg_0::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum POP_Arg_0 {
    BC,
    DE,
    HL,
    AF,
}

impl POP_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            POP_Arg_0::BC => cpu.registers.get_bc(),
            POP_Arg_0::DE => cpu.registers.get_de(),
            POP_Arg_0::HL => cpu.registers.get_hl(),
            POP_Arg_0::AF => cpu.registers.get_af(),
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            POP_Arg_0::BC => {
                cpu.registers.set_bc(value);
                value
            }
            POP_Arg_0::DE => {
                cpu.registers.set_de(value);
                value
            }
            POP_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            POP_Arg_0::AF => {
                cpu.registers.set_af(value);
                value
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BIT_Arg_0 {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl BIT_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum BIT_Arg_1 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl BIT_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            BIT_Arg_1::B => cpu.registers.b as u16,
            BIT_Arg_1::C => cpu.registers.c as u16,
            BIT_Arg_1::D => cpu.registers.d as u16,
            BIT_Arg_1::E => cpu.registers.e as u16,
            BIT_Arg_1::H => cpu.registers.h as u16,
            BIT_Arg_1::L => cpu.registers.l as u16,
            BIT_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            BIT_Arg_1::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            BIT_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            BIT_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            BIT_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LD_Arg_0 {
    BC,
    Indirect_BC,
    B,
    Indirect_a16,
    A,
    C,
    DE,
    Indirect_DE,
    D,
    E,
    HL,
    Indirect_HLI,
    H,
    L,
    SP,
    Indirect_HLD,
    Indirect_HL,
    Indirect_C,
    Indirect_a16_8,
}

impl LD_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            LD_Arg_0::BC => cpu.registers.get_bc(),
            LD_Arg_0::Indirect_BC => cpu.bus.read_byte(cpu.registers.get_bc()) as u16,
            LD_Arg_0::B => cpu.registers.b as u16,
            LD_Arg_0::Indirect_a16 => {
                let addr = cpu.read_next_word();
                cpu.bus.read_word(addr)
            }
            LD_Arg_0::A => cpu.registers.a as u16,
            LD_Arg_0::C => cpu.registers.c as u16,
            LD_Arg_0::DE => cpu.registers.get_de(),
            LD_Arg_0::Indirect_DE => cpu.bus.read_byte(cpu.registers.get_de()) as u16,
            LD_Arg_0::D => cpu.registers.d as u16,
            LD_Arg_0::E => cpu.registers.e as u16,
            LD_Arg_0::HL => cpu.registers.get_hl(),
            LD_Arg_0::Indirect_HLI => {
                let value = cpu.bus.read_byte(cpu.registers.get_hl()) as u16;
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1));
                value
            }
            LD_Arg_0::H => cpu.registers.h as u16,
            LD_Arg_0::L => cpu.registers.l as u16,
            LD_Arg_0::SP => cpu.sp,
            LD_Arg_0::Indirect_HLD => {
                let value = cpu.bus.read_byte(cpu.registers.get_hl()) as u16;
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1));
                value
            }
            LD_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            LD_Arg_0::Indirect_C => cpu.bus.read_byte(0xFF00 | cpu.registers.c as u16) as u16,
            LD_Arg_0::Indirect_a16_8 => {
                let addr = cpu.read_next_word();
                cpu.bus.read_byte(addr) as u16
            }
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            LD_Arg_0::BC => {
                cpu.registers.set_bc(value);
                value
            }
            LD_Arg_0::Indirect_BC => {
                cpu.bus.write_byte(cpu.registers.get_bc(), value as u8);
                value as u8 as u16
            }
            LD_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::Indirect_a16 => {
                let addr = cpu.read_next_word();
                cpu.bus.write_word(addr, value);
                value
            }
            LD_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::DE => {
                cpu.registers.set_de(value);
                value
            }
            LD_Arg_0::Indirect_DE => {
                cpu.bus.write_byte(cpu.registers.get_de(), value as u8);
                value as u8 as u16
            }
            LD_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            LD_Arg_0::Indirect_HLI => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1));
                value as u8 as u16
            }
            LD_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            LD_Arg_0::SP => {
                cpu.sp = value;
                value
            }
            LD_Arg_0::Indirect_HLD => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1));
                value as u8 as u16
            }
            LD_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            LD_Arg_0::Indirect_C => {
                cpu.bus
                    .write_byte(0xFF00 | cpu.registers.c as u16, value as u8);
                value as u8 as u16
            }
            LD_Arg_0::Indirect_a16_8 => {
                let addr = cpu.read_next_word();
                cpu.bus.write_byte(addr, value as u8);
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LD_Arg_1 {
    d16,
    A,
    d8,
    SP,
    Indirect_BC,
    Indirect_DE,
    Indirect_HLI,
    Indirect_HLD,
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    Indirect_C,
    SP_r8,
    HL,
    Indirect_a16_8,
}

impl LD_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            LD_Arg_1::d16 => cpu.read_next_word(),
            LD_Arg_1::A => cpu.registers.a as u16,
            LD_Arg_1::d8 => cpu.read_next_byte() as u16,
            LD_Arg_1::SP => cpu.sp,
            LD_Arg_1::Indirect_BC => cpu.bus.read_byte(cpu.registers.get_bc()) as u16,
            LD_Arg_1::Indirect_DE => cpu.bus.read_byte(cpu.registers.get_de()) as u16,
            LD_Arg_1::Indirect_HLI => {
                let value = cpu.bus.read_byte(cpu.registers.get_hl()) as u16;
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1));
                value
            }
            LD_Arg_1::Indirect_HLD => {
                let value = cpu.bus.read_byte(cpu.registers.get_hl()) as u16;
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1));
                value
            }
            LD_Arg_1::B => cpu.registers.b as u16,
            LD_Arg_1::C => cpu.registers.c as u16,
            LD_Arg_1::D => cpu.registers.d as u16,
            LD_Arg_1::E => cpu.registers.e as u16,
            LD_Arg_1::H => cpu.registers.h as u16,
            LD_Arg_1::L => cpu.registers.l as u16,
            LD_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            LD_Arg_1::Indirect_C => cpu.bus.read_byte(0xFF00 | cpu.registers.c as u16) as u16,
            LD_Arg_1::SP_r8 => {
                let value = cpu.read_next_byte();
                cpu.add_e8(cpu.sp, value)
            }
            LD_Arg_1::HL => cpu.registers.get_hl(),
            LD_Arg_1::Indirect_a16_8 => {
                let addr = cpu.read_next_word();
                cpu.bus.read_byte(addr) as u16
            }
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            LD_Arg_1::d16 => panic!("can not call!"),
            LD_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::d8 => panic!("can not call!"),
            LD_Arg_1::SP => {
                cpu.sp = value;
                value
            }
            LD_Arg_1::Indirect_BC => {
                cpu.bus.write_byte(cpu.registers.get_bc(), value as u8);
                value as u8 as u16
            }
            LD_Arg_1::Indirect_DE => {
                cpu.bus.write_byte(cpu.registers.get_de(), value as u8);
                value as u8 as u16
            }
            LD_Arg_1::Indirect_HLI => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1));
                value as u8 as u16
            }
            LD_Arg_1::Indirect_HLD => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1));
                value as u8 as u16
            }
            LD_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            LD_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            LD_Arg_1::Indirect_C => {
                cpu.bus
                    .write_byte(0xFF00 | cpu.registers.c as u16, value as u8);
                value as u8 as u16
            }
            LD_Arg_1::SP_r8 => panic!("can not call!"),
            LD_Arg_1::HL => {
                cpu.registers.set_hl(value);
                value
            }
            LD_Arg_1::Indirect_a16_8 => {
                let addr = cpu.read_next_word();
                cpu.bus.write_byte(addr, value as u8);
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum STOP_Arg_0 {
    _0,
}

impl STOP_Arg_0 {}

#[derive(Debug, PartialEq)]
pub enum RL_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl RL_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            RL_Arg_0::B => cpu.registers.b as u16,
            RL_Arg_0::C => cpu.registers.c as u16,
            RL_Arg_0::D => cpu.registers.d as u16,
            RL_Arg_0::E => cpu.registers.e as u16,
            RL_Arg_0::H => cpu.registers.h as u16,
            RL_Arg_0::L => cpu.registers.l as u16,
            RL_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            RL_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            RL_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            RL_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            RL_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RR_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl RR_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            RR_Arg_0::B => cpu.registers.b as u16,
            RR_Arg_0::C => cpu.registers.c as u16,
            RR_Arg_0::D => cpu.registers.d as u16,
            RR_Arg_0::E => cpu.registers.e as u16,
            RR_Arg_0::H => cpu.registers.h as u16,
            RR_Arg_0::L => cpu.registers.l as u16,
            RR_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            RR_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            RR_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            RR_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            RR_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SRL_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl SRL_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SRL_Arg_0::B => cpu.registers.b as u16,
            SRL_Arg_0::C => cpu.registers.c as u16,
            SRL_Arg_0::D => cpu.registers.d as u16,
            SRL_Arg_0::E => cpu.registers.e as u16,
            SRL_Arg_0::H => cpu.registers.h as u16,
            SRL_Arg_0::L => cpu.registers.l as u16,
            SRL_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SRL_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SRL_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SRL_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SRL_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LDH_Arg_0 {
    Indirect_a8,
    A,
}

impl LDH_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            LDH_Arg_0::Indirect_a8 => {
                let addr = cpu.read_next_byte() as u16;
                cpu.bus.read_byte(addr) as u16
            }
            LDH_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            LDH_Arg_0::Indirect_a8 => {
                let addr = cpu.read_next_byte() as u16;
                cpu.bus.write_byte(addr, value as u8);
                value as u8 as u16
            }
            LDH_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LDH_Arg_1 {
    A,
    Indirect_a8,
}

impl LDH_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            LDH_Arg_1::A => cpu.registers.a as u16,
            LDH_Arg_1::Indirect_a8 => {
                let addr = cpu.read_next_byte() as u16;
                cpu.bus.read_byte(addr) as u16
            }
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            LDH_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            LDH_Arg_1::Indirect_a8 => {
                let addr = cpu.read_next_byte() as u16;
                cpu.bus.write_byte(addr, value as u8);
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SRA_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl SRA_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            SRA_Arg_0::B => cpu.registers.b as u16,
            SRA_Arg_0::C => cpu.registers.c as u16,
            SRA_Arg_0::D => cpu.registers.d as u16,
            SRA_Arg_0::E => cpu.registers.e as u16,
            SRA_Arg_0::H => cpu.registers.h as u16,
            SRA_Arg_0::L => cpu.registers.l as u16,
            SRA_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            SRA_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            SRA_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            SRA_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            SRA_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ADD_Arg_0 {
    HL,
    A,
    SP,
}

impl ADD_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            ADD_Arg_0::HL => cpu.registers.get_hl(),
            ADD_Arg_0::A => cpu.registers.a as u16,
            ADD_Arg_0::SP => cpu.sp,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            ADD_Arg_0::HL => {
                cpu.registers.set_hl(value);
                value
            }
            ADD_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            ADD_Arg_0::SP => {
                cpu.sp = value;
                value
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ADD_Arg_1 {
    BC,
    DE,
    HL,
    SP,
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
    r8,
}

impl ADD_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            ADD_Arg_1::BC => cpu.registers.get_bc(),
            ADD_Arg_1::DE => cpu.registers.get_de(),
            ADD_Arg_1::HL => cpu.registers.get_hl(),
            ADD_Arg_1::SP => cpu.sp,
            ADD_Arg_1::B => cpu.registers.b as u16,
            ADD_Arg_1::C => cpu.registers.c as u16,
            ADD_Arg_1::D => cpu.registers.d as u16,
            ADD_Arg_1::E => cpu.registers.e as u16,
            ADD_Arg_1::H => cpu.registers.h as u16,
            ADD_Arg_1::L => cpu.registers.l as u16,
            ADD_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            ADD_Arg_1::A => cpu.registers.a as u16,
            ADD_Arg_1::d8 => cpu.read_next_byte() as u16,
            ADD_Arg_1::r8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            ADD_Arg_1::BC => {
                cpu.registers.set_bc(value);
                value
            }
            ADD_Arg_1::DE => {
                cpu.registers.set_de(value);
                value
            }
            ADD_Arg_1::HL => {
                cpu.registers.set_hl(value);
                value
            }
            ADD_Arg_1::SP => {
                cpu.sp = value;
                value
            }
            ADD_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            ADD_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            ADD_Arg_1::d8 => panic!("can not call!"),
            ADD_Arg_1::r8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ADC_Arg_0 {
    A,
}

impl ADC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            ADC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            ADC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ADC_Arg_1 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl ADC_Arg_1 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            ADC_Arg_1::B => cpu.registers.b as u16,
            ADC_Arg_1::C => cpu.registers.c as u16,
            ADC_Arg_1::D => cpu.registers.d as u16,
            ADC_Arg_1::E => cpu.registers.e as u16,
            ADC_Arg_1::H => cpu.registers.h as u16,
            ADC_Arg_1::L => cpu.registers.l as u16,
            ADC_Arg_1::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            ADC_Arg_1::A => cpu.registers.a as u16,
            ADC_Arg_1::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            ADC_Arg_1::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            ADC_Arg_1::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            ADC_Arg_1::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OR_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
    d8,
}

impl OR_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            OR_Arg_0::B => cpu.registers.b as u16,
            OR_Arg_0::C => cpu.registers.c as u16,
            OR_Arg_0::D => cpu.registers.d as u16,
            OR_Arg_0::E => cpu.registers.e as u16,
            OR_Arg_0::H => cpu.registers.h as u16,
            OR_Arg_0::L => cpu.registers.l as u16,
            OR_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            OR_Arg_0::A => cpu.registers.a as u16,
            OR_Arg_0::d8 => cpu.read_next_byte() as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            OR_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            OR_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
            OR_Arg_0::d8 => panic!("can not call!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RLC_Arg_0 {
    B,
    C,
    D,
    E,
    H,
    L,
    Indirect_HL,
    A,
}

impl RLC_Arg_0 {
    pub fn get_value(&self, cpu: &mut cpu::CPU) -> u16 {
        match *self {
            RLC_Arg_0::B => cpu.registers.b as u16,
            RLC_Arg_0::C => cpu.registers.c as u16,
            RLC_Arg_0::D => cpu.registers.d as u16,
            RLC_Arg_0::E => cpu.registers.e as u16,
            RLC_Arg_0::H => cpu.registers.h as u16,
            RLC_Arg_0::L => cpu.registers.l as u16,
            RLC_Arg_0::Indirect_HL => cpu.bus.read_byte(cpu.registers.get_hl()) as u16,
            RLC_Arg_0::A => cpu.registers.a as u16,
        }
    }
    pub fn set_value(&self, cpu: &mut cpu::CPU, value: u16) -> u16 {
        match *self {
            RLC_Arg_0::B => {
                cpu.registers.b = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::C => {
                cpu.registers.c = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::D => {
                cpu.registers.d = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::E => {
                cpu.registers.e = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::H => {
                cpu.registers.h = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::L => {
                cpu.registers.l = value as u8;
                value as u8 as u16
            }
            RLC_Arg_0::Indirect_HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), value as u8);
                value as u8 as u16
            }
            RLC_Arg_0::A => {
                cpu.registers.a = value as u8;
                value as u8 as u16
            }
        }
    }
}

pub enum Instruction {
    DEC(DEC_Arg_0, Flags),
    JP(JP_Arg_0, JP_Arg_1, Flags),
    DAA(Flags),
    SBC(SBC_Arg_0, SBC_Arg_1, Flags),
    SWAP(SWAP_Arg_0, Flags),
    SUB(SUB_Arg_0, Flags),
    RETI(Flags),
    CALL(CALL_Arg_0, CALL_Arg_1, Flags),
    NOP(Flags),
    CP(CP_Arg_0, Flags),
    RRCA(Flags),
    RET(RET_Arg_0, Flags),
    SLA(SLA_Arg_0, Flags),
    JR(JR_Arg_0, JR_Arg_1, Flags),
    PREFIX(PREFIX_Arg_0, Flags),
    SET(SET_Arg_0, SET_Arg_1, Flags),
    DI(Flags),
    RRC(RRC_Arg_0, Flags),
    SCF(Flags),
    INC(INC_Arg_0, Flags),
    RST(RST_Arg_0, Flags),
    RES(RES_Arg_0, RES_Arg_1, Flags),
    AND(AND_Arg_0, Flags),
    PUSH(PUSH_Arg_0, Flags),
    HALT(Flags),
    XOR(XOR_Arg_0, Flags),
    POP(POP_Arg_0, Flags),
    BIT(BIT_Arg_0, BIT_Arg_1, Flags),
    RRA(Flags),
    LD(LD_Arg_0, LD_Arg_1, Flags),
    RLA(Flags),
    STOP(STOP_Arg_0, Flags),
    CCF(Flags),
    RL(RL_Arg_0, Flags),
    RR(RR_Arg_0, Flags),
    SRL(SRL_Arg_0, Flags),
    CPL(Flags),
    LDH(LDH_Arg_0, LDH_Arg_1, Flags),
    SRA(SRA_Arg_0, Flags),
    RLCA(Flags),
    ADD(ADD_Arg_0, ADD_Arg_1, Flags),
    ADC(ADC_Arg_0, ADC_Arg_1, Flags),
    EI(Flags),
    OR(OR_Arg_0, Flags),
    RLC(RLC_Arg_0, Flags),
}

pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
        0x00 => Some(Instruction::NOP(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::NO_CHANGE,
            carry: FlagValue::NO_CHANGE,
        })),
        0x01 => Some(Instruction::LD(
            LD_Arg_0::BC,
            LD_Arg_1::d16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x02 => Some(Instruction::LD(
            LD_Arg_0::Indirect_BC,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x03 => Some(Instruction::INC(
            INC_Arg_0::BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x04 => Some(Instruction::INC(
            INC_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x05 => Some(Instruction::DEC(
            DEC_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x06 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x07 => Some(Instruction::RLCA(Flags {
            zero: FlagValue::FORCE_FALSE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x08 => Some(Instruction::LD(
            LD_Arg_0::Indirect_a16,
            LD_Arg_1::SP,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x09 => Some(Instruction::ADD(
            ADD_Arg_0::HL,
            ADD_Arg_1::BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0A => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x0B => Some(Instruction::DEC(
            DEC_Arg_0::BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x0C => Some(Instruction::INC(
            INC_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x0D => Some(Instruction::DEC(
            DEC_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x0E => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x0F => Some(Instruction::RRCA(Flags {
            zero: FlagValue::FORCE_FALSE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x10 => Some(Instruction::STOP(
            STOP_Arg_0::_0,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x11 => Some(Instruction::LD(
            LD_Arg_0::DE,
            LD_Arg_1::d16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x12 => Some(Instruction::LD(
            LD_Arg_0::Indirect_DE,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x13 => Some(Instruction::INC(
            INC_Arg_0::DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x14 => Some(Instruction::INC(
            INC_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x15 => Some(Instruction::DEC(
            DEC_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x16 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x17 => Some(Instruction::RLA(Flags {
            zero: FlagValue::FORCE_FALSE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x18 => Some(Instruction::JR(
            JR_Arg_0::r8,
            JR_Arg_1::NONE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x19 => Some(Instruction::ADD(
            ADD_Arg_0::HL,
            ADD_Arg_1::DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1A => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x1B => Some(Instruction::DEC(
            DEC_Arg_0::DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x1C => Some(Instruction::INC(
            INC_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x1D => Some(Instruction::DEC(
            DEC_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x1E => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x1F => Some(Instruction::RRA(Flags {
            zero: FlagValue::FORCE_FALSE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x20 => Some(Instruction::JR(
            JR_Arg_0::NZ,
            JR_Arg_1::r8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x21 => Some(Instruction::LD(
            LD_Arg_0::HL,
            LD_Arg_1::d16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x22 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HLI,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x23 => Some(Instruction::INC(
            INC_Arg_0::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x24 => Some(Instruction::INC(
            INC_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x25 => Some(Instruction::DEC(
            DEC_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x26 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x27 => Some(Instruction::DAA(Flags {
            zero: FlagValue::CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x28 => Some(Instruction::JR(
            JR_Arg_0::Z,
            JR_Arg_1::r8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x29 => Some(Instruction::ADD(
            ADD_Arg_0::HL,
            ADD_Arg_1::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2A => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_HLI,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x2B => Some(Instruction::DEC(
            DEC_Arg_0::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x2C => Some(Instruction::INC(
            INC_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x2D => Some(Instruction::DEC(
            DEC_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x2E => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x2F => Some(Instruction::CPL(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::FORCE_TRUE,
            half_carry: FlagValue::FORCE_TRUE,
            carry: FlagValue::NO_CHANGE,
        })),
        0x30 => Some(Instruction::JR(
            JR_Arg_0::NC,
            JR_Arg_1::r8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x31 => Some(Instruction::LD(
            LD_Arg_0::SP,
            LD_Arg_1::d16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x32 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HLD,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x33 => Some(Instruction::INC(
            INC_Arg_0::SP,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x34 => Some(Instruction::INC(
            INC_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x35 => Some(Instruction::DEC(
            DEC_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x36 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x37 => Some(Instruction::SCF(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::FORCE_TRUE,
        })),
        0x38 => Some(Instruction::JR(
            JR_Arg_0::C,
            JR_Arg_1::r8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x39 => Some(Instruction::ADD(
            ADD_Arg_0::HL,
            ADD_Arg_1::SP,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3A => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_HLD,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x3B => Some(Instruction::DEC(
            DEC_Arg_0::SP,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x3C => Some(Instruction::INC(
            INC_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x3D => Some(Instruction::DEC(
            DEC_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x3E => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::d8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x3F => Some(Instruction::CCF(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::FORCE_FALSE,
            half_carry: FlagValue::FORCE_FALSE,
            carry: FlagValue::CHANGE,
        })),
        0x40 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x41 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x42 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x43 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x44 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x45 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x46 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x47 => Some(Instruction::LD(
            LD_Arg_0::B,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x48 => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x49 => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4A => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4B => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4C => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4D => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4E => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4F => Some(Instruction::LD(
            LD_Arg_0::C,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x50 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x51 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x52 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x53 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x54 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x55 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x56 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x57 => Some(Instruction::LD(
            LD_Arg_0::D,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x58 => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x59 => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5A => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5B => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5C => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5D => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5E => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5F => Some(Instruction::LD(
            LD_Arg_0::E,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x60 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x61 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x62 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x63 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x64 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x65 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x66 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x67 => Some(Instruction::LD(
            LD_Arg_0::H,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x68 => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x69 => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6A => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6B => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6C => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6D => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6E => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6F => Some(Instruction::LD(
            LD_Arg_0::L,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x70 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x71 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x72 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x73 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x74 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x75 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x76 => Some(Instruction::HALT(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::NO_CHANGE,
            carry: FlagValue::NO_CHANGE,
        })),
        0x77 => Some(Instruction::LD(
            LD_Arg_0::Indirect_HL,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x78 => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x79 => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7A => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7B => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7C => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7D => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7E => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7F => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x80 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x81 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x82 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x83 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x84 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x85 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x86 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x87 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x88 => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x89 => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8A => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8B => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8C => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8D => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8E => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x8F => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x90 => Some(Instruction::SUB(
            SUB_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x91 => Some(Instruction::SUB(
            SUB_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x92 => Some(Instruction::SUB(
            SUB_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x93 => Some(Instruction::SUB(
            SUB_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x94 => Some(Instruction::SUB(
            SUB_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x95 => Some(Instruction::SUB(
            SUB_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x96 => Some(Instruction::SUB(
            SUB_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x97 => Some(Instruction::SUB(
            SUB_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x98 => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x99 => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9A => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9B => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9C => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9D => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9E => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x9F => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xA0 => Some(Instruction::AND(
            AND_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA1 => Some(Instruction::AND(
            AND_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA2 => Some(Instruction::AND(
            AND_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA3 => Some(Instruction::AND(
            AND_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA4 => Some(Instruction::AND(
            AND_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA5 => Some(Instruction::AND(
            AND_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA6 => Some(Instruction::AND(
            AND_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA7 => Some(Instruction::AND(
            AND_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA8 => Some(Instruction::XOR(
            XOR_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xA9 => Some(Instruction::XOR(
            XOR_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAA => Some(Instruction::XOR(
            XOR_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAB => Some(Instruction::XOR(
            XOR_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAC => Some(Instruction::XOR(
            XOR_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAD => Some(Instruction::XOR(
            XOR_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAE => Some(Instruction::XOR(
            XOR_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xAF => Some(Instruction::XOR(
            XOR_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB0 => Some(Instruction::OR(
            OR_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB1 => Some(Instruction::OR(
            OR_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB2 => Some(Instruction::OR(
            OR_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB3 => Some(Instruction::OR(
            OR_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB4 => Some(Instruction::OR(
            OR_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB5 => Some(Instruction::OR(
            OR_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB6 => Some(Instruction::OR(
            OR_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB7 => Some(Instruction::OR(
            OR_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xB8 => Some(Instruction::CP(
            CP_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xB9 => Some(Instruction::CP(
            CP_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBA => Some(Instruction::CP(
            CP_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBB => Some(Instruction::CP(
            CP_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBC => Some(Instruction::CP(
            CP_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBD => Some(Instruction::CP(
            CP_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBE => Some(Instruction::CP(
            CP_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xBF => Some(Instruction::CP(
            CP_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xC0 => Some(Instruction::RET(
            RET_Arg_0::NZ,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC1 => Some(Instruction::POP(
            POP_Arg_0::BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC2 => Some(Instruction::JP(
            JP_Arg_0::NZ,
            JP_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC3 => Some(Instruction::JP(
            JP_Arg_0::a16,
            JP_Arg_1::NONE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC4 => Some(Instruction::CALL(
            CALL_Arg_0::NZ,
            CALL_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC5 => Some(Instruction::PUSH(
            PUSH_Arg_0::BC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC6 => Some(Instruction::ADD(
            ADD_Arg_0::A,
            ADD_Arg_1::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xC7 => Some(Instruction::RST(
            RST_Arg_0::_00H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC8 => Some(Instruction::RET(
            RET_Arg_0::Z,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC9 => Some(Instruction::RET(
            RET_Arg_0::NONE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCA => Some(Instruction::JP(
            JP_Arg_0::Z,
            JP_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCB => Some(Instruction::PREFIX(
            PREFIX_Arg_0::CB,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCC => Some(Instruction::CALL(
            CALL_Arg_0::Z,
            CALL_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCD => Some(Instruction::CALL(
            CALL_Arg_0::a16,
            CALL_Arg_1::NONE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCE => Some(Instruction::ADC(
            ADC_Arg_0::A,
            ADC_Arg_1::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xCF => Some(Instruction::RST(
            RST_Arg_0::_08H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD0 => Some(Instruction::RET(
            RET_Arg_0::NC,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD1 => Some(Instruction::POP(
            POP_Arg_0::DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD2 => Some(Instruction::JP(
            JP_Arg_0::NC,
            JP_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD4 => Some(Instruction::CALL(
            CALL_Arg_0::NC,
            CALL_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD5 => Some(Instruction::PUSH(
            PUSH_Arg_0::DE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD6 => Some(Instruction::SUB(
            SUB_Arg_0::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xD7 => Some(Instruction::RST(
            RST_Arg_0::_10H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD8 => Some(Instruction::RET(
            RET_Arg_0::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD9 => Some(Instruction::RETI(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::NO_CHANGE,
            carry: FlagValue::NO_CHANGE,
        })),
        0xDA => Some(Instruction::JP(
            JP_Arg_0::C,
            JP_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDC => Some(Instruction::CALL(
            CALL_Arg_0::C,
            CALL_Arg_1::a16,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDE => Some(Instruction::SBC(
            SBC_Arg_0::A,
            SBC_Arg_1::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xDF => Some(Instruction::RST(
            RST_Arg_0::_18H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE0 => Some(Instruction::LDH(
            LDH_Arg_0::Indirect_a8,
            LDH_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE1 => Some(Instruction::POP(
            POP_Arg_0::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE2 => Some(Instruction::LD(
            LD_Arg_0::Indirect_C,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE5 => Some(Instruction::PUSH(
            PUSH_Arg_0::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE6 => Some(Instruction::AND(
            AND_Arg_0::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xE7 => Some(Instruction::RST(
            RST_Arg_0::_20H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE8 => Some(Instruction::ADD(
            ADD_Arg_0::SP,
            ADD_Arg_1::r8,
            Flags {
                zero: FlagValue::FORCE_FALSE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xE9 => Some(Instruction::JP(
            JP_Arg_0::HL,
            JP_Arg_1::NONE,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEA => Some(Instruction::LD(
            LD_Arg_0::Indirect_a16_8,
            LD_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEE => Some(Instruction::XOR(
            XOR_Arg_0::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xEF => Some(Instruction::RST(
            RST_Arg_0::_28H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF0 => Some(Instruction::LDH(
            LDH_Arg_0::A,
            LDH_Arg_1::Indirect_a8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF1 => Some(Instruction::POP(
            POP_Arg_0::AF,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::CHANGE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xF2 => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF3 => Some(Instruction::DI(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::NO_CHANGE,
            carry: FlagValue::NO_CHANGE,
        })),
        0xF5 => Some(Instruction::PUSH(
            PUSH_Arg_0::AF,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF6 => Some(Instruction::OR(
            OR_Arg_0::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0xF7 => Some(Instruction::RST(
            RST_Arg_0::_30H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF8 => Some(Instruction::LD(
            LD_Arg_0::HL,
            LD_Arg_1::SP_r8,
            Flags {
                zero: FlagValue::FORCE_FALSE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xF9 => Some(Instruction::LD(
            LD_Arg_0::SP,
            LD_Arg_1::HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFA => Some(Instruction::LD(
            LD_Arg_0::A,
            LD_Arg_1::Indirect_a16_8,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFB => Some(Instruction::EI(Flags {
            zero: FlagValue::NO_CHANGE,
            subtract: FlagValue::NO_CHANGE,
            half_carry: FlagValue::NO_CHANGE,
            carry: FlagValue::NO_CHANGE,
        })),
        0xFE => Some(Instruction::CP(
            CP_Arg_0::d8,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_TRUE,
                half_carry: FlagValue::CHANGE,
                carry: FlagValue::CHANGE,
            },
        )),
        0xFF => Some(Instruction::RST(
            RST_Arg_0::_38H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        _ => None,
    }
}

pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
        0x00 => Some(Instruction::RLC(
            RLC_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x01 => Some(Instruction::RLC(
            RLC_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x02 => Some(Instruction::RLC(
            RLC_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x03 => Some(Instruction::RLC(
            RLC_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x04 => Some(Instruction::RLC(
            RLC_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x05 => Some(Instruction::RLC(
            RLC_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x06 => Some(Instruction::RLC(
            RLC_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x07 => Some(Instruction::RLC(
            RLC_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x08 => Some(Instruction::RRC(
            RRC_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x09 => Some(Instruction::RRC(
            RRC_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0A => Some(Instruction::RRC(
            RRC_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0B => Some(Instruction::RRC(
            RRC_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0C => Some(Instruction::RRC(
            RRC_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0D => Some(Instruction::RRC(
            RRC_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0E => Some(Instruction::RRC(
            RRC_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x0F => Some(Instruction::RRC(
            RRC_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x10 => Some(Instruction::RL(
            RL_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x11 => Some(Instruction::RL(
            RL_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x12 => Some(Instruction::RL(
            RL_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x13 => Some(Instruction::RL(
            RL_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x14 => Some(Instruction::RL(
            RL_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x15 => Some(Instruction::RL(
            RL_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x16 => Some(Instruction::RL(
            RL_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x17 => Some(Instruction::RL(
            RL_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x18 => Some(Instruction::RR(
            RR_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x19 => Some(Instruction::RR(
            RR_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1A => Some(Instruction::RR(
            RR_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1B => Some(Instruction::RR(
            RR_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1C => Some(Instruction::RR(
            RR_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1D => Some(Instruction::RR(
            RR_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1E => Some(Instruction::RR(
            RR_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x1F => Some(Instruction::RR(
            RR_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x20 => Some(Instruction::SLA(
            SLA_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x21 => Some(Instruction::SLA(
            SLA_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x22 => Some(Instruction::SLA(
            SLA_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x23 => Some(Instruction::SLA(
            SLA_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x24 => Some(Instruction::SLA(
            SLA_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x25 => Some(Instruction::SLA(
            SLA_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x26 => Some(Instruction::SLA(
            SLA_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x27 => Some(Instruction::SLA(
            SLA_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x28 => Some(Instruction::SRA(
            SRA_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x29 => Some(Instruction::SRA(
            SRA_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2A => Some(Instruction::SRA(
            SRA_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2B => Some(Instruction::SRA(
            SRA_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2C => Some(Instruction::SRA(
            SRA_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2D => Some(Instruction::SRA(
            SRA_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2E => Some(Instruction::SRA(
            SRA_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x2F => Some(Instruction::SRA(
            SRA_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x30 => Some(Instruction::SWAP(
            SWAP_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x31 => Some(Instruction::SWAP(
            SWAP_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x32 => Some(Instruction::SWAP(
            SWAP_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x33 => Some(Instruction::SWAP(
            SWAP_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x34 => Some(Instruction::SWAP(
            SWAP_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x35 => Some(Instruction::SWAP(
            SWAP_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x36 => Some(Instruction::SWAP(
            SWAP_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x37 => Some(Instruction::SWAP(
            SWAP_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::FORCE_FALSE,
            },
        )),
        0x38 => Some(Instruction::SRL(
            SRL_Arg_0::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x39 => Some(Instruction::SRL(
            SRL_Arg_0::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3A => Some(Instruction::SRL(
            SRL_Arg_0::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3B => Some(Instruction::SRL(
            SRL_Arg_0::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3C => Some(Instruction::SRL(
            SRL_Arg_0::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3D => Some(Instruction::SRL(
            SRL_Arg_0::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3E => Some(Instruction::SRL(
            SRL_Arg_0::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x3F => Some(Instruction::SRL(
            SRL_Arg_0::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_FALSE,
                carry: FlagValue::CHANGE,
            },
        )),
        0x40 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x41 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x42 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x43 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x44 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x45 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x46 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x47 => Some(Instruction::BIT(
            BIT_Arg_0::_0,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x48 => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x49 => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4A => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4B => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4C => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4D => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4E => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x4F => Some(Instruction::BIT(
            BIT_Arg_0::_1,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x50 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x51 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x52 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x53 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x54 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x55 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x56 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x57 => Some(Instruction::BIT(
            BIT_Arg_0::_2,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x58 => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x59 => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5A => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5B => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5C => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5D => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5E => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x5F => Some(Instruction::BIT(
            BIT_Arg_0::_3,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x60 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x61 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x62 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x63 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x64 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x65 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x66 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x67 => Some(Instruction::BIT(
            BIT_Arg_0::_4,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x68 => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x69 => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6A => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6B => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6C => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6D => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6E => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x6F => Some(Instruction::BIT(
            BIT_Arg_0::_5,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x70 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x71 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x72 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x73 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x74 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x75 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x76 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x77 => Some(Instruction::BIT(
            BIT_Arg_0::_6,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x78 => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::B,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x79 => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::C,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7A => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::D,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7B => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::E,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7C => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::H,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7D => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::L,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7E => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x7F => Some(Instruction::BIT(
            BIT_Arg_0::_7,
            BIT_Arg_1::A,
            Flags {
                zero: FlagValue::CHANGE,
                subtract: FlagValue::FORCE_FALSE,
                half_carry: FlagValue::FORCE_TRUE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x80 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x81 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x82 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x83 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x84 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x85 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x86 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x87 => Some(Instruction::RES(
            RES_Arg_0::_0,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x88 => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x89 => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8A => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8B => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8C => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8D => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8E => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x8F => Some(Instruction::RES(
            RES_Arg_0::_1,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x90 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x91 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x92 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x93 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x94 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x95 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x96 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x97 => Some(Instruction::RES(
            RES_Arg_0::_2,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x98 => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x99 => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9A => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9B => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9C => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9D => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9E => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0x9F => Some(Instruction::RES(
            RES_Arg_0::_3,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA0 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA1 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA2 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA3 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA4 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA5 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA6 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA7 => Some(Instruction::RES(
            RES_Arg_0::_4,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA8 => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xA9 => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAA => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAB => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAC => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAD => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAE => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xAF => Some(Instruction::RES(
            RES_Arg_0::_5,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB0 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB1 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB2 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB3 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB4 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB5 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB6 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB7 => Some(Instruction::RES(
            RES_Arg_0::_6,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB8 => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xB9 => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBA => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBB => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBC => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBD => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBE => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xBF => Some(Instruction::RES(
            RES_Arg_0::_7,
            RES_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC0 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC1 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC2 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC3 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC4 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC5 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC6 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC7 => Some(Instruction::SET(
            SET_Arg_0::_0,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC8 => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xC9 => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCA => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCB => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCC => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCD => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCE => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xCF => Some(Instruction::SET(
            SET_Arg_0::_1,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD0 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD1 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD2 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD3 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD4 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD5 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD6 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD7 => Some(Instruction::SET(
            SET_Arg_0::_2,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD8 => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xD9 => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDA => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDB => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDC => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDD => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDE => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xDF => Some(Instruction::SET(
            SET_Arg_0::_3,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE0 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE1 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE2 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE3 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE4 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE5 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE6 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE7 => Some(Instruction::SET(
            SET_Arg_0::_4,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE8 => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xE9 => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEA => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEB => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEC => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xED => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEE => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xEF => Some(Instruction::SET(
            SET_Arg_0::_5,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF0 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF1 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF2 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF3 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF4 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF5 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF6 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF7 => Some(Instruction::SET(
            SET_Arg_0::_6,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF8 => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::B,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xF9 => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::C,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFA => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::D,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFB => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::E,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFC => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::H,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFD => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::L,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFE => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::Indirect_HL,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        0xFF => Some(Instruction::SET(
            SET_Arg_0::_7,
            SET_Arg_1::A,
            Flags {
                zero: FlagValue::NO_CHANGE,
                subtract: FlagValue::NO_CHANGE,
                half_carry: FlagValue::NO_CHANGE,
                carry: FlagValue::NO_CHANGE,
            },
        )),
        _ => None,
    }
}

pub fn instruction_bytes(byte: u8, prefiexed: bool) -> u16 {
    match prefiexed {
        false => match byte {
            0x00 => 1,
            0x01 => 3,
            0x02 => 1,
            0x03 => 1,
            0x04 => 1,
            0x05 => 1,
            0x06 => 2,
            0x07 => 1,
            0x08 => 3,
            0x09 => 1,
            0x0A => 1,
            0x0B => 1,
            0x0C => 1,
            0x0D => 1,
            0x0E => 2,
            0x0F => 1,
            0x10 => 2,
            0x11 => 3,
            0x12 => 1,
            0x13 => 1,
            0x14 => 1,
            0x15 => 1,
            0x16 => 2,
            0x17 => 1,
            0x18 => 2,
            0x19 => 1,
            0x1A => 1,
            0x1B => 1,
            0x1C => 1,
            0x1D => 1,
            0x1E => 2,
            0x1F => 1,
            0x20 => 2,
            0x21 => 3,
            0x22 => 1,
            0x23 => 1,
            0x24 => 1,
            0x25 => 1,
            0x26 => 2,
            0x27 => 1,
            0x28 => 2,
            0x29 => 1,
            0x2A => 1,
            0x2B => 1,
            0x2C => 1,
            0x2D => 1,
            0x2E => 2,
            0x2F => 1,
            0x30 => 2,
            0x31 => 3,
            0x32 => 1,
            0x33 => 1,
            0x34 => 1,
            0x35 => 1,
            0x36 => 2,
            0x37 => 1,
            0x38 => 2,
            0x39 => 1,
            0x3A => 1,
            0x3B => 1,
            0x3C => 1,
            0x3D => 1,
            0x3E => 2,
            0x3F => 1,
            0x40 => 1,
            0x41 => 1,
            0x42 => 1,
            0x43 => 1,
            0x44 => 1,
            0x45 => 1,
            0x46 => 1,
            0x47 => 1,
            0x48 => 1,
            0x49 => 1,
            0x4A => 1,
            0x4B => 1,
            0x4C => 1,
            0x4D => 1,
            0x4E => 1,
            0x4F => 1,
            0x50 => 1,
            0x51 => 1,
            0x52 => 1,
            0x53 => 1,
            0x54 => 1,
            0x55 => 1,
            0x56 => 1,
            0x57 => 1,
            0x58 => 1,
            0x59 => 1,
            0x5A => 1,
            0x5B => 1,
            0x5C => 1,
            0x5D => 1,
            0x5E => 1,
            0x5F => 1,
            0x60 => 1,
            0x61 => 1,
            0x62 => 1,
            0x63 => 1,
            0x64 => 1,
            0x65 => 1,
            0x66 => 1,
            0x67 => 1,
            0x68 => 1,
            0x69 => 1,
            0x6A => 1,
            0x6B => 1,
            0x6C => 1,
            0x6D => 1,
            0x6E => 1,
            0x6F => 1,
            0x70 => 1,
            0x71 => 1,
            0x72 => 1,
            0x73 => 1,
            0x74 => 1,
            0x75 => 1,
            0x76 => 1,
            0x77 => 1,
            0x78 => 1,
            0x79 => 1,
            0x7A => 1,
            0x7B => 1,
            0x7C => 1,
            0x7D => 1,
            0x7E => 1,
            0x7F => 1,
            0x80 => 1,
            0x81 => 1,
            0x82 => 1,
            0x83 => 1,
            0x84 => 1,
            0x85 => 1,
            0x86 => 1,
            0x87 => 1,
            0x88 => 1,
            0x89 => 1,
            0x8A => 1,
            0x8B => 1,
            0x8C => 1,
            0x8D => 1,
            0x8E => 1,
            0x8F => 1,
            0x90 => 1,
            0x91 => 1,
            0x92 => 1,
            0x93 => 1,
            0x94 => 1,
            0x95 => 1,
            0x96 => 1,
            0x97 => 1,
            0x98 => 1,
            0x99 => 1,
            0x9A => 1,
            0x9B => 1,
            0x9C => 1,
            0x9D => 1,
            0x9E => 1,
            0x9F => 1,
            0xA0 => 1,
            0xA1 => 1,
            0xA2 => 1,
            0xA3 => 1,
            0xA4 => 1,
            0xA5 => 1,
            0xA6 => 1,
            0xA7 => 1,
            0xA8 => 1,
            0xA9 => 1,
            0xAA => 1,
            0xAB => 1,
            0xAC => 1,
            0xAD => 1,
            0xAE => 1,
            0xAF => 1,
            0xB0 => 1,
            0xB1 => 1,
            0xB2 => 1,
            0xB3 => 1,
            0xB4 => 1,
            0xB5 => 1,
            0xB6 => 1,
            0xB7 => 1,
            0xB8 => 1,
            0xB9 => 1,
            0xBA => 1,
            0xBB => 1,
            0xBC => 1,
            0xBD => 1,
            0xBE => 1,
            0xBF => 1,
            0xC0 => 1,
            0xC1 => 1,
            0xC2 => 3,
            0xC3 => 3,
            0xC4 => 3,
            0xC5 => 1,
            0xC6 => 2,
            0xC7 => 1,
            0xC8 => 1,
            0xC9 => 1,
            0xCA => 3,
            0xCB => 1,
            0xCC => 3,
            0xCD => 3,
            0xCE => 2,
            0xCF => 1,
            0xD0 => 1,
            0xD1 => 1,
            0xD2 => 3,
            0xD4 => 3,
            0xD5 => 1,
            0xD6 => 2,
            0xD7 => 1,
            0xD8 => 1,
            0xD9 => 1,
            0xDA => 3,
            0xDC => 3,
            0xDE => 2,
            0xDF => 1,
            0xE0 => 2,
            0xE1 => 1,
            0xE2 => 1,
            0xE5 => 1,
            0xE6 => 2,
            0xE7 => 1,
            0xE8 => 2,
            0xE9 => 1,
            0xEA => 3,
            0xEE => 2,
            0xEF => 1,
            0xF0 => 2,
            0xF1 => 1,
            0xF2 => 1,
            0xF3 => 1,
            0xF5 => 1,
            0xF6 => 2,
            0xF7 => 1,
            0xF8 => 2,
            0xF9 => 1,
            0xFA => 3,
            0xFB => 1,
            0xFE => 2,
            0xFF => 1,
            _ => 0,
        },
        true => match byte {
            0x00 => 2,
            0x01 => 2,
            0x02 => 2,
            0x03 => 2,
            0x04 => 2,
            0x05 => 2,
            0x06 => 2,
            0x07 => 2,
            0x08 => 2,
            0x09 => 2,
            0x0A => 2,
            0x0B => 2,
            0x0C => 2,
            0x0D => 2,
            0x0E => 2,
            0x0F => 2,
            0x10 => 2,
            0x11 => 2,
            0x12 => 2,
            0x13 => 2,
            0x14 => 2,
            0x15 => 2,
            0x16 => 2,
            0x17 => 2,
            0x18 => 2,
            0x19 => 2,
            0x1A => 2,
            0x1B => 2,
            0x1C => 2,
            0x1D => 2,
            0x1E => 2,
            0x1F => 2,
            0x20 => 2,
            0x21 => 2,
            0x22 => 2,
            0x23 => 2,
            0x24 => 2,
            0x25 => 2,
            0x26 => 2,
            0x27 => 2,
            0x28 => 2,
            0x29 => 2,
            0x2A => 2,
            0x2B => 2,
            0x2C => 2,
            0x2D => 2,
            0x2E => 2,
            0x2F => 2,
            0x30 => 2,
            0x31 => 2,
            0x32 => 2,
            0x33 => 2,
            0x34 => 2,
            0x35 => 2,
            0x36 => 2,
            0x37 => 2,
            0x38 => 2,
            0x39 => 2,
            0x3A => 2,
            0x3B => 2,
            0x3C => 2,
            0x3D => 2,
            0x3E => 2,
            0x3F => 2,
            0x40 => 2,
            0x41 => 2,
            0x42 => 2,
            0x43 => 2,
            0x44 => 2,
            0x45 => 2,
            0x46 => 2,
            0x47 => 2,
            0x48 => 2,
            0x49 => 2,
            0x4A => 2,
            0x4B => 2,
            0x4C => 2,
            0x4D => 2,
            0x4E => 2,
            0x4F => 2,
            0x50 => 2,
            0x51 => 2,
            0x52 => 2,
            0x53 => 2,
            0x54 => 2,
            0x55 => 2,
            0x56 => 2,
            0x57 => 2,
            0x58 => 2,
            0x59 => 2,
            0x5A => 2,
            0x5B => 2,
            0x5C => 2,
            0x5D => 2,
            0x5E => 2,
            0x5F => 2,
            0x60 => 2,
            0x61 => 2,
            0x62 => 2,
            0x63 => 2,
            0x64 => 2,
            0x65 => 2,
            0x66 => 2,
            0x67 => 2,
            0x68 => 2,
            0x69 => 2,
            0x6A => 2,
            0x6B => 2,
            0x6C => 2,
            0x6D => 2,
            0x6E => 2,
            0x6F => 2,
            0x70 => 2,
            0x71 => 2,
            0x72 => 2,
            0x73 => 2,
            0x74 => 2,
            0x75 => 2,
            0x76 => 2,
            0x77 => 2,
            0x78 => 2,
            0x79 => 2,
            0x7A => 2,
            0x7B => 2,
            0x7C => 2,
            0x7D => 2,
            0x7E => 2,
            0x7F => 2,
            0x80 => 2,
            0x81 => 2,
            0x82 => 2,
            0x83 => 2,
            0x84 => 2,
            0x85 => 2,
            0x86 => 2,
            0x87 => 2,
            0x88 => 2,
            0x89 => 2,
            0x8A => 2,
            0x8B => 2,
            0x8C => 2,
            0x8D => 2,
            0x8E => 2,
            0x8F => 2,
            0x90 => 2,
            0x91 => 2,
            0x92 => 2,
            0x93 => 2,
            0x94 => 2,
            0x95 => 2,
            0x96 => 2,
            0x97 => 2,
            0x98 => 2,
            0x99 => 2,
            0x9A => 2,
            0x9B => 2,
            0x9C => 2,
            0x9D => 2,
            0x9E => 2,
            0x9F => 2,
            0xA0 => 2,
            0xA1 => 2,
            0xA2 => 2,
            0xA3 => 2,
            0xA4 => 2,
            0xA5 => 2,
            0xA6 => 2,
            0xA7 => 2,
            0xA8 => 2,
            0xA9 => 2,
            0xAA => 2,
            0xAB => 2,
            0xAC => 2,
            0xAD => 2,
            0xAE => 2,
            0xAF => 2,
            0xB0 => 2,
            0xB1 => 2,
            0xB2 => 2,
            0xB3 => 2,
            0xB4 => 2,
            0xB5 => 2,
            0xB6 => 2,
            0xB7 => 2,
            0xB8 => 2,
            0xB9 => 2,
            0xBA => 2,
            0xBB => 2,
            0xBC => 2,
            0xBD => 2,
            0xBE => 2,
            0xBF => 2,
            0xC0 => 2,
            0xC1 => 2,
            0xC2 => 2,
            0xC3 => 2,
            0xC4 => 2,
            0xC5 => 2,
            0xC6 => 2,
            0xC7 => 2,
            0xC8 => 2,
            0xC9 => 2,
            0xCA => 2,
            0xCB => 2,
            0xCC => 2,
            0xCD => 2,
            0xCE => 2,
            0xCF => 2,
            0xD0 => 2,
            0xD1 => 2,
            0xD2 => 2,
            0xD3 => 2,
            0xD4 => 2,
            0xD5 => 2,
            0xD6 => 2,
            0xD7 => 2,
            0xD8 => 2,
            0xD9 => 2,
            0xDA => 2,
            0xDB => 2,
            0xDC => 2,
            0xDD => 2,
            0xDE => 2,
            0xDF => 2,
            0xE0 => 2,
            0xE1 => 2,
            0xE2 => 2,
            0xE3 => 2,
            0xE4 => 2,
            0xE5 => 2,
            0xE6 => 2,
            0xE7 => 2,
            0xE8 => 2,
            0xE9 => 2,
            0xEA => 2,
            0xEB => 2,
            0xEC => 2,
            0xED => 2,
            0xEE => 2,
            0xEF => 2,
            0xF0 => 2,
            0xF1 => 2,
            0xF2 => 2,
            0xF3 => 2,
            0xF4 => 2,
            0xF5 => 2,
            0xF6 => 2,
            0xF7 => 2,
            0xF8 => 2,
            0xF9 => 2,
            0xFA => 2,
            0xFB => 2,
            0xFC => 2,
            0xFD => 2,
            0xFE => 2,
            0xFF => 2,
            _ => 0,
        },
    }
}

pub fn instruction_cycles(byte: u8, prefiexed: bool) -> u16 {
    match prefiexed {
        false => match byte {
            0x00 => 4,
            0x01 => 12,
            0x02 => 8,
            0x03 => 8,
            0x04 => 4,
            0x05 => 4,
            0x06 => 8,
            0x07 => 4,
            0x08 => 20,
            0x09 => 8,
            0x0A => 8,
            0x0B => 8,
            0x0C => 4,
            0x0D => 4,
            0x0E => 8,
            0x0F => 4,
            0x10 => 4,
            0x11 => 12,
            0x12 => 8,
            0x13 => 8,
            0x14 => 4,
            0x15 => 4,
            0x16 => 8,
            0x17 => 4,
            0x18 => 12,
            0x19 => 8,
            0x1A => 8,
            0x1B => 8,
            0x1C => 4,
            0x1D => 4,
            0x1E => 8,
            0x1F => 4,
            0x20 => 12,
            0x21 => 12,
            0x22 => 8,
            0x23 => 8,
            0x24 => 4,
            0x25 => 4,
            0x26 => 8,
            0x27 => 4,
            0x28 => 12,
            0x29 => 8,
            0x2A => 8,
            0x2B => 8,
            0x2C => 4,
            0x2D => 4,
            0x2E => 8,
            0x2F => 4,
            0x30 => 12,
            0x31 => 12,
            0x32 => 8,
            0x33 => 8,
            0x34 => 12,
            0x35 => 12,
            0x36 => 12,
            0x37 => 4,
            0x38 => 12,
            0x39 => 8,
            0x3A => 8,
            0x3B => 8,
            0x3C => 4,
            0x3D => 4,
            0x3E => 8,
            0x3F => 4,
            0x40 => 4,
            0x41 => 4,
            0x42 => 4,
            0x43 => 4,
            0x44 => 4,
            0x45 => 4,
            0x46 => 8,
            0x47 => 4,
            0x48 => 4,
            0x49 => 4,
            0x4A => 4,
            0x4B => 4,
            0x4C => 4,
            0x4D => 4,
            0x4E => 8,
            0x4F => 4,
            0x50 => 4,
            0x51 => 4,
            0x52 => 4,
            0x53 => 4,
            0x54 => 4,
            0x55 => 4,
            0x56 => 8,
            0x57 => 4,
            0x58 => 4,
            0x59 => 4,
            0x5A => 4,
            0x5B => 4,
            0x5C => 4,
            0x5D => 4,
            0x5E => 8,
            0x5F => 4,
            0x60 => 4,
            0x61 => 4,
            0x62 => 4,
            0x63 => 4,
            0x64 => 4,
            0x65 => 4,
            0x66 => 8,
            0x67 => 4,
            0x68 => 4,
            0x69 => 4,
            0x6A => 4,
            0x6B => 4,
            0x6C => 4,
            0x6D => 4,
            0x6E => 8,
            0x6F => 4,
            0x70 => 8,
            0x71 => 8,
            0x72 => 8,
            0x73 => 8,
            0x74 => 8,
            0x75 => 8,
            0x76 => 4,
            0x77 => 8,
            0x78 => 4,
            0x79 => 4,
            0x7A => 4,
            0x7B => 4,
            0x7C => 4,
            0x7D => 4,
            0x7E => 8,
            0x7F => 4,
            0x80 => 4,
            0x81 => 4,
            0x82 => 4,
            0x83 => 4,
            0x84 => 4,
            0x85 => 4,
            0x86 => 8,
            0x87 => 4,
            0x88 => 4,
            0x89 => 4,
            0x8A => 4,
            0x8B => 4,
            0x8C => 4,
            0x8D => 4,
            0x8E => 8,
            0x8F => 4,
            0x90 => 4,
            0x91 => 4,
            0x92 => 4,
            0x93 => 4,
            0x94 => 4,
            0x95 => 4,
            0x96 => 8,
            0x97 => 4,
            0x98 => 4,
            0x99 => 4,
            0x9A => 4,
            0x9B => 4,
            0x9C => 4,
            0x9D => 4,
            0x9E => 8,
            0x9F => 4,
            0xA0 => 4,
            0xA1 => 4,
            0xA2 => 4,
            0xA3 => 4,
            0xA4 => 4,
            0xA5 => 4,
            0xA6 => 8,
            0xA7 => 4,
            0xA8 => 4,
            0xA9 => 4,
            0xAA => 4,
            0xAB => 4,
            0xAC => 4,
            0xAD => 4,
            0xAE => 8,
            0xAF => 4,
            0xB0 => 4,
            0xB1 => 4,
            0xB2 => 4,
            0xB3 => 4,
            0xB4 => 4,
            0xB5 => 4,
            0xB6 => 8,
            0xB7 => 4,
            0xB8 => 4,
            0xB9 => 4,
            0xBA => 4,
            0xBB => 4,
            0xBC => 4,
            0xBD => 4,
            0xBE => 8,
            0xBF => 4,
            0xC0 => 20,
            0xC1 => 12,
            0xC2 => 16,
            0xC3 => 16,
            0xC4 => 24,
            0xC5 => 16,
            0xC6 => 8,
            0xC7 => 16,
            0xC8 => 20,
            0xC9 => 16,
            0xCA => 16,
            0xCB => 4,
            0xCC => 24,
            0xCD => 24,
            0xCE => 8,
            0xCF => 16,
            0xD0 => 20,
            0xD1 => 12,
            0xD2 => 16,
            0xD4 => 24,
            0xD5 => 16,
            0xD6 => 8,
            0xD7 => 16,
            0xD8 => 20,
            0xD9 => 16,
            0xDA => 16,
            0xDC => 24,
            0xDE => 8,
            0xDF => 16,
            0xE0 => 12,
            0xE1 => 12,
            0xE2 => 8,
            0xE5 => 16,
            0xE6 => 8,
            0xE7 => 16,
            0xE8 => 16,
            0xE9 => 4,
            0xEA => 16,
            0xEE => 8,
            0xEF => 16,
            0xF0 => 12,
            0xF1 => 12,
            0xF2 => 8,
            0xF3 => 4,
            0xF5 => 16,
            0xF6 => 8,
            0xF7 => 16,
            0xF8 => 12,
            0xF9 => 8,
            0xFA => 16,
            0xFB => 4,
            0xFE => 8,
            0xFF => 16,
            _ => 0,
        },
        true => match byte {
            0x00 => 8,
            0x01 => 8,
            0x02 => 8,
            0x03 => 8,
            0x04 => 8,
            0x05 => 8,
            0x06 => 16,
            0x07 => 8,
            0x08 => 8,
            0x09 => 8,
            0x0A => 8,
            0x0B => 8,
            0x0C => 8,
            0x0D => 8,
            0x0E => 16,
            0x0F => 8,
            0x10 => 8,
            0x11 => 8,
            0x12 => 8,
            0x13 => 8,
            0x14 => 8,
            0x15 => 8,
            0x16 => 16,
            0x17 => 8,
            0x18 => 8,
            0x19 => 8,
            0x1A => 8,
            0x1B => 8,
            0x1C => 8,
            0x1D => 8,
            0x1E => 16,
            0x1F => 8,
            0x20 => 8,
            0x21 => 8,
            0x22 => 8,
            0x23 => 8,
            0x24 => 8,
            0x25 => 8,
            0x26 => 16,
            0x27 => 8,
            0x28 => 8,
            0x29 => 8,
            0x2A => 8,
            0x2B => 8,
            0x2C => 8,
            0x2D => 8,
            0x2E => 16,
            0x2F => 8,
            0x30 => 8,
            0x31 => 8,
            0x32 => 8,
            0x33 => 8,
            0x34 => 8,
            0x35 => 8,
            0x36 => 16,
            0x37 => 8,
            0x38 => 8,
            0x39 => 8,
            0x3A => 8,
            0x3B => 8,
            0x3C => 8,
            0x3D => 8,
            0x3E => 16,
            0x3F => 8,
            0x40 => 8,
            0x41 => 8,
            0x42 => 8,
            0x43 => 8,
            0x44 => 8,
            0x45 => 8,
            0x46 => 16,
            0x47 => 8,
            0x48 => 8,
            0x49 => 8,
            0x4A => 8,
            0x4B => 8,
            0x4C => 8,
            0x4D => 8,
            0x4E => 16,
            0x4F => 8,
            0x50 => 8,
            0x51 => 8,
            0x52 => 8,
            0x53 => 8,
            0x54 => 8,
            0x55 => 8,
            0x56 => 16,
            0x57 => 8,
            0x58 => 8,
            0x59 => 8,
            0x5A => 8,
            0x5B => 8,
            0x5C => 8,
            0x5D => 8,
            0x5E => 16,
            0x5F => 8,
            0x60 => 8,
            0x61 => 8,
            0x62 => 8,
            0x63 => 8,
            0x64 => 8,
            0x65 => 8,
            0x66 => 16,
            0x67 => 8,
            0x68 => 8,
            0x69 => 8,
            0x6A => 8,
            0x6B => 8,
            0x6C => 8,
            0x6D => 8,
            0x6E => 16,
            0x6F => 8,
            0x70 => 8,
            0x71 => 8,
            0x72 => 8,
            0x73 => 8,
            0x74 => 8,
            0x75 => 8,
            0x76 => 16,
            0x77 => 8,
            0x78 => 8,
            0x79 => 8,
            0x7A => 8,
            0x7B => 8,
            0x7C => 8,
            0x7D => 8,
            0x7E => 16,
            0x7F => 8,
            0x80 => 8,
            0x81 => 8,
            0x82 => 8,
            0x83 => 8,
            0x84 => 8,
            0x85 => 8,
            0x86 => 16,
            0x87 => 8,
            0x88 => 8,
            0x89 => 8,
            0x8A => 8,
            0x8B => 8,
            0x8C => 8,
            0x8D => 8,
            0x8E => 16,
            0x8F => 8,
            0x90 => 8,
            0x91 => 8,
            0x92 => 8,
            0x93 => 8,
            0x94 => 8,
            0x95 => 8,
            0x96 => 16,
            0x97 => 8,
            0x98 => 8,
            0x99 => 8,
            0x9A => 8,
            0x9B => 8,
            0x9C => 8,
            0x9D => 8,
            0x9E => 16,
            0x9F => 8,
            0xA0 => 8,
            0xA1 => 8,
            0xA2 => 8,
            0xA3 => 8,
            0xA4 => 8,
            0xA5 => 8,
            0xA6 => 16,
            0xA7 => 8,
            0xA8 => 8,
            0xA9 => 8,
            0xAA => 8,
            0xAB => 8,
            0xAC => 8,
            0xAD => 8,
            0xAE => 16,
            0xAF => 8,
            0xB0 => 8,
            0xB1 => 8,
            0xB2 => 8,
            0xB3 => 8,
            0xB4 => 8,
            0xB5 => 8,
            0xB6 => 16,
            0xB7 => 8,
            0xB8 => 8,
            0xB9 => 8,
            0xBA => 8,
            0xBB => 8,
            0xBC => 8,
            0xBD => 8,
            0xBE => 16,
            0xBF => 8,
            0xC0 => 8,
            0xC1 => 8,
            0xC2 => 8,
            0xC3 => 8,
            0xC4 => 8,
            0xC5 => 8,
            0xC6 => 16,
            0xC7 => 8,
            0xC8 => 8,
            0xC9 => 8,
            0xCA => 8,
            0xCB => 8,
            0xCC => 8,
            0xCD => 8,
            0xCE => 16,
            0xCF => 8,
            0xD0 => 8,
            0xD1 => 8,
            0xD2 => 8,
            0xD3 => 8,
            0xD4 => 8,
            0xD5 => 8,
            0xD6 => 16,
            0xD7 => 8,
            0xD8 => 8,
            0xD9 => 8,
            0xDA => 8,
            0xDB => 8,
            0xDC => 8,
            0xDD => 8,
            0xDE => 16,
            0xDF => 8,
            0xE0 => 8,
            0xE1 => 8,
            0xE2 => 8,
            0xE3 => 8,
            0xE4 => 8,
            0xE5 => 8,
            0xE6 => 16,
            0xE7 => 8,
            0xE8 => 8,
            0xE9 => 8,
            0xEA => 8,
            0xEB => 8,
            0xEC => 8,
            0xED => 8,
            0xEE => 16,
            0xEF => 8,
            0xF0 => 8,
            0xF1 => 8,
            0xF2 => 8,
            0xF3 => 8,
            0xF4 => 8,
            0xF5 => 8,
            0xF6 => 16,
            0xF7 => 8,
            0xF8 => 8,
            0xF9 => 8,
            0xFA => 8,
            0xFB => 8,
            0xFC => 8,
            0xFD => 8,
            0xFE => 16,
            0xFF => 8,
            _ => 0,
        },
    }
}
