mod cpu;
mod memory_map;
mod instruction;

use super::screen::Screen;
use self::cpu::CPU;
use self::memory_map::MemoryMap;
use self::instruction::Instruction;

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
      self.screen.clear_screen();
      self.screen.print_screen();

      let word = self.memory_map.read_word(self.pc);
      let instruction = Instruction::new(word);

      match instruction.opcode() {
        instruction::O_CODE => {
          match instruction.o_code() {
            instruction::RET => {
              self.sp -= 1;
              let ret_addr = self.stack[self.sp];
              self.stack[self.sp] = 0;
              self.pc = ret_addr as usize;
            },
            _ => panic!("Unknown instruction: {:?}", instruction),
          }
        }
        instruction::CALL => {
          let nnn = instruction.nnn();
          self.stack[self.sp] = self.pc as u16;
          self.sp += 1;
          self.pc = nnn as usize;
        },
        instruction::LD_VX => {
          let x = instruction.vx() as usize;
          let byte = instruction.nn();
          self.cpu.load_vx(x, byte);
        },
        instruction::ADD_VX => {
          let vx = instruction.vx() as usize;
          let x = self.cpu.get_vx(vx);
          let res = x + instruction.nn();
          self.cpu.load_vx(vx, res);
        },
        instruction::LD_I => {
          let nnn = instruction.nnn();
          self.cpu.load_i(nnn);
        },
        instruction::DRW => {
          let vx = instruction.vx() as usize;
          let x = self.cpu.get_vx(vx) as usize;

          let vy = instruction.vy() as usize;
          let y = self.cpu.get_vx(vy) as usize;

          let num_bytes = instruction.n() as usize;
          let mut sprite: Vec<u8> = Vec::new();
          for i in 0 .. num_bytes {
            sprite.push(
              self.memory_map.read_byte(self.cpu.get_i() + i)
            );
          }
          let collision = self.screen.draw_sprite(x, y, &sprite);
          self.cpu.load_vx(0xF, collision as u8)
        },
        instruction::F_CODE => {
          match instruction.f_code() {
            instruction::LD_F => {
              let vx = instruction.vx() as usize;
              let x =  self.cpu.get_vx(vx);
              self.cpu.load_i(x as u16 * 5)
            },
            instruction::LD_B => {
              let vx = instruction.vx() as usize;
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
            instruction::LD_VX_I => {
              let vx = instruction.vx() as usize;
              let i = self.cpu.get_i();

              for j in 0..vx {
                let byte  = self.memory_map.read_byte(i + j);
                self.cpu.load_vx(j, byte);
              }
            }
            _ => panic!("Unknown instruction: {:?}", instruction),
          }
        }
        _ => panic!("Unknown instruction: {:?}", instruction),
      }

      self.pc += 2;
    }
  }
}
