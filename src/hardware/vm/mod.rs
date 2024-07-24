use crate::hardware::register::*;
use crate::MEMORY_SIZE;
use std::io::Read;

/// The memory is just a bit array of 16-bit unsigned integers.
pub struct VM  {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

pub enum MemoryMappedReg {
    Kbsr = 0xFE00,    // Keyboard status
    Kbdr = 0xFE02,    // Keyboard data
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }
    
    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn read_memory(&mut self, address: u16) -> u16 {
        if (address == MemoryMappedReg::Kbsr as u16) {
            self.handle_keyboard();
        }
        self.memory[address as usize]
    }

    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write_memory(MemoryMappedReg::Kbsr as usize, 1 << 15);
            self.write_memory(MemoryMappedReg::Kbdr as usize, buffer[0] as u16);
        } else {
           self.write_memory(MemoryMappedReg::Kbsr as usize, 0)
        }
    }
}
