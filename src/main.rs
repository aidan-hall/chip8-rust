mod interpreter;
use std::time::*;
fn main() {
    env_logger::init();

    let mut interp = interpreter::MyInterpreter::new(Duration::new(0, 200_000_000));

    let prog = std::fs::read("roms/uwcs.ch8").expect("Should've worked.");



    let prog_len = std::cmp::min(4096, prog.len());
    interp.memory[0x200..prog_len + 0x200].copy_from_slice(&prog[..prog_len]);

    chip8_base::run(interp);
}
