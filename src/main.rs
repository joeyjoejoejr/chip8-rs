use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: chip8-rs rom_file");
        process::exit(1);
    });
    let rom_bin = read_rom(rom_file_name).unwrap_or_else(|err| {
        eprintln!("Error reading rom: {:?}", err);
        process::exit(1);
    });
    println!("{:02x?}{:02x?}", rom_bin[0], rom_bin[1]);
}

fn read_rom(rom_file: String) -> Result<Box<[u8]>, io::Error> {
    let mut file = File::open(rom_file)?;
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)?;
    Ok(file_buffer.into_boxed_slice())
}
