pub const PC_START: u16 = 0x3000;

pub enum ConditionFlag {
    // We are bit shifting to the left
    // for the value of each flag
    POS = 1 << 0, // 1 Positive
    ZRO = 1 << 1, // 2 Zero
    NEG = 1 << 2, // 4 Negative
}

pub struct Registers {
    pub r0: u16,  // r0-r7 general purpose registers
    pub r1: u16,  
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,   // Program Counter
    pub cond: u16, // Condition flags
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PC_START,
            cond: 0,
        }
    }

    pub fn update(&mut self, index: u16, value: u16) {
        match index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.cond = value,
            _ => panic!("Invalid register index"),
        }
    }

    pub fn get(&self, index: u16) -> u16 {
        match index {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => panic!("Invalid register index"),
        }
    }

    /// Update the condition register based
    /// on the value of the register
    pub fn update_r_cond_register(&mut self, r: u16) {
        if self.get(r) == 0 {
            // register 9 is the condition register
            self.update(9, ConditionFlag::ZRO as u16);
        } else if (self.get(r) >> 15) != 0 {
            // a 1 in the leftmost bit indicates negative
            self.update(9, ConditionFlag::NEG as u16);
        } else {
            self.update(9, ConditionFlag::POS as u16);
        }
    }
}
