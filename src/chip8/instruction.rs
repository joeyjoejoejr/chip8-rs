use std::fmt;

pub struct Instruction {
  word: u16,
}

pub const O_CODE: u16 = 0x0000;
pub const RET: u16 = 0x00EE;

pub const CALL: u16 = 0x2000;
pub const LD_VX: u16 = 0x6000;
pub const ADD_VX: u16 = 0x7000;
pub const LD_I: u16 = 0xA000;
pub const DRW: u16 = 0xD000;

pub const F_CODE: u16 = 0xF000;
pub const LD_F: u16 = 0x0029;
pub const LD_B: u16 = 0x0033;
pub const LD_VX_I: u16 = 0x0065;


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

  pub fn o_code(& self) -> u16 {
    self.f_code()
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
