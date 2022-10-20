mod interpreter;
use std::time::*;
fn main() {
    let interp = interpreter::MyInterpreter::new(Duration::new(0, 200_000_000));
    chip8_base::run(interp);
}
