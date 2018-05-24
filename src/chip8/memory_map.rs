use std::fmt;

const SPRITES: [u8; 80] = [
    0b11110000, // 0
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,

    0b00100000, // 1
    0b01100000,
    0b00100000,
    0b00100000,
    0b00100000,

    0b11110000, // 2
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,

    0b11110000, // 3
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,

    0b10010000, // 4
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,

    0b11110000,
    0b10000000, // 5
    0b11110000,
    0b00010000,
    0b11110000,

    0b11110000, //6
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,

    0b11110000, //7
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,


    0b11110000, //8
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,

    0b11110000, //9
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,

    0b11110000, //A
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,

    0b11100000, //B
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,

    0b11110000, //C
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,

    0b11100000, //D
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,

    0b11110000, //E
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,

    0b11110000, //F
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];

pub struct MemoryMap {
    ram: Box<[u8; 0x1000]>
}

impl MemoryMap {
    pub fn set_rom(&mut self, rom: Box<[u8]>) {
        rom.iter().enumerate()
            .for_each(|(i, &val)| {
                self.ram[i + 0x200] = val;
            });
    }

    pub fn read_word(& self, addr: usize) -> [u8; 2] {
        [self.read_byte(addr), self.read_byte(addr +1)]
    }

    pub fn load_byte(&mut self, addr: usize, byte: u8) {
        self.ram[addr] = byte;
    }

    pub fn read_byte(& self, addr: usize) -> u8 {
        match addr {
            0x0200 ... 0x0FFF => self.ram[addr],
            _ => panic!("Unknown address: {:x?}", addr),
        }
    }
}

impl fmt::Debug for MemoryMap {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory Map - ")
        // for (i, &byte) in self.ram.iter().enumerate() {
        //     write!(f, "{}: {} ", i, byte)?;
        // }
        // write!(f, "")
    }
}

impl Default for MemoryMap {
    fn default() -> MemoryMap {
        let mut ram  = Box::new([0u8; 0x1000]);

        SPRITES.iter().enumerate().for_each(|(i, &byte)| {
            ram[i] = byte;
        });

        MemoryMap { ram: ram }
    }
}
