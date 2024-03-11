mod instruction;

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

#[derive(Debug, PartialEq)]
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
    fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        }
    }
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }
    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }
    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

impl instruction::Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<instruction::Instruction> {
        if prefixed {
            instruction::from_byte_prefixed(byte)
        } else {
            instruction::from_byte_not_prefixed(byte)
        }
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: Registers::new(),
            pc: 0x0000,
            sp: 0x0000, // FIXME たぶん0xFFFF
            bus: MemoryBus::new(),
        }
    }

    fn execute(&mut self, instruction: instruction::Instruction) -> u16 {
        match instruction {
            instruction::Instruction::ADD(arg0, arg1) => match arg0 {
                instruction::ADD_Arg_0::A => {
                    match arg1 {
                        instruction::ADD_Arg_1::A => self.pc, // FIXME 実装する
                        instruction::ADD_Arg_1::B => self.pc,
                        instruction::ADD_Arg_1::C => {
                            let value = self.registers.c;
                            let new_value = self.add(value);
                            self.registers.a = new_value;
                            self.pc.wrapping_add(1)
                        }
                        instruction::ADD_Arg_1::D => self.pc,
                        instruction::ADD_Arg_1::E => self.pc,
                        instruction::ADD_Arg_1::H => self.pc,
                        instruction::ADD_Arg_1::L => self.pc,
                        _ => todo!("implement"),
                    }
                }
                _ => todo!("implement"),
            },
            instruction::Instruction::JP(arg0, arg1) => {
                let jump_condition = match arg0 {
                    instruction::JP_Arg_0::NZ => !self.registers.f.zero,
                    instruction::JP_Arg_0::Z => self.registers.f.zero,
                    instruction::JP_Arg_0::NC => !self.registers.f.carry,
                    instruction::JP_Arg_0::C => self.registers.f.carry,
                    instruction::JP_Arg_0::a16 => true,
                    _ => todo!("impl"),
                };
                self.jump(jump_condition)
            }
            // instruction::Instruction::LD(arg0, arg1) => {
            //       let source_value = match arg1 {
            //         instruction::LD_Arg_1::A => self.registers.a,
            //         instruction::LD_Arg_1::B => self.registers.b,
            //         instruction::LD_Arg_1::C => self.registers.c,
            //         instruction::LD_Arg_1::D => self.registers.d,
            //         instruction::LD_Arg_1::E => self.registers.e,
            //         instruction::LD_Arg_1::H => self.registers.h,
            //         instruction::LD_Arg_1::L => self.registers.l,
            //         instruction::LD_Arg_1::d8 => self.read_next_byte(),
            //         instruction::LD_Arg_1::Indirect_HLI => self.bus.read_byte(self.registers.get_hl()),
            //         _ => todo!("impl")
            //       };
            //       match arg0 {
            //         instruction::LD_Arg_0::A => self.registers.a = source_value,
            //         instruction::LD_Arg_0::B => self.registers.b = source_value,
            //         instruction::LD_Arg_0::C => self.registers.c = source_value,
            //         instruction::LD_Arg_0::D => self.registers.d = source_value,
            //         instruction::LD_Arg_0::E => self.registers.e = source_value,
            //         instruction::LD_Arg_0::H => self.registers.h = source_value,
            //         instruction::LD_Arg_0::L => self.registers.l = source_value,
            //         instruction::LD_Arg_0::Indirect_HLI => {
            //                 self.bus.write_byte(self.registers.get_hl(), source_value)
            //             }
            //             _ => todo!("impl")
            //         };
            //         // match source {
            //         //     LoadByteSource::D8 => self.pc.wrapping_add(2),
            //         //     _ => self.pc.wrapping_add(1),
            //         // }
            //     }
            // },
            _ => todo!("impl"), // Instruction::PUSH(target) => {
                                //     let value = match target {
                                //         StackTarget::BC => self.registers.get_bc(),
                                //     };
                                //     self.push(value);
                                //     self.pc.wrapping_add(1)
                                // }
                                // Instruction::POP(target) => {
                                //     let result = self.pop();
                                //     match target {
                                //         StackTarget::BC => self.registers.set_bc(result),
                                //     }
                                //     self.pc.wrapping_add(1)
                                // }
                                // Instruction::CALL(test) => {
                                //     let jump_condition = match test {
                                //         JumpTest::NotZero => !self.registers.f.zero,
                                //         _ => {
                                //             panic!("TODO: support more condition")
                                //         }
                                //     };
                                //     self.call(jump_condition)
                                // }
                                // Instruction::RET(test) => {
                                //     let jump_condition = match test {
                                //         JumpTest::NotZero => !self.registers.f.zero,
                                //         _ => {
                                //             panic!("TODO: support more condition")
                                //         }
                                //     };
                                //     self.return_(jump_condition)
                                // }
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
            self.read_next_word()
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) =
            instruction::Instruction::from_byte(instruction_byte, prefixed)
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

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        let l = self.bus.read_byte(self.pc + 1) as u16;
        let u = self.bus.read_byte(self.pc + 2) as u16;
        return (u << 8) | l;
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_a_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81); // ADD A, C
        cpu.registers.c = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            }
        );
    }

    #[test]
    fn test_add_a_c_zero() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81); // ADD A, C
        cpu.registers.c = 0x00;
        cpu.registers.a = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false,
            }
        );
    }

    #[test]
    fn test_add_a_c_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81); // ADD A, C
        cpu.registers.c = 0xF0;
        cpu.registers.a = 0x20;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: true,
            }
        );
    }

    #[test]
    fn test_add_a_c_half_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x81); // ADD A, C
        cpu.registers.c = 0x0F;
        cpu.registers.a = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(
            cpu.registers.f,
            FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: true,
                carry: false,
            }
        );
    }

    #[test]
    fn test_jp_zero() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCA); // JP Z, a16
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.registers.f.zero = true;
        cpu.step();
        assert_eq!(cpu.pc, 0x0201);
    }

    #[test]
    fn test_jp_zero_fail() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCA); // JP Z, a16
        cpu.bus.write_byte(0x0001, 0x01);
        cpu.bus.write_byte(0x0002, 0x02);
        cpu.registers.f.zero = false;
        cpu.step();
        assert_eq!(cpu.pc, 0x0003);
    }
}
