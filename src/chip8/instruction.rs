use std::fmt;

pub struct Instruction {
  word: u16,
}

pub const CALL: u16 = 0x2000;
pub const LD_VX: u16 = 0x6000;

impl Instruction {
  pub fn new(word: [u8; 2]) -> Instruction {
    Instruction {
      word: (word[0] as u16) << 8 | word[1] as u16,
    }
  }

  pub fn opcode(& self) -> u16 {
    self.word & 0xF000
  }

  pub fn f_code(& self) -> u16 {
    self.word & 0x00FF
  }

  pub fn n(& self) -> u16 {
    self.word & 0x000F
  }

  pub fn nn(& self) -> u8 {
    (self.word & 0x00FF) as u8
  }

  pub fn nnn(& self) -> u16 {
    self.word & 0x0FFF
  }

  pub fn vx(& self) -> u16 {
    (self.word & 0x0F00) >> 8
  }

  pub fn vy(& self) -> u16 {
    (self.word & 0x00F0) >> 4
  }
}

impl fmt::Debug for Instruction {
  fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Instruction: 0x{:04X?}", self.word)
  }
}
