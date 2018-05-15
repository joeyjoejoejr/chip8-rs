#[derive(Debug)]
pub struct Chip8 {
  pc: usize,
  vx: [u8; 16],
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
       pc: 0,
       vx: [0; 16],
    }
  }

  pub fn run(&mut self, rom_bin: Box<[u8]>) {
    loop {
      let instruction = (rom_bin[self.pc] as u16) << 8 | (rom_bin[self.pc + 1] as u16);
      println!("Instruction: {:04x?}", instruction);
      println!("{:?}", self);
      let opcode = instruction & 0xF000;

      match opcode {
        0x6000 => {
          // LD Vx, byte
          let x = ((instruction & 0x0F00) >> 8) as usize;
          let byte = (instruction & 0x00FF) as u8;
          self.vx[x] = byte;
        },
        _ => panic!("Unknown opcode: {:x?}", opcode),
      }

      self.pc += 2;
    }
  }
}
