use core::hash;

fn main() {
    println!("Hello, world!");
}


struct Registers {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
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

impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | (self.c as u16)
  }
  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0x00FF) as u8;
  }
}
