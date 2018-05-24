mod cpu;
mod memory_map;
mod opcode;

use super::screen::Screen;
use self::cpu::CPU;
use self::memory_map::MemoryMap;

#[derive(Debug)]
pub struct Chip8 {
  cpu: CPU,
  screen: Screen,
  memory_map: MemoryMap,
  pc: usize,
  sp: usize,
  stack: [u16; 16],
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
       cpu: CPU::default(),
       screen: Screen::default(),
       memory_map: MemoryMap::default(),
       pc: 0x0200,
       sp: 0,
       stack: [0; 16]
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
        0x2000 => {
          // CALL addr
          let nnn = instruction & 0x0FFF;
          self.stack[self.sp] = self.pc as u16;
          self.sp += 1;
          self.pc = nnn as usize;
        },
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
        },
        0xF000 => {
          match instruction & 0x00FF {
            0x0029 => {
              // LD F, Vx
              let vx = ((instruction & 0x0F00) >> 8) as usize;
              let x =  self.cpu.get_vx(vx);
              self.cpu.load_i(x as u16 * 5)
            },
            0x0033 => {
              // LD B, Vx
              let vx = ((instruction & 0x0F00) >> 8) as usize;
              let x =  self.cpu.get_vx(vx);
              let i = self.cpu.get_i();
              println!("Vx is {}", vx);
              println!("Loading {} into memory location {}", x, i);

              let hundreds = x / 100;
              self.memory_map.load_byte(i, hundreds);
              let remainder = x % 100;
              let tens = remainder / 10;
              self.memory_map.load_byte(i + 1, tens);
              let ones = remainder % 10;
              self.memory_map.load_byte(i + 1, ones);
            },
            0x0065 => {
              // LD Vx, [I]
              let vx = ((instruction & 0x0F00) >> 8) as usize;
              let i = self.cpu.get_i();

              for j in 0..vx {
                let byte  = self.memory_map.read_byte(i + j);
                self.cpu.load_vx(j, byte);
              }
            }
            _ => panic!("Unknown instruction: {:x?}", instruction),
          }
        }
        _ => panic!("Unknown instruction: {:x?}", instruction),
      }

      self.pc += 2;
    }
  }
}
