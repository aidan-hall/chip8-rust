use chip8_base::*;
use log::{debug, error, info, log_enabled, Level};
use std::time::Duration;

pub struct MyInterpreter {
    pub display: chip8_base::Display,
    pub memory: [u8; 4096],
    pub reg_general: [u8; 16],
    pub reg_sound: u8,
    pub reg_display: u8,
    pub pc: Address,
    pub sp: u8,
    pub clock: Duration,
}

fn nibbles(byte: u8) -> (Nibble, Nibble) {
    (
        Nibble::new(byte >> 4).expect("Fine"),
        Nibble::new(byte & 0b1111).expect("Fine"),
    )
}

pub type Address = u16;
pub struct Nibble(u8);

impl Nibble {
    fn new(val: u8) -> Option<Nibble> {
        if val > 0b1111 {
            None
        } else {
            Some(Self(val))
        }
    }

    fn val(&self) -> u8 {
        self.0
    }
}

#[derive(Debug)]
struct Datum(u16);

impl Datum {
    fn new(val: u16) -> Option<Datum> {
        if val > 0b111111111111 {
            None
        } else {
            Some(Self(val))
        }
    }

    fn from_nibbles(n0: Nibble, n1: Nibble, n2: Nibble) -> Option<Datum> {
        Datum::new(((n0.val() as u16) << 8) + ((n1.val() as u16) << 4) + (n2.val() as u16))
    }

    fn val(&self) -> u16 {
	self.0
    }
}

#[derive(Debug)]
enum Instruction {
    Nop,
    ClearScreen,
    Jump(Datum),
}

impl MyInterpreter {
    pub fn new(clock: Duration) -> Self {
        Self {
            display: [[chip8_base::Pixel::White; 64]; 32],
            memory: [0; 4096],
            reg_general: [0; 16],
            reg_sound: 0,
            reg_display: 0,
            pc: 0,
            sp: 0,
            clock,
        }
    }
    fn fetch(&mut self) -> (u8, u8) {
        let instruction = (
            self.memory[self.pc as usize],
            self.memory[self.pc as usize + 1],
        );
        self.pc += 2;
        info!(
            "Instruction: {}, {}",
            instruction.0, instruction.1
        );
        instruction
    }

    fn decode(&self, bytes: (u8, u8)) -> Instruction {
        let (n0, n1) = nibbles(bytes.0);
        let (n2, n3) = nibbles(bytes.1);

        match n0.val() {
            0 => match (n1.val(), n2.val()) {
                (0, 0xE) => match n3.val() {
                    0 => Instruction::ClearScreen, // CLS
                    0xE => todo!(),                // RET
                    _ => unimplemented!(),
                },
                (0, 0) => match n3.val() {
                    0 => Instruction::Nop,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            1 => Instruction::Jump(Datum::from_nibbles(n1, n2, n3).expect("Fine")),
            _ => unimplemented!(),
        }
    }

    fn execute(&mut self, instruction: &Instruction) -> Option<Display> {
        info!("Executing: {:?}", instruction);
        match instruction {
            Instruction::Nop => {
                info!("No Op");
            }
            Instruction::ClearScreen => {
                info!("Clearing display...");
                self.display = [[Pixel::Black; 64]; 32];
            },
	    Instruction::Jump(d) => {
		self.pc = d.val();
		info!("Jumped to {}", self.pc);
	    }
        }
        Some(self.display)
    }
}

impl Interpreter for MyInterpreter {
    fn step(&mut self, input: &[bool; 16]) -> Option<[[Pixel; 64]; 32]> {
        let bytes = self.fetch();
        let instruction = self.decode(bytes);
        self.execute(&instruction)
    }
    fn speed(&self) -> Duration {
        self.clock
    }
    fn buzzer_active(&self) -> bool {
        false
    }
}
