use chip8_base::*;
use std::time::Duration;

pub struct MyInterpreter {
    display: chip8_base::Display,
    memory: [u8; 4096],
    reg_general : [u8; 16],
    reg_sound: u8,
    reg_display: u8,
    pc: u16,
    sp: u8,
    clock: Duration,
}

impl MyInterpreter {
    pub fn new(clock: Duration) -> Self {
	Self {
	    display: [[chip8_base::Pixel::Black; 64]; 32],
	    memory: [0; 4096],
	    reg_general: [0; 16],
	    reg_sound: 0,
	    reg_display: 0,
	    pc: 0,
	    sp: 0,
	    clock,
	}
    }
}

impl Interpreter for MyInterpreter {
    fn step(&mut self, _: &[bool; 16]) -> Option<[[Pixel; 64]; 32]> { todo!() }
    fn speed(&self) -> Duration {  self.clock }
    fn buzzer_active(&self) -> bool { todo!() }
}
