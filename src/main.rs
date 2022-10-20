mod interpreter;
use std::time::*;
fn main() {
    env_logger::init();

    let mut interp = interpreter::MyInterpreter::new(Duration::new(0, 200_000_000));

    interp.memory[5] = 0xE0;

    interp.memory[6] = 0x10;
    chip8_base::run(interp);
}
