use core::panic;

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
            instruction::JP_Arg_0::HL => true,
            instruction::JP_Arg_0::a16 => true,
        };
        if jump_condition {
            self.pc = match arg0 {
                instruction::JP_Arg_0::HL => self.registers.get_hl() - 1,
                _ => self.read_next_word() - 3,
            };
        }
        self.update_flags(0, flags);
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
        self.update_flags(0, flags);
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
        self.update_flags(0, flags);
    }
    fn push(&mut self, arg0: instruction::PUSH_Arg_0, flags: instruction::Flags) {
        let value = match arg0 {
            instruction::PUSH_Arg_0::AF => self.registers.get_af(),
            instruction::PUSH_Arg_0::BC => self.registers.get_bc(),
            instruction::PUSH_Arg_0::DE => self.registers.get_de(),
            instruction::PUSH_Arg_0::HL => self.registers.get_hl(),
        };
        self.push_u16(value);
        self.update_flags(value, flags);
    }
    fn pop(&mut self, arg0: instruction::POP_Arg_0, flags: instruction::Flags) {
        let value = self.pop_u16();
        match arg0 {
            instruction::POP_Arg_0::AF => self.registers.set_af(value),
            instruction::POP_Arg_0::BC => self.registers.set_bc(value),
            instruction::POP_Arg_0::DE => self.registers.set_de(value),
            instruction::POP_Arg_0::HL => self.registers.set_hl(value),
        };
        self.update_flags(value, flags);
    }
    fn ld(
        &mut self,
        arg0: instruction::LD_Arg_0,
        arg1: instruction::LD_Arg_1,
        flags: instruction::Flags,
    ) {
        if self.is_16bit_ld_operation(&arg1) {
            self.ld_16bit(arg0, arg1, flags);
        } else {
            self.ld_8bit(arg0, arg1, flags)
        };
    }

    fn is_16bit_ld_operation(&self, arg1: &instruction::LD_Arg_1) -> bool {
        match arg1 {
            instruction::LD_Arg_1::d16 => true,
            instruction::LD_Arg_1::HL => true,
            instruction::LD_Arg_1::SP => true,
            instruction::LD_Arg_1::SP_r8 => true,
            _ => false,
        }
    }

    fn ld_16bit(
        &mut self,
        arg0: instruction::LD_Arg_0,
        arg1: instruction::LD_Arg_1,
        flags: instruction::Flags,
    ) {
        let source_value = match arg1 {
            instruction::LD_Arg_1::d16 => self.read_next_word(),
            instruction::LD_Arg_1::HL => self.registers.get_hl(),
            instruction::LD_Arg_1::SP => self.sp,
            instruction::LD_Arg_1::SP_r8 => {
                self.sp = self.add_e8(self.sp, self.read_next_byte());
                self.sp
            }
            _ => panic!("shound not reach"),
        };

        match arg0 {
            instruction::LD_Arg_0::BC => {
                self.registers.set_bc(source_value);
            }
            instruction::LD_Arg_0::DE => {
                self.registers.set_de(source_value);
            }
            instruction::LD_Arg_0::HL => {
                self.registers.set_hl(source_value);
            }
            instruction::LD_Arg_0::SP => {
                self.sp = source_value;
            }
            instruction::LD_Arg_0::Indirect_a16 => {
                self.bus.write_word(self.read_next_word(), source_value);
            }
            _ => panic!("shound not reach"),
        };
        self.update_flags(source_value, flags);
    }

    fn ld_8bit(
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
            instruction::LD_Arg_1::Indirect_a16 => {
                let v = self.read_next_word();
                self.bus.read_byte(v)
            }
            instruction::LD_Arg_1::Indirect_HLI => {
                let v = self.bus.read_byte(self.registers.get_hl());
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
                v
            }
            instruction::LD_Arg_1::Indirect_HLD => {
                let v = self.bus.read_byte(self.registers.get_hl());
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
                v
            }
            instruction::LD_Arg_1::Indirect_BC => self.bus.read_byte(self.registers.get_bc()),
            instruction::LD_Arg_1::Indirect_DE => self.bus.read_byte(self.registers.get_de()),
            instruction::LD_Arg_1::Indirect_HL => self.bus.read_byte(self.registers.get_hl()),
            instruction::LD_Arg_1::Indirect_C => {
                self.bus.read_byte(0xFF00 | self.registers.c as u16)
            }
            _ => panic!("should not reach"),
        };

        match arg0 {
            instruction::LD_Arg_0::A => self.registers.a = source_value,
            instruction::LD_Arg_0::B => self.registers.b = source_value,
            instruction::LD_Arg_0::C => self.registers.c = source_value,
            instruction::LD_Arg_0::D => self.registers.d = source_value,
            instruction::LD_Arg_0::E => self.registers.e = source_value,
            instruction::LD_Arg_0::H => self.registers.h = source_value,
            instruction::LD_Arg_0::L => self.registers.l = source_value,
            instruction::LD_Arg_0::Indirect_HL => {
                self.bus.write_byte(self.registers.get_hl(), source_value)
            }
            instruction::LD_Arg_0::Indirect_HLI => {
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
            }
            instruction::LD_Arg_0::Indirect_HLD => {
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
            }
            instruction::LD_Arg_0::Indirect_BC => {
                self.bus.write_byte(self.registers.get_bc(), source_value)
            }
            instruction::LD_Arg_0::Indirect_DE => {
                self.bus.write_byte(self.registers.get_de(), source_value)
            }
            instruction::LD_Arg_0::Indirect_C => self
                .bus
                .write_byte(0xFF00 | self.registers.c as u16, source_value),
            instruction::LD_Arg_0::Indirect_a16 => {
                self.bus.write_byte(self.read_next_word(), source_value)
            }
            _ => panic!("should not reach"),
        };
        self.update_flags(source_value as u16, flags);
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
                self.registers.a = self.update_carry_u8(self.registers.a, value);
                self.update_flags(self.registers.a as u16, flags);
            }
            instruction::ADD_Arg_0::HL => {
                let value = match arg1 {
                    instruction::ADD_Arg_1::BC => self.registers.get_bc(),
                    instruction::ADD_Arg_1::DE => self.registers.get_de(),
                    instruction::ADD_Arg_1::HL => self.registers.get_hl(),
                    instruction::ADD_Arg_1::SP => self.sp,
                    _ => todo!("impl"),
                };
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
                self.registers.f.carry = did_overflow;
                self.registers.f.half_carry =
                    (self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
                self.registers.set_hl(new_value);

                self.update_flags(self.registers.get_hl(), flags)
            }
            instruction::ADD_Arg_0::SP => {
                let value = match arg1 {
                    instruction::ADD_Arg_1::r8 => self.read_next_byte(),
                    _ => todo!("impl"),
                };
                self.sp = self.add_e8(self.sp, value);
                self.update_flags(self.sp, flags);
            }
        }
    }

    fn dec(&mut self, arg0: instruction::DEC_Arg_0, flags: instruction::Flags) {
        match arg0 {
            instruction::DEC_Arg_0::A => {
                let v = self.registers.a.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.a & 0x0F);
                self.registers.a = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::B => {
                let v = self.registers.b.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.b & 0x0F);
                self.registers.b = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::C => {
                let v = self.registers.c.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.c & 0x0F);
                self.registers.c = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::D => {
                let v = self.registers.d.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.d & 0x0F);
                self.registers.d = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::E => {
                let v = self.registers.e.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.e & 0x0F);
                self.registers.e = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::H => {
                let v = self.registers.h.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.h & 0x0F);
                self.registers.h = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::L => {
                let v = self.registers.l.wrapping_sub(1);
                self.registers.f.half_carry = (v & 0x0F) > (self.registers.l & 0x0F);
                self.registers.l = v;
                self.update_flags(v as u16, flags);
            }
            instruction::DEC_Arg_0::Indirect_HL => {
                let v = self.bus.read_byte(self.registers.get_hl());
                let sub = v.wrapping_sub(1);
                self.registers.f.half_carry = (sub & 0x0F) > (v & 0x0F);
                self.bus.write_byte(self.registers.get_hl(), sub);
                self.update_flags(sub as u16, flags);
            }
            instruction::DEC_Arg_0::BC => {
                self.registers
                    .set_bc(self.registers.get_bc().wrapping_sub(1));
            }
            instruction::DEC_Arg_0::DE => {
                self.registers
                    .set_de(self.registers.get_de().wrapping_sub(1));
            }
            instruction::DEC_Arg_0::HL => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
            }
            instruction::DEC_Arg_0::SP => {
                self.sp = self.sp.wrapping_sub(1);
            }
        };
    }
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
    fn cp(&mut self, arg0: instruction::CP_Arg_0, flags: instruction::Flags) {
        let source_value = match arg0 {
            instruction::CP_Arg_0::A => self.registers.a,
            instruction::CP_Arg_0::B => self.registers.b,
            instruction::CP_Arg_0::C => self.registers.c,
            instruction::CP_Arg_0::D => self.registers.d,
            instruction::CP_Arg_0::E => self.registers.e,
            instruction::CP_Arg_0::H => self.registers.h,
            instruction::CP_Arg_0::L => self.registers.l,
            instruction::CP_Arg_0::Indirect_HL => self.bus.read_byte(self.registers.get_hl()),
            instruction::CP_Arg_0::d8 => self.read_next_byte(),
        };
        let sub = self.registers.a.wrapping_sub(source_value);
        self.registers.f.half_carry = (sub & 0x0F) > (self.registers.a & 0x0F);
        self.registers.f.carry = source_value > self.registers.a;
        self.update_flags(sub as u16, flags)
    }
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
    fn and(&mut self, arg0: instruction::AND_Arg_0, flags: instruction::Flags) {
        let source_value = match arg0 {
            instruction::AND_Arg_0::A => self.registers.a,
            instruction::AND_Arg_0::B => self.registers.b,
            instruction::AND_Arg_0::C => self.registers.c,
            instruction::AND_Arg_0::D => self.registers.d,
            instruction::AND_Arg_0::E => self.registers.e,
            instruction::AND_Arg_0::H => self.registers.h,
            instruction::AND_Arg_0::L => self.registers.l,
            instruction::AND_Arg_0::Indirect_HL => self.bus.read_byte(self.registers.get_hl()),
            instruction::AND_Arg_0::d8 => self.read_next_byte(),
        };
        self.registers.a = self.registers.a & source_value;
        self.update_flags(self.registers.a as u16, flags);
    }
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
        let source_value = match arg1 {
            instruction::ADC_Arg_1::A => self.registers.a,
            instruction::ADC_Arg_1::B => self.registers.b,
            instruction::ADC_Arg_1::C => self.registers.c,
            instruction::ADC_Arg_1::D => self.registers.d,
            instruction::ADC_Arg_1::E => self.registers.e,
            instruction::ADC_Arg_1::H => self.registers.h,
            instruction::ADC_Arg_1::L => self.registers.l,
            instruction::ADC_Arg_1::Indirect_HL => self.bus.read_byte(self.registers.get_hl()),
            instruction::ADC_Arg_1::d8 => self.read_next_byte(),
        };

        // TODO A + SOURCE + CARRY の結果を↓
        // TODO ビット演算でcarry & half_carryを計算したほうが良い。

        let carry = if self.registers.f.carry { 1 } else { 0 };
        let source_value = self.update_carry_u8(source_value, carry);

        let c = self.registers.f.carry;
        let hc = self.registers.f.half_carry;

        self.registers.a = self.update_carry_u8(self.registers.a, source_value);

        self.registers.f.carry = self.registers.f.carry | c;
        self.registers.f.half_carry = self.registers.f.half_carry | hc;

        self.update_flags(self.registers.a as u16, flags);
    }
    fn ei(&mut self, flags: instruction::Flags) {}
    fn or(&mut self, arg0: instruction::OR_Arg_0, flags: instruction::Flags) {}
    fn rlc(&mut self, arg0: instruction::RLC_Arg_0, flags: instruction::Flags) {}

    fn add_e8(&mut self, value: u16, add_value: u8) -> u16 {
        let add_value = add_value as i8;
        let add_value = add_value as i32;
        let left = value as i32;
        self.registers.f.carry = (left & 0xFF) + (add_value & 0xFF) > 0xFF;
        self.registers.f.half_carry = (left & 0x0F) + (add_value & 0x0F) > 0x0F;
        let new_value = left.wrapping_add(add_value);
        new_value as u16
    }

    fn update_carry_u8(&mut self, left: u8, right: u8) -> u8 {
        let (new_value, did_overflow) = left.overflowing_add(right);
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (left & 0x0F) + (right & 0x0F) > 0x0F;
        return new_value;
    }

    fn update_flags(&mut self, value: u16, flags: instruction::Flags) {
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
        self.registers.f.half_carry = match flags.half_carry {
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
    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
    fn read_word(&self, address: u16) -> u16 {
        return self.read_byte(address) as u16 | (self.read_byte(address + 1) as u16) << 8;
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
    fn test_add_a_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xC6); // ADD A, d8
        cpu.bus.write_byte(0x0001, 0x05); // d8
        cpu.registers.a = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x06);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x09); // ADD HL, BC
        cpu.registers.set_hl(0x0103);
        cpu.registers.set_bc(0x0204);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0307);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        cpu.bus.write_byte(0x0001, 0x19); // ADD HL, DE
        cpu.registers.set_hl(0x0103);
        cpu.registers.set_de(0x0305);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0408);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));

        cpu.bus.write_byte(0x0002, 0x29); // ADD HL, HL
        cpu.registers.set_hl(0x0104);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0208);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_hl_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x0003;
        cpu.bus.write_byte(0x0000, 0x39); // ADD HL, SP
        cpu.registers.set_hl(0x0106);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0109);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_hl_carry() {
        let mut cpu = CPU::new();
        cpu.sp = 0x0003;
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_hl(0xF000);
        cpu.registers.set_bc(0x1000);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_add_hl_half_carry() {
        let mut cpu = CPU::new();
        cpu.sp = 0x0003;
        cpu.bus.write_byte(0x0000, 0x09);
        cpu.registers.set_hl(0x0F00);
        cpu.registers.set_bc(0x0100);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x1000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_add_sp_r8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8); // ADD SP, r8
        cpu.bus.write_byte(0x0001, 0x04); // r8
        cpu.sp = 0x0003;
        cpu.step();
        assert_eq!(cpu.sp, 0x0007);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_add_sp_r8_minus() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8); // ADD SP, r8
        cpu.bus.write_byte(0x0001, 0xFF); // r8
        cpu.sp = 0x0003;
        cpu.step();
        assert_eq!(cpu.sp, 0x0002);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, true, true));
    }

    #[test]
    fn test_add_sp_r8_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8); // ADD SP, r8
        cpu.bus.write_byte(0x0001, 0x10); // r8
        cpu.sp = 0x00F0;
        cpu.step();
        assert_eq!(cpu.sp, 0x0100);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_add_sp_r8_half_carry() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE8); // ADD SP, r8
        cpu.bus.write_byte(0x0001, 0x01); // r8
        cpu.sp = 0x000F;
        cpu.step();
        assert_eq!(cpu.sp, 0x0010);
        assert_eq!(cpu.pc, 0x0002);
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

    // TODO add tests
    // JP [HL]

    #[test]
    fn test_ld_bc_d16() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x01); // LD BC, d16
        cpu.bus.write_byte(0x0001, 0x04); // args
        cpu.bus.write_byte(0x0002, 0x03); // args
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0304);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_bc_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x02); // LD Indirect_BC, A
        cpu.registers.a = 0x12;
        cpu.registers.set_bc(0x0304);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x0304), 0x12);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x06); // LD B, d8
        cpu.bus.write_byte(0x0001, 0x33); // args
        cpu.step();
        assert_eq!(cpu.registers.b, 0x33);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_a16_sp() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x08); // LD Indirect_a16, SP
        cpu.bus.write_byte(0x0001, 0x23); // args
        cpu.bus.write_byte(0x0002, 0x34); // args
        cpu.sp = 0x5467;
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x3423), 0x67);
        assert_eq!(cpu.bus.read_byte(0x3424), 0x54);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_bc() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0A); // LD A, Indirect_BC
        cpu.bus.write_byte(0x0403, 0x23);
        cpu.registers.set_bc(0x0403);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x23);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0E); // LD C, d8
        cpu.bus.write_byte(0x0001, 0x44); // args
        cpu.step();
        assert_eq!(cpu.registers.c, 0x44);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_de_d16() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x11); // LD DE, d16
        cpu.bus.write_byte(0x0001, 0x12); // args
        cpu.bus.write_byte(0x0002, 0x34); // args
        cpu.step();
        assert_eq!(cpu.registers.get_de(), 0x3412);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_de_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x12); // LD Indirect_DE, A
        cpu.registers.a = 0x05;
        cpu.registers.set_de(0x0987);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x0987), 0x05);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x16); // LD D, d8
        cpu.bus.write_byte(0x0001, 0x50); // args
        cpu.step();
        assert_eq!(cpu.registers.d, 0x50);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_de() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x1A); // LD A, Indirect_DE
        cpu.registers.set_de(0x3467);
        cpu.bus.write_byte(0x3467, 0x77);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x77);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x1E); // LD E, d8
        cpu.bus.write_byte(0x0001, 0x44); // args
        cpu.step();
        assert_eq!(cpu.registers.e, 0x44);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_hl_d16() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x21); // LD HL, d16
        cpu.bus.write_byte(0x0001, 0x45); // args
        cpu.bus.write_byte(0x0002, 0x54); // args
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x5445);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hli_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x22); // LD Indirect_HLI, A
        cpu.registers.a = 0x33;
        cpu.registers.set_hl(0x4455);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x4455), 0x33);
        assert_eq!(cpu.registers.get_hl(), 0x4456);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x26); // LD H, d8
        cpu.bus.write_byte(0x0001, 0x46); // args
        cpu.step();
        assert_eq!(cpu.registers.h, 0x46);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_hli() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x2A); // LD A, Indirect_HLI
        cpu.registers.set_hl(0x4455);
        cpu.bus.write_byte(0x4455, 0x77);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x77);
        assert_eq!(cpu.registers.get_hl(), 0x4456);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x2E); // LD L, d8
        cpu.bus.write_byte(0x0001, 0x67); // args
        cpu.step();
        assert_eq!(cpu.registers.l, 0x67);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_sp_d16() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x31); // LD SP, d16
        cpu.bus.write_byte(0x0001, 0x65); // args
        cpu.bus.write_byte(0x0002, 0x32); // args
        cpu.step();
        assert_eq!(cpu.sp, 0x3265);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hld_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x32); // LD Indirect_HLD, A
        cpu.registers.a = 0x33;
        cpu.registers.set_hl(0x4455);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x4455), 0x33);
        assert_eq!(cpu.registers.get_hl(), 0x4454);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x36); // LD Indirect_HL, d8
        cpu.bus.write_byte(0x0001, 0x38); // args
        cpu.registers.set_hl(0x4455);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x4455), 0x38);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_hld() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x3A); // LD A, Indirect_HLD
        cpu.registers.set_hl(0x4455);
        cpu.bus.write_byte(0x4455, 0x77);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x77);
        assert_eq!(cpu.registers.get_hl(), 0x4454);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x3E); // LD A, d8
        cpu.bus.write_byte(0x0001, 0x90); // args
        cpu.step();
        assert_eq!(cpu.registers.a, 0x90);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x40); // LD B, B
        cpu.registers.b = 0x12;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x12);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x41); // LD B, C
        cpu.registers.c = 0x13;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x13);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x42); // LD B, D
        cpu.registers.d = 0x14;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x14);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x43); // LD B, E
        cpu.registers.e = 0x15;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x15);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x44); // LD B, H
        cpu.registers.h = 0x16;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x16);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x45); // LD B, L
        cpu.registers.l = 0x17;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x17);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x46); // LD B, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_word(0x1040, 0x18);
        cpu.step();
        assert_eq!(cpu.registers.b, 0x18);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_b_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x47); // LD B, A
        cpu.registers.a = 0x19;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x19);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x48); // LD C, B
        cpu.registers.b = 0x20;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x20);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x49); // LD C, C
        cpu.registers.c = 0x21;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x21);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4A); // LD C, D
        cpu.registers.d = 0x22;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x22);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4B); // LD C, E
        cpu.registers.e = 0x23;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x23);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4C); // LD C, H
        cpu.registers.h = 0x24;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x24);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4D); // LD C, L
        cpu.registers.l = 0x25;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x25);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4E); // LD C, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x26);
        cpu.step();
        assert_eq!(cpu.registers.c, 0x26);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x4F); // LD C, A
        cpu.registers.a = 0x27;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x27);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x50); // LD D, B
        cpu.registers.b = 0x28;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x28);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x51); // LD D, C
        cpu.registers.c = 0x29;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x29);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x52); // LD D, D
        cpu.registers.d = 0x2A;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2A);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x53); // LD D, E
        cpu.registers.e = 0x2B;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2B);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x54); // LD D, H
        cpu.registers.h = 0x2C;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2C);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x55); // LD D, L
        cpu.registers.l = 0x2D;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2D);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x56); // LD D, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x2E);
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2E);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_d_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x57); // LD D, A
        cpu.registers.a = 0x2F;
        cpu.step();
        assert_eq!(cpu.registers.d, 0x2F);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x58); // LD E, B
        cpu.registers.b = 0x30;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x30);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x59); // LD E, C
        cpu.registers.c = 0x31;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x31);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5A); // LD E, D
        cpu.registers.d = 0x32;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x32);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5B); // LD E, E
        cpu.registers.e = 0x33;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x33);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5C); // LD E, H
        cpu.registers.h = 0x34;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x34);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5D); // LD E, L
        cpu.registers.l = 0x35;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x35);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5E); // LD E, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x36);
        cpu.step();
        assert_eq!(cpu.registers.e, 0x36);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_e_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x5F); // LD E, A
        cpu.registers.a = 0x37;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x37);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x60); // LD H, B
        cpu.registers.b = 0x38;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x38);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x61); // LD H, C
        cpu.registers.c = 0x39;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x39);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x62); // LD H, D
        cpu.registers.d = 0x3A;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3A);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x63); // LD H, E
        cpu.registers.e = 0x3B;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3B);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x64); // LD H, H
        cpu.registers.h = 0x3C;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3C);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x65); // LD H, L
        cpu.registers.l = 0x3D;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3D);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x66); // LD H, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x3E);
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3E);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_h_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x67); // LD H, A
        cpu.registers.a = 0x3F;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x3F);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x68); // LD L, B
        cpu.registers.b = 0x40;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x40);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x69); // LD L, C
        cpu.registers.c = 0x41;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x41);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6A); // LD L, D
        cpu.registers.d = 0x42;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x42);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6B); // LD L, E
        cpu.registers.e = 0x43;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x43);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6C); // LD L, H
        cpu.registers.h = 0x44;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x44);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6D); // LD L, L
        cpu.registers.l = 0x45;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x45);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6E); // LD L, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x46);
        cpu.step();
        assert_eq!(cpu.registers.l, 0x46);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_l_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x6F); // LD L, A
        cpu.registers.a = 0x47;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x47);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x70); // LD Indirect_HL, B
        cpu.registers.b = 0x48;
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x48);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x71); // LD Indirect_HL, C
        cpu.registers.c = 0x49;
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x49);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x72); // LD Indirect_HL, D
        cpu.registers.d = 0x4A;
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x4A);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x73); // LD Indirect_HL, E
        cpu.registers.e = 0x4B;
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x4B);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x74); // LD Indirect_HL, H
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x75); // LD Indirect_HL, L
        cpu.registers.set_hl(0x104D);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x104D), 0x4D);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_hl_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x77); // LD Indirect_HL, A
        cpu.registers.a = 0x4F;
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1040), 0x4F);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x78); // LD A, B
        cpu.registers.b = 0x50;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x50);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x79); // LD A, C
        cpu.registers.c = 0x51;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x51);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7A); // LD A, D
        cpu.registers.d = 0x52;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x52);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7B); // LD A, E
        cpu.registers.e = 0x53;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x53);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7C); // LD A, H
        cpu.registers.h = 0x54;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x54);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7D); // LD A, L
        cpu.registers.l = 0x55;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x55);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7E); // LD A, Indirect_HL
        cpu.registers.set_hl(0x1040);
        cpu.bus.write_byte(0x1040, 0x56);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x56);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x7F); // LD A, A
        cpu.registers.a = 0x57;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x57);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_c_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE2); // LD Indirect_C, A
        cpu.registers.c = 0x58;
        cpu.registers.a = 0x59;
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0xFF58), 0x59);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_indirect_a16_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xEA); // LD Indirect_a16, A
        cpu.bus.write_byte(0x0001, 0x34); // args
        cpu.bus.write_byte(0x0002, 0x66); // args
        cpu.registers.a = 0x5A;
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x6634), 0x5A);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xF2); // LD A, Indirect_C
        cpu.registers.c = 0x5B;
        cpu.bus.write_byte(0xFF5B, 0x5C);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x5C);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_hl_sp_r8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xF8); // LD HL, SP_r8
        cpu.bus.write_byte(0x0001, 0x67); // args
        cpu.sp = 0x1040;
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0x1040 + 0x67);
        assert_eq!(cpu.sp, 0x1040 + 0x67);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xF9); // LD SP, HL
        cpu.registers.set_hl(0x1040);
        cpu.step();
        assert_eq!(cpu.sp, 0x1040);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_a_indirect_a16() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xFA); // LD A, Indirect_a16
        cpu.bus.write_byte(0x0001, 0x34); // args
        cpu.bus.write_byte(0x0002, 0x55); // args
        cpu.bus.write_byte(0x5534, 0x5D);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x5D);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_ld_hl_sp_r8_minus() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xF8); // LD HL, SP_r8
        cpu.bus.write_byte(0x0001, 0xFF); // args
        cpu.sp = 0x1040;
        cpu.step();
        assert_eq!(cpu.sp, 0x1040 - 1);
        assert_eq!(cpu.registers.get_hl(), 0x1040 - 1);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, true));
    }

    #[test]
    fn test_adc_a_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x88); // ADC A, B
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x20;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x30);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x89); // ADC A, C
        cpu.registers.a = 0x10;
        cpu.registers.c = 0x03;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x13);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8A); // ADC A, D
        cpu.registers.a = 0x10;
        cpu.registers.d = 0x03;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x14);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8B); // ADC A, E
        cpu.registers.a = 0xFF;
        cpu.registers.e = 0x00;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, true, true));
    }

    #[test]
    fn test_adc_a_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8C); // ADC A, H
        cpu.registers.a = 0xFF;
        cpu.registers.h = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, true, true));
    }

    #[test]
    fn test_adc_a_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8D); // ADC A, L
        cpu.registers.a = 0x10;
        cpu.registers.l = 0x06;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x16);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8E); // ADC A, Indirect_HL
        cpu.registers.a = 0x30;
        cpu.registers.set_hl(0x5432);
        cpu.bus.write_byte(0x5432, 0x04);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x34);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x8F); // ADC A, A
        cpu.registers.a = 0x10;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x20);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_adc_a_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xCE); // ADC A, d8
        cpu.bus.write_byte(0x0001, 0x07); // args
        cpu.registers.a = 0x12;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x19);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_and_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA7); // AND A
        cpu.registers.a = 0xF1;
        cpu.step();
        assert_eq!(cpu.registers.a, 0xF1);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA0); // AND B
        cpu.registers.a = 0x07; // 0b0111
        cpu.registers.b = 0x0A; // 0b1010
        cpu.step();
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA1); // AND C
        cpu.registers.a = 0x00;
        cpu.registers.c = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, false, true, false));
    }

    #[test]
    fn test_and_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA2); // AND D
        cpu.registers.a = 0x01;
        cpu.registers.d = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA3); // AND E
        cpu.registers.a = 0x10;
        cpu.registers.e = 0x1F;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA4); // AND H
        cpu.registers.a = 0x10;
        cpu.registers.h = 0x1F;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA5); // AND L
        cpu.registers.a = 0x10;
        cpu.registers.l = 0x1F;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xA6); // AND Indirect_HL
        cpu.registers.a = 0x10;
        cpu.registers.set_hl(0x0032);
        cpu.bus.write_byte(0x0032, 0x1F);
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_and_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xE6); // AND d8
        cpu.bus.write_byte(0x0001, 0x3F); // args
        cpu.registers.a = 0x10;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, false, true, false));
    }

    #[test]
    fn test_cp_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xB8); // CP B
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x01;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, true, false));
    }

    #[test]
    fn test_cp_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xB9); // CP C
        cpu.registers.a = 0x15;
        cpu.registers.c = 0x06;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, true, false));
    }

    #[test]
    fn test_cp_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBA); // CP D
        cpu.registers.a = 0x10;
        cpu.registers.d = 0x11;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, true, true));
    }

    #[test]
    fn test_cp_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBB); // CP E
        cpu.registers.a = 0x01;
        cpu.registers.e = 0x00;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_cp_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBC); // CP H
        cpu.registers.a = 0x01;
        cpu.registers.h = 0x00;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_cp_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBD); // CP L
        cpu.registers.a = 0x01;
        cpu.registers.l = 0x00;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_cp_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBE); // CP Indirect_HL
        cpu.registers.a = 0x05;
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0x03);
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_cp_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xBF); // CP A
        cpu.registers.a = 0x10;
        cpu.step();
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, true, false, false));
    }

    #[test]
    fn test_cp_d8() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0xFE); // CP d8
        cpu.bus.write_byte(0x0001, 0x06); // args
        cpu.registers.a = 0x09;
        cpu.step();
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_b() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x05); // DEC B
        cpu.registers.b = 0x02;
        cpu.step();
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_bc() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0B); // DEC BC
        cpu.registers.set_bc(0x0003);
        cpu.step();
        assert_eq!(cpu.registers.get_bc(), 0x0002);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_dec_c() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x0D); // DEC C
        cpu.registers.c = 0x01;
        cpu.step();
        assert_eq!(cpu.registers.c, 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, true, false, false));
    }

    #[test]
    fn test_dec_d() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x15); // DEC D
        cpu.registers.d = 0x00;
        cpu.step();
        assert_eq!(cpu.registers.d, 0xFF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, true, false));
    }

    #[test]
    fn test_dec_de() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x1B); // DEC DE
        cpu.registers.set_de(0x0001);
        cpu.step();
        assert_eq!(cpu.registers.get_de(), 0x0000);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_dec_e() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x1D); // DEC E
        cpu.registers.e = 0x11;
        cpu.step();
        assert_eq!(cpu.registers.e, 0x10);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_h() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x25); // DEC H
        cpu.registers.h = 0x03;
        cpu.step();
        assert_eq!(cpu.registers.h, 0x02);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x2B); // DEC HL
        cpu.registers.set_hl(0x0000);
        cpu.step();
        assert_eq!(cpu.registers.get_hl(), 0xFFFF);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_dec_l() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x2D); // DEC L
        cpu.registers.l = 0x03;
        cpu.step();
        assert_eq!(cpu.registers.l, 0x02);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_indirect_hl() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x35); // DEC Indirect_HL
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0x02);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1234), 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }

    #[test]
    fn test_dec_indirect_hl_zero() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x35); // DEC Indirect_HL
        cpu.registers.set_hl(0x1234);
        cpu.bus.write_byte(0x1234, 0x01);
        cpu.step();
        assert_eq!(cpu.bus.read_byte(0x1234), 0x00);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(true, true, false, false));
    }

    #[test]
    fn test_dec_sp() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x3B); // DEC SP
        cpu.sp = 0x02;
        cpu.step();
        assert_eq!(cpu.sp, 0x01);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, false, false, false));
    }

    #[test]
    fn test_dec_a() {
        let mut cpu = CPU::new();
        cpu.bus.write_byte(0x0000, 0x3D); // DEC A
        cpu.registers.a = 0x03;
        cpu.step();
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.registers.f, F(false, true, false, false));
    }
}
