fn main() {
    println!("Hello, world!");
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0x01) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0x01) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0x01) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // 0x00 => Some(Instruction::RLC(PrefixTrarget::B)),
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // 0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            // ..
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            _ => None,
        }
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => self.pc,
                ArithmeticTarget::B => self.pc,
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => self.pc,
                ArithmeticTarget::E => self.pc,
                ArithmeticTarget::H => self.pc,
                ArithmeticTarget::L => self.pc,
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HLI => {
                            self.bus_write_byte(self.registers.get_hl(), source_value)
                        }
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;
        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:02X}",
                if prefixed { "CB" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description)
        };
        self.pc = next_pc;
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
