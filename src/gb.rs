use std::{
    fs::File,
    io::{self, Read},
};
struct Emulator {
    rom: Vec<u8>
}
impl Emulator {
    fn new() -> Self {
        Emulator { rom: vec![] }
    }
    fn nextOp() -> i32 {
        todo!()
    }
}
pub fn emulate(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut emulator = Emulator::new();
    let size = file.read_to_end(&mut emulator.rom)?;
    Ok(())
}
