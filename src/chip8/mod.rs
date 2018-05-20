mod cpu;
mod memory_map;
use super::screen::Screen;
use self::cpu::CPU;
use self::memory_map::MemoryMap;

#[derive(Debug)]
pub struct Chip8 {
  pc: usize,
  cpu: CPU,
  screen: Screen,
  memory_map: MemoryMap,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
       pc: 0x0200,
       cpu: CPU::default(),
       screen: Screen::default(),
       memory_map: MemoryMap::default(),
    }
  }

  pub fn run(&mut self, rom_bin: Box<[u8]>) {
    self.memory_map.set_rom(rom_bin);
    loop {
      let word = self.memory_map.read_word(self.pc);
      let instruction = (word[0] as u16) << 8 | word[1] as u16;
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
        0xD000 => {
          // DRW Vx Vy nibble
          let vx = ((instruction & 0x0F00) >> 8) as usize;
          let x = self.cpu.get_vx(vx) as usize;

          let vy = ((instruction & 0x00F0) >> 4) as usize;
          let y = self.cpu.get_vx(vy) as usize;

          let num_bytes = (instruction & 0x000F) as usize;
          let mut sprite: Vec<u8> = Vec::new();
          for i in 0 .. num_bytes {
            sprite.push(
              self.memory_map.read_byte(self.cpu.get_i() + i)
            );
          }
          let collision = self.screen.draw_sprite(x, y, &sprite);
          self.cpu.load_vx(0xF, collision as u8)
        }
        _ => panic!("Unknown opcode: {:x?}", opcode),
      }

      self.pc += 2;
    }
  }
}
