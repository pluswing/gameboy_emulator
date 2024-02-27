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
  fn new() -> Self {
    Registers {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      f: FlagsRegister {
        zero:false,
        subtract: false,
        half_carry: false,
        carry: false
      },
      h: 0,
      l: 0
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

enum Instruction {
    ADD(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),

// ADDHL (HL に追加) - ターゲットが HL レジスタに追加される点を除き、ADD と同様です。
// ADC (キャリー付き加算) - キャリー フラグの値も数値に追加される点を除いて、ADD と同様です。
// SUB (減算) - 特定のレジスタに格納されている値を A レジスタの値と減算します。
// SBC (キャリー付き減算) - キャリー フラグの値も数値から減算される点を除き、ADD と同様です。
// AND (論理積) - 特定のレジスタの値と A レジスタの値に対してビットごとの AND を実行します。
// OR (論理和) - 特定のレジスタの値と A レジスタの値に対してビットごとの OR を実行します。
// XOR (論理 xor) - 特定のレジスタの値と A レジスタの値に対してビット単位の xor を実行します。
// CP (比較) - 減算の結果が A に戻されない点を除けば SUB と同様です。
// INC (インクリメント) - 特定のレジスタの値を 1 ずつインクリメントします。
// DEC (デクリメント) - 特定のレジスタの値を 1 ずつデクリメントします。
// CCF (補数キャリー フラグ) - キャリー フラグの値を切り替えます。
// SCF (キャリー フラグの設定) - キャリー フラグを true に設定します。
// RRA (A レジスターの右回転) - キャリー フラグを通じて A レジスターを右にビット回転します。
// RLA (A レジスタの左回転) - キャリー フラグを通じて A レジスタを左にビット回転します。
// RRCA (A レジスター右回転) - A レジスターを右にビット回転します (キャリー フラグを介さない)
// RRLA (A レジスタの左回転) - A レジスタのビットを左に回転します (キャリー フラグを介さない)
// CPL (補数) - A レジスタの各ビットを切り替えます。
// BIT (ビットテスト) - 特定のレジスタの特定のビットが設定されているかどうかを確認するテスト
// RESET (ビットリセット) - 特定のレジスタの特定のビットを0に設定します。
// SET (ビットセット) - 特定のレジスタの特定のビットを 1 に設定します。
// SRL (論理右シフト) - 特定のレジスタを右に 1 ビットシフトします。
// RR (右回転) - キャリー フラグを使用して特定のレジスタを右に 1 ビット回転します。
// RL (左回転) - キャリー フラグを使用して特定のレジスタを 1 だけ左にビット回転します。
// RRC (右回転) - 特定のレジスタを 1 だけ右にビット回転します (キャリー フラグを介さない)
// RLC (左回転) - 特定のレジスタを 1 だけ左にビット回転します (キャリー フラグを介さない)
// SRA (右シフト算術) - 特定のレジスタを右に 1 算術シフトします。
// SLA (シフト左算術) - 特定のレジスタを左に 1 算術シフトします。
// SWAP (スワップニブル) - 特定のレジスタの上位ニブルと下位ニブルを切り替えます

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

enum StackTarget {
    BC,
    // FIXME
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
    sp: u16,
    bus: MemoryBus,
}

impl CPU {
    fn new() -> Self {
      CPU {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        bus: MemoryBus { memory: [] }
      }
    }

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
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
            },
            Instruction::PUSH(target) => {
              let value = match target {
                StackTarget::BC => self.registers.get_bc(),
              }
              self.push(value);
              self.pc.wrapping_add(1)
            },
            Instruction::POP(target) => {
              let result = self.pop();
              match target {
                StackTarget::BC => self.registers.set_bc(result),
              }
              self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
              let jump_condition = match test {
                JumpTest::NotZero => !self.registers.zero,
                _ => { panic!("TODO: support more condition")}
              };
              self.call(jump_condition)
            }
            Instruction::RET(test) => {
              let jump_condition = match test {
                JumpTest::NotZero => !self.registers.zero,
                _ => { panic!("TODO: support more condition")}
              };
              self.return_(jump_condition)
            }
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

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
      let l = self.bus.read_byte(self.pc + 1) as u16;
      let u = self.bus.read_byte(self.pc + 2) as u16;
      return (u << 8) | l
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn new() -> Self {
      MemoryBus {
        memory: // [0xFFFF; 0]
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
    fn test_add_c() {
      let cpu = CPU::new();
      cpu.registers.a = 0x0F;
      // assert_eq!(cpu.register_a, 0x05);
    }
}
