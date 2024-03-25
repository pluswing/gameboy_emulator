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

#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (u8::from(self.f) as u16)
    }
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0x00FF) as u8);
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

    fn execute(&mut self, instruction: instruction::Instruction) {
        match instruction {
            instruction::Instruction::DEC(arg0, flags) => self.dec(arg0, flags),
            instruction::Instruction::JP(arg0, arg1, flags) => self.jp(arg0, arg1, flags),
            instruction::Instruction::DAA(flags) => self.daa(flags),
            instruction::Instruction::SBC(arg0, arg1, flags) => self.sbc(arg0, arg1, flags),
            instruction::Instruction::SWAP(arg0, flags) => self.swap(arg0, flags),
            instruction::Instruction::SUB(arg0, flags) => self.sub(arg0, flags),
            instruction::Instruction::RETI(flags) => self.reti(flags),
            instruction::Instruction::CALL(arg0, arg1, flags) => self.call(arg0, arg1, flags),
            instruction::Instruction::NOP(flags) => self.nop(flags),
            instruction::Instruction::CP(arg0, flags) => self.cp(arg0, flags),
            instruction::Instruction::RRCA(flags) => self.rrca(flags),
            instruction::Instruction::RET(arg0, flags) => self.ret(arg0, flags),
            instruction::Instruction::SLA(arg0, flags) => self.sla(arg0, flags),
            instruction::Instruction::JR(arg0, arg1, flags) => self.jr(arg0, arg1, flags),
            instruction::Instruction::PREFIX(arg0, flags) => self.prefix(arg0, flags),
            instruction::Instruction::SET(arg0, arg1, flags) => self.set(arg0, arg1, flags),
            instruction::Instruction::DI(flags) => self.di(flags),
            instruction::Instruction::RRC(arg0, flags) => self.rrc(arg0, flags),
            instruction::Instruction::SCF(flags) => self.scf(flags),
            instruction::Instruction::INC(arg0, flags) => self.inc(arg0, flags),
            instruction::Instruction::RST(arg0, flags) => self.rst(arg0, flags),
            instruction::Instruction::RES(arg0, arg1, flags) => self.res(arg0, arg1, flags),
            instruction::Instruction::AND(arg0, flags) => self.and(arg0, flags),
            instruction::Instruction::PUSH(arg0, flags) => self.push(arg0, flags),
            instruction::Instruction::HALT(flags) => self.halt(flags),
            instruction::Instruction::XOR(arg0, flags) => self.xor(arg0, flags),
            instruction::Instruction::POP(arg0, flags) => self.pop(arg0, flags),
            instruction::Instruction::BIT(arg0, arg1, flags) => self.bit(arg0, arg1, flags),
            instruction::Instruction::RRA(flags) => self.rra(flags),
            instruction::Instruction::LD(arg0, arg1, flags) => self.ld(arg0, arg1, flags),
            instruction::Instruction::RLA(flags) => self.rla(flags),
            instruction::Instruction::STOP(arg0, flags) => self.stop(arg0, flags),
            instruction::Instruction::CCF(flags) => self.ccf(flags),
            instruction::Instruction::RL(arg0, flags) => self.rl(arg0, flags),
            instruction::Instruction::RR(arg0, flags) => self.rr(arg0, flags),
            instruction::Instruction::SRL(arg0, flags) => self.srl(arg0, flags),
            instruction::Instruction::CPL(flags) => self.cpl(flags),
            instruction::Instruction::LDH(arg0, arg1, flags) => self.ldh(arg0, arg1, flags),
            instruction::Instruction::SRA(arg0, flags) => self.sra(arg0, flags),
            instruction::Instruction::RLCA(flags) => self.rlca(flags),
            instruction::Instruction::ADD(arg0, arg1, flags) => self.add(arg0, arg1, flags),
            instruction::Instruction::ADC(arg0, arg1, flags) => self.adc(arg0, arg1, flags),
            instruction::Instruction::EI(flags) => self.ei(flags),
            instruction::Instruction::OR(arg0, flags) => self.or(arg0, flags),
            instruction::Instruction::RLC(arg0, flags) => self.rlc(arg0, flags),
        }
    }

    fn jp(
        &mut self,
        arg0: instruction::JP_Arg_0,
        arg1: instruction::JP_Arg_1,
        flags: instruction::Flags,
    ) {
        let jump_condition = match arg0 {
            instruction::JP_Arg_0::NZ => !self.registers.f.zero,
            instruction::JP_Arg_0::Z => self.registers.f.zero,
            instruction::JP_Arg_0::NC => !self.registers.f.carry,
            instruction::JP_Arg_0::C => self.registers.f.carry,
            instruction::JP_Arg_0::a16 => true,
            _ => todo!("impl"),
        };
        if jump_condition {
            self.pc = self.read_next_word()
                - match arg0 {
                    instruction::JP_Arg_0::Indirect_HL => 1,
                    _ => 3,
                }
        }
    }

    fn call(
        &mut self,
        arg0: instruction::CALL_Arg_0,
        arg1: instruction::CALL_Arg_1,
        flags: instruction::Flags,
    ) {
        let jump_condition = match arg0 {
            instruction::CALL_Arg_0::NZ => !self.registers.f.zero,
            instruction::CALL_Arg_0::Z => self.registers.f.zero,
            instruction::CALL_Arg_0::NC => !self.registers.f.carry,
            instruction::CALL_Arg_0::C => self.registers.f.carry,
            instruction::CALL_Arg_0::a16 => true,
        };
        let next_pc = self.pc.wrapping_add(3);
        if jump_condition {
            self.push_u16(next_pc);
            self.pc = self.pc.wrapping_add(self.read_next_word().wrapping_sub(3))
        }
        // self.update_flags(value, flags)
    }
    fn ret(&mut self, arg0: instruction::RET_Arg_0, flags: instruction::Flags) {
        let jump_condition = match arg0 {
            instruction::RET_Arg_0::NZ => !self.registers.f.zero,
            instruction::RET_Arg_0::Z => self.registers.f.zero,
            instruction::RET_Arg_0::NC => !self.registers.f.carry,
            instruction::RET_Arg_0::C => self.registers.f.carry,
            instruction::RET_Arg_0::NONE => true,
        };
        if jump_condition {
            let pc = self.pop_u16();
            // 共通処理でPCがbyte数足されるので、それを考慮して引いておく。
            self.pc = pc - 1
        }
    }
    fn push(&mut self, arg0: instruction::PUSH_Arg_0, flags: instruction::Flags) {
        let value = match arg0 {
            instruction::PUSH_Arg_0::AF => self.registers.get_af(),
            instruction::PUSH_Arg_0::BC => self.registers.get_bc(),
            instruction::PUSH_Arg_0::DE => self.registers.get_de(),
            instruction::PUSH_Arg_0::HL => self.registers.get_hl(),
        };
        self.push_u16(value);
    }
    fn pop(&mut self, arg0: instruction::POP_Arg_0, flags: instruction::Flags) {
        let value = self.pop_u16();
        match arg0 {
            instruction::POP_Arg_0::AF => self.registers.set_af(value),
            instruction::POP_Arg_0::BC => self.registers.set_bc(value),
            instruction::POP_Arg_0::DE => self.registers.set_de(value),
            instruction::POP_Arg_0::HL => self.registers.set_hl(value),
        }
    }
    fn ld(
        &mut self,
        arg0: instruction::LD_Arg_0,
        arg1: instruction::LD_Arg_1,
        flags: instruction::Flags,
    ) {
        let source_value = match arg1 {
            instruction::LD_Arg_1::A => self.registers.a,
            instruction::LD_Arg_1::B => self.registers.b,
            instruction::LD_Arg_1::C => self.registers.c,
            instruction::LD_Arg_1::D => self.registers.d,
            instruction::LD_Arg_1::E => self.registers.e,
            instruction::LD_Arg_1::H => self.registers.h,
            instruction::LD_Arg_1::L => self.registers.l,
            instruction::LD_Arg_1::d8 => self.read_next_byte(),
            instruction::LD_Arg_1::Indirect_HLI => self.bus.read_byte(self.registers.get_hl()),
            _ => todo!("impl"),
        };
        match arg0 {
            instruction::LD_Arg_0::A => self.registers.a = source_value,
            instruction::LD_Arg_0::B => self.registers.b = source_value,
            instruction::LD_Arg_0::C => self.registers.c = source_value,
            instruction::LD_Arg_0::D => self.registers.d = source_value,
            instruction::LD_Arg_0::E => self.registers.e = source_value,
            instruction::LD_Arg_0::H => self.registers.h = source_value,
            instruction::LD_Arg_0::L => self.registers.l = source_value,
            instruction::LD_Arg_0::Indirect_HLI => {
                self.bus.write_byte(self.registers.get_hl(), source_value)
            }
            _ => todo!("impl"),
        };
    }
    fn add(
        &mut self,
        arg0: instruction::ADD_Arg_0,
        arg1: instruction::ADD_Arg_1,
        flags: instruction::Flags,
    ) {
        match arg0 {
            instruction::ADD_Arg_0::A => {
                let value = match arg1 {
                    instruction::ADD_Arg_1::A => self.registers.a,
                    instruction::ADD_Arg_1::B => self.registers.b,
                    instruction::ADD_Arg_1::C => self.registers.c,
                    instruction::ADD_Arg_1::D => self.registers.d,
                    instruction::ADD_Arg_1::E => self.registers.e,
                    instruction::ADD_Arg_1::H => self.registers.h,
                    instruction::ADD_Arg_1::L => self.registers.l,
                    instruction::ADD_Arg_1::Indirect_HL => {
                        self.bus.read_byte(self.registers.get_hl())
                    }
                    instruction::ADD_Arg_1::d8 => self.read_next_byte(),
                    _ => todo!("implement"),
                };
                self.add_a(value);
                self.update_flags(self.registers.a, flags);
            }
            instruction::ADD_Arg_0::HL => {
                let value = match arg1 {
                    instruction::ADD_Arg_1::BC => self.registers.get_bc(),
                    instruction::ADD_Arg_1::DE => self.registers.get_de(),
                    instruction::ADD_Arg_1::HL => self.registers.get_hl(),
                    instruction::ADD_Arg_1::SP => self.sp,
                    _ => todo!("impl"),
                };
                // TODO フラグの変更
                self.registers.set_hl(self.registers.get_hl() + value);
            }
            instruction::ADD_Arg_0::SP => {
                let value = match arg1 {
                    instruction::ADD_Arg_1::r8 => self.read_next_byte(),
                    _ => todo!("impl"),
                } as i8; // 符号ありに変える
                let left = self.sp as i32;
                let value = left + value as i32;
                self.sp = value as u16;
                // TODO フラグの変更
            }
        }
    }

    fn dec(&mut self, arg0: instruction::DEC_Arg_0, flags: instruction::Flags) {}
    fn daa(&mut self, flags: instruction::Flags) {}
    fn sbc(
        &mut self,
        arg0: instruction::SBC_Arg_0,
        arg1: instruction::SBC_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn swap(&mut self, arg0: instruction::SWAP_Arg_0, flags: instruction::Flags) {}
    fn sub(&mut self, arg0: instruction::SUB_Arg_0, flags: instruction::Flags) {}
    fn reti(&mut self, flags: instruction::Flags) {}
    fn nop(&mut self, flags: instruction::Flags) {}
    fn cp(&mut self, arg0: instruction::CP_Arg_0, flags: instruction::Flags) {}
    fn rrca(&mut self, flags: instruction::Flags) {}
    fn sla(&mut self, arg0: instruction::SLA_Arg_0, flags: instruction::Flags) {}
    fn jr(
        &mut self,
        arg0: instruction::JR_Arg_0,
        arg1: instruction::JR_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn prefix(&mut self, arg0: instruction::PREFIX_Arg_0, flags: instruction::Flags) {}
    fn set(
        &mut self,
        arg0: instruction::SET_Arg_0,
        arg1: instruction::SET_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn di(&mut self, flags: instruction::Flags) {}
    fn rrc(&mut self, arg0: instruction::RRC_Arg_0, flags: instruction::Flags) {}
    fn scf(&mut self, flags: instruction::Flags) {}
    fn inc(&mut self, arg0: instruction::INC_Arg_0, flags: instruction::Flags) {}
    fn rst(&mut self, arg0: instruction::RST_Arg_0, flags: instruction::Flags) {}
    fn res(
        &mut self,
        arg0: instruction::RES_Arg_0,
        arg1: instruction::RES_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn and(&mut self, arg0: instruction::AND_Arg_0, flags: instruction::Flags) {}
    fn halt(&mut self, flags: instruction::Flags) {}
    fn xor(&mut self, arg0: instruction::XOR_Arg_0, flags: instruction::Flags) {}
    fn bit(
        &mut self,
        arg0: instruction::BIT_Arg_0,
        arg1: instruction::BIT_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn rra(&mut self, flags: instruction::Flags) {}
    fn rla(&mut self, flags: instruction::Flags) {}
    fn stop(&mut self, arg0: instruction::STOP_Arg_0, flags: instruction::Flags) {}
    fn ccf(&mut self, flags: instruction::Flags) {}
    fn rl(&mut self, arg0: instruction::RL_Arg_0, flags: instruction::Flags) {}
    fn rr(&mut self, arg0: instruction::RR_Arg_0, flags: instruction::Flags) {}
    fn srl(&mut self, arg0: instruction::SRL_Arg_0, flags: instruction::Flags) {}
    fn cpl(&mut self, flags: instruction::Flags) {}
    fn ldh(
        &mut self,
        arg0: instruction::LDH_Arg_0,
        arg1: instruction::LDH_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn sra(&mut self, arg0: instruction::SRA_Arg_0, flags: instruction::Flags) {}
    fn rlca(&mut self, flags: instruction::Flags) {}
    fn adc(
        &mut self,
        arg0: instruction::ADC_Arg_0,
        arg1: instruction::ADC_Arg_1,
        flags: instruction::Flags,
    ) {
    }
    fn ei(&mut self, flags: instruction::Flags) {}
    fn or(&mut self, arg0: instruction::OR_Arg_0, flags: instruction::Flags) {}
    fn rlc(&mut self, arg0: instruction::RLC_Arg_0, flags: instruction::Flags) {}

    fn add_a(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;
        self.registers.a = new_value;
        new_value
    }

    fn update_flags(&mut self, value: u8, flags: instruction::Flags) {
        self.registers.f.zero = match flags.zero {
            instruction::FlagValue::FORCE_FALSE => false,
            instruction::FlagValue::FORCE_TRUE => true,
            instruction::FlagValue::CHANGE => value == 0,
            instruction::FlagValue::NO_CHANGE => self.registers.f.zero,
        };
        self.registers.f.subtract = match flags.subtract {
            instruction::FlagValue::FORCE_FALSE => false,
            instruction::FlagValue::FORCE_TRUE => true,
            instruction::FlagValue::CHANGE => panic!("subtract flag CHANGE not support"),
            instruction::FlagValue::NO_CHANGE => self.registers.f.subtract,
        };
        self.registers.f.carry = match flags.carry {
            instruction::FlagValue::FORCE_FALSE => false,
            instruction::FlagValue::FORCE_TRUE => true,
            // carryは各命令で変更する
            _ => self.registers.f.carry,
        };
        self.registers.f.half_carry = match flags.subtract {
            instruction::FlagValue::FORCE_FALSE => false,
            instruction::FlagValue::FORCE_TRUE => true,
            //half_carryは各命令で変更する
            _ => self.registers.f.half_carry,
        };
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        if let Some(instruction) = instruction::Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:02X}",
                if prefixed { "CB" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description)
        };
        self.pc = self
            .pc
            .wrapping_add(instruction::instruction_bytes(instruction_byte, prefixed))
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        let l = self.bus.read_byte(self.pc + 1) as u16;
        let u = self.bus.read_byte(self.pc + 2) as u16;
        return (u << 8) | l;
    }

    fn push_u16(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    fn pop_u16(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (msb << 8) | lsb
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

    fn F(zero: bool, subtract: bool, half_carry: bool, carry: bool) -> FlagsRegister {
        FlagsRegister {
            zero: zero,
            subtract: subtract,
            half_carry: half_carry,
            carry: carry,
        }
    }

    #[test]
    fn test_add_a() {
        // A, A
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x87);
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x04);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, B
        cpu.bus.write_byte(0x0001, 0x80);
        cpu.registers.b = 0x03;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x05);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, C
        cpu.bus.write_byte(0x0002, 0x81);
        cpu.registers.c = 0x04;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x06);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, D
        cpu.bus.write_byte(0x0003, 0x82);
        cpu.registers.d = 0x05;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x07);
        assert_eq!(cpu.pc, 0x0004);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, E
        cpu.bus.write_byte(0x0004, 0x83);
        cpu.registers.e = 0x06;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x08);
        assert_eq!(cpu.pc, 0x0005);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, H
        cpu.bus.write_byte(0x0005, 0x84);
        cpu.registers.h = 0x07;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x09);
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, L
        cpu.bus.write_byte(0x0006, 0x85);
        cpu.registers.l = 0x08;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x0A);
        assert_eq!(cpu.pc, 0x0007);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        // A, (HL)
        cpu.bus.write_byte(0x0007, 0x86);
        cpu.registers.set_hl(0x1040);
        cpu.bus.memory[0x1040] = 0x09;
        cpu.registers.a = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x0B);
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
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
        assert_eq!(cpu.registers.f, F(true, false, false, false));
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
        assert_eq!(cpu.registers.f, F(false, false, false, true));
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
        assert_eq!(cpu.registers.f, F(false, false, true, false));
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
