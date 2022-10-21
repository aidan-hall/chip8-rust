use chip8_base::*;
use log::{debug, error, info, log_enabled, Level};
use std::time::Duration;

pub struct MyInterpreter {
    pub display: chip8_base::Display,
    pub memory: [u8; 4096],
    pub reg_general: [u8; 16],
    pub reg_sound: u8,
    pub reg_display: u8,
    pub index: u16,
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
#[derive(Debug)]
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

    fn byte(hi: Nibble, lo: Nibble) -> u8 {
        (hi.val() << 4) + lo.val()
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

fn bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

#[derive(Debug)]
enum Instruction {
    Nop,
    ClearScreen,
    Jump(Datum),
    Load(Address, u8),
    Add(Address, u8),
    SetIndex(Datum),
    Draw(Nibble, Nibble, Nibble),
}

impl MyInterpreter {
    pub fn new(clock: Duration) -> Self {
        Self {
            display: [[chip8_base::Pixel::Black; 64]; 32],
            memory: [0; 4096],
            reg_general: [0; 16],
            reg_sound: 0,
            reg_display: 0,
            index: 0,
            pc: 0x200,
            sp: 0,
            clock,
        }
    }
    fn fetch(&mut self) -> (u8, u8) {
        info!("PC: {}", self.pc);
        let instruction = (
            self.memory[self.pc as usize],
            self.memory[self.pc as usize + 1],
        );
        info!("Instruction: {:#02X} {:#02X}", instruction.0, instruction.1);
        self.pc += 2;
        instruction
    }

    fn decode(&self, bytes: (u8, u8)) -> Instruction {
        let (n0, n1) = nibbles(bytes.0);
        let (n2, n3) = nibbles(bytes.1);

        use Instruction::*;
        match n0.val() {
            0 => match (n1.val(), n2.val()) {
                (0, 0xE) => match n3.val() {
                    0 => ClearScreen, // CLS
                    0xE => todo!(),   // RET
                    _ => unimplemented!(),
                },
                (0, 0) => match n3.val() {
                    0 => Nop,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            1 => Jump(Datum::from_nibbles(n1, n2, n3).expect("Fine")),
            6 => Load(n1.val() as Address, bytes.1),
            7 => Add(n1.val() as Address, bytes.1),
            0xA => SetIndex(Datum::from_nibbles(n1, n2, n3).expect("Fine")),
            0xD => Draw(n1, n2, n3),
            _ => unimplemented!(),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Option<Display> {
        info!("Executing: {:?}", instruction);
        use Instruction::*;
        match instruction {
            Nop => {}
            ClearScreen => {
                self.display = [[Pixel::Black; 64]; 32];
            }
            Jump(d) => {
                self.pc = d.val();
            }
            Load(reg, val) => {
                self.reg_general[reg as usize] = val;
            }
            Add(reg, val) => {
                self.reg_general[reg as usize] = self.reg_general[reg as usize].wrapping_add(val);
            }
            SetIndex(val) => {
                self.index = val.val();
            }
            Draw(x, y, n) => {
                let x = self.reg_general[x.val() as usize];
                let y = self.reg_general[y.val() as usize];
                info!("Index: 0x{:04X}, x: {}, y: {}", self.index, x, y);
                let last_y = y + n.val() / 8;
                self.reg_general[0xF] = 0;

                for i in 0..n.val() {
                    let row = self.memory[(self.index as usize) + (i as usize)];
                    info!("Drawing row: {:08b}", row);
                    for j in 0..8 {
                        let y_coord = ((y + i) % 32) as usize;
                        let x_coord = ((x + j) % 64) as usize;
                        let px = self.display[y_coord][x_coord];
                        let bit_px = if bit_at(row, 7 - j) {
                            Pixel::White
                        } else {
                            Pixel::Black
                        };
                        let res = px ^ bit_px;
                        if res == Pixel::Black {
                            self.reg_general[0xF] = 1;
                        }
                        info!("Pixel[{}][{}] = {:?}", y_coord, x_coord, res);
                        self.display[y_coord][x_coord] = res;
                    }
                }
            }
        }
        Some(self.display)
    }
}

impl Interpreter for MyInterpreter {
    fn step(&mut self, input: &[bool; 16]) -> Option<[[Pixel; 64]; 32]> {
        let bytes = self.fetch();
        let instruction = self.decode(bytes);
        self.execute(instruction)
    }
    fn speed(&self) -> Duration {
        self.clock
    }
    fn buzzer_active(&self) -> bool {
        false
    }
}
