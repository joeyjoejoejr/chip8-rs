#[derive(Debug, Default)]
pub struct CPU {
  vx: [u8; 16],
  i: u16,
}

impl CPU {
  pub fn load_vx(&mut self, index: usize, byte: u8) {
    self.vx[index] = byte;
  }

  pub fn load_i(&mut self, nnn: u16) {
    self.i = nnn;
  }
}