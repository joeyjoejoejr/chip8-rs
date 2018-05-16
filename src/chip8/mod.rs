mod cpu;
use super::screen;

#[derive(Debug)]
pub struct Chip8 {
  pc: usize,
  cpu: cpu::CPU,
  screen: screen::Screen,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
       pc: 0,
       cpu: cpu::CPU::default(),
       screen: screen::Screen::default(),
    }
  }

  pub fn run(&mut self, rom_bin: Box<[u8]>) {
    loop {
      let instruction = (rom_bin[self.pc] as u16) << 8 | (rom_bin[self.pc + 1] as u16);
      println!("Instruction: {:04x?}", instruction);
      println!("{:#?}", self);
      let opcode = instruction & 0xF000;

      match opcode {
        0x6000 => {
          // LD Vx, byte
          let x = ((instruction & 0x0F00) >> 8) as usize;
          let byte = (instruction & 0x00FF) as u8;
          self.cpu.load_vx(x, byte);
        },
        0xA000 => {
          // LD I, byte
          let nnn = instruction & 0x0FFF;
          self.cpu.load_i(nnn);
        },
        _ => panic!("Unknown opcode: {:x?}", opcode),
      }

      self.pc += 2;
    }
  }
}
