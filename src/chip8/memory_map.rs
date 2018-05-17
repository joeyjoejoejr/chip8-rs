use std::fmt;

#[derive(Default)]
pub struct MemoryMap {
    rom: Box<[u8]>
}

impl MemoryMap {
    pub fn set_rom(&mut self, rom: Box<[u8]>) {
        self.rom = rom;
    }

    pub fn read_word(& self, addr: usize) -> [u8; 2] {
        [self.read_byte(addr), self.read_byte(addr +1)]
    }

    pub fn read_byte(& self, addr: usize) -> u8 {
        match addr {
            0x0200 ... 0x0FFF => self.rom[addr - 0x200],
            _ => panic!("Unknown address: {:x?}", addr),
        }
    }
}

impl fmt::Debug for MemoryMap {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory Map")
    }
}