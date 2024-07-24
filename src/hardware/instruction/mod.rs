use crate::hardware::vm::*;
use std::process;
use std::io::{self, Write, Read};

pub enum OpCode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    ST,     // store
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

pub fn get_op_code(instr: &u16) -> Option<OpCode> {
    let op_code = (instr >> 12);
    match op_code {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None,
    }
}

pub fn execute_instruction(instr: u16, vm: &mut VM) {
    let op_code = get_op_code(&instr);

    match op_code {
        Some(OpCode::ADD)  => add(instr, vm),
        Some(OpCode::AND)  => and(instr, vm),
        Some(OpCode::NOT)  => not(instr, vm),
        Some(OpCode::BR)   => br(instr, vm),
        Some(OpCode::JMP)  => jmp(instr, vm),
        Some(OpCode::RTI)  => println!("RTI is not implemented"),
        Some(OpCode::JSR)  => jsr(instr, vm),
        Some(OpCode::LD)   => ld(instr, vm),
        Some(OpCode::LDI)  => ldi(instr, vm),
        Some(OpCode::LDR)  => ldr(instr, vm),
        Some(OpCode::LEA)  => lea(instr, vm),
        Some(OpCode::ST)   => st(instr, vm),
        Some(OpCode::STI)  => sti(instr, vm),
        Some(OpCode::STR)  => str(instr, vm),
        Some(OpCode::TRAP) => trap(instr, vm),
        _ => println!("Invalid instruction"),
    }
}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    // If positive
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}

/// TRAP
/// OPCODE u4, 0 u4 trapvect8 u8
/// First R7 is loaded with the incremented PC.
/// (This enables a return to the instruction
/// physically following the TRAP instruction in
/// the original program after the service routine
/// has completed execution.) Then the PC is loaded
/// with the starting address of the system call
/// specified by trapvector8. The starting address is
/// contained in the memory location whose address
/// is obtained by zero-extending trapvector8 to 16 bits.
fn trap(instr: u16, vm: &mut VM) {
    vm.registers.update(7, vm.registers.pc);
    let trap_vector = instr & 0xFF;

    match trap_vector {
        0x20 => {
            /// GETC
            /// Read a single character from the keyboard.
            /// The character is not echoed onto the console.
            /// Its ASCII code is copied into R0. The high
            /// eight bits of R0 are cleared.
            let mut buffer = [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            vm.registers.r0 = buffer[0] as u16;
        }
        0x21 => {
            /// OUT
            /// Write a character in R0[7:0] to the console
            /// display.
            print!("{}", (vm.registers.get(0) & 0xFF) as u8 as char);
        }
        0x22 => {
            /// PUTS
            /// Write a string of ASCII characters to the 
            /// console display. The characters are 
            /// contained in consecutive memory locations, 
            /// one character per memory location, starting
            /// with the address specified in R0. Writing 
            /// terminates with the occurrence of x0000 in
            /// a memory location.
            let mut index = vm.registers.get(0);
            loop {
                let c = vm.read_memory(index);
                index += 1;
                if c == 0x0000 {
                    break;
                }
                print!("{}", (c as u8) as char);
            }
            io::stdout().flush().expect("Failed to flush");
        }
        0x23 => {
            /// IN
            /// Print a prompt on the screen and read a
            /// single character from the keyboard.
            /// The character is echoed onto the console
            /// monitor, and its ASCII code is copied
            /// into R0. The high eight bits of R0 are
            /// cleared
            print!("Enter a  character : ");
            io::stdout().flush().expect("failed to flush");
            let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();
            vm.registers.update(0, char);
        }
        0x24 => {
            /// PUTSP
            /// Write a string of ASCII characters to the 
            /// console. The characters are contained in
            /// consecutive memory locations, two characters
            /// per memory location, starting with the
            /// address specified in R0. The ASCII code
            /// contained in bits [7:0] of a memory
            /// location is written to the console first.
            /// Then the ASCII code contained in bits
            /// [15:8] of that memory location is written
            /// to the console. (A character string
            /// consisting of an odd number of characters
            /// to be written will have x00 in bits
            /// [15:8] of the memory location containing
            /// the last character to be written.) Writing
            /// terminates with the occurrence of x0000 in
            /// a memory location.
            let mut index = vm.registers.r0;
            let mut c = vm.read_memory(index);
            while c != 0x0000 {
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}", c1);
                let c2 = ((c >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                c = vm.read_memory(index);
            }
            io::stdout().flush().expect("Failed to flush");
        }
        0x25 => {
            /// HALT
            /// Halt execution and print a message on
            /// the console.
            println!("HALT detected");
            io::stdout().flush().expect("Failed to flush");
            process::exit(0);
        }
        _ => {
            println!("Invalid trap vector");
            process::exit(1);
        }
    }
}

/// LEA
/// OPCODE u4, DR u3, PCOffset9 u9
/// An address is computed by sign-extending bits
/// [8:0] to 16 bits and adding this value to the
/// incremented PC. This address is loaded into DR.
/// The condition codes are set, based on whether
/// the value loaded is negative, zero, or positive.
fn lea(instr: u16, vm: &mut VM) {
    let mut dr = (instr >> 9) & 0x7;
    let mut pc_offset = sign_extend(instr & 0x1FF, 9);
    pc_offset += vm.registers.pc;
    vm.registers.update(dr, pc_offset);
    vm.registers.update_r_cond_register(dr);
}


/// ADD
/// If bit [5] is 0, the second source operand is
/// obtained from SR2. If bit [5] is 1, the second
/// source operand is obtained by sign-extending
/// the imm5 field to 16 bits. In both cases, the
/// second source operand is added to the contents
/// of SR1 and the result stored in DR. The
/// condition codes are set, based on whether the
/// result is negative, zero, or positive
fn add(instr: u16, vm: &mut VM) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);

        // Need to cast to u32 to avoid overflow
        let val: u32 = imm5 as u32 + vm.registers.get(sr1) as u32;
        vm.registers.update(dr, val as u16);
    } else {
        let sr2 = instr & 0x7;

        let val: u32 = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32;
        vm.registers.update(dr, val as u16);
    }
    vm.registers.update_r_cond_register(dr);
}

/// LDI
/// An address is computed by sign-extending bits 
/// [8:0] to 16 bits and adding this value to the 
/// incremented PC. What is stored in memory at 
/// this address is the address of the data to 
/// be loaded into DR. The condition codes are 
/// set, based on whether the value loaded is 
/// negative, zero, or positive.
pub fn ldi(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1ff, 9);
    let first_read = vm.read_memory(vm.registers.pc + pc_offset);
    let resulting_address = vm.read_memory(first_read);
    vm.registers.update(dr, resulting_address);
    vm.registers.update_r_cond_register(dr);
}

/// AND
/// If bit [5] is 0, the second source operand is 
/// obtained from SR2. If bit [5] is 1, the second
/// source operand is obtained by sign-extending
/// the imm5 field to 16 bits. In either case, the
/// second source operand and the contents of SR1
/// are bit- wise ANDed, and the result stored in DR.
/// The condition codes are set, based on whether
/// the binary value produced, taken as a 2’s 
/// complement integer, is negative, zero, or positive.
pub fn and(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.registers.update(dr, vm.registers.get(sr1) & imm5);
    } else {
        let sr2 = instruction & 0x7;
        vm.registers
            .update(dr, vm.registers.get(sr1) & vm.registers.get(sr2));
    }

    vm.registers.update_r_cond_register(dr);
}

/// NOT
/// The bit-wise complement of the contents of SR is 
/// stored in DR. The condi- tion codes are set, 
/// based on whether the binary value produced, 
/// taken as a 2’s complement integer, is negative,
/// zero, or positive.
pub fn not(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    vm.registers.update(dr, !vm.registers.get(sr1));

    vm.registers.update_r_cond_register(dr);
}

/// BR
/// The condition codes specified by the state of 
/// bits [11:9] are tested. If bit [11] is set, N is 
/// tested; if bit [11] is clear, N is not tested.
/// If bit [10] is set, Z is tested. If any of the 
/// condition codes tested is set, the program 
/// branches to the location specified by adding 
/// the sign-extended PCOffset9 field to the 
/// incremented PC.
pub fn br(instruction: u16, vm: &mut VM) {
    let pc_offset = sign_extend((instruction) & 0x1ff, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    if cond_flag & vm.registers.cond != 0 {
        let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
        vm.registers.pc = val as u16;
    }
}

/// The program unconditionally jumps to the location 
/// specified by the contents of the base
/// register. Bits [8:6] identify the base register. 
/// The RET instruction is a special case of the
/// JMP instruction. The PC is loaded with the contents 
/// of R7, which contains the linkage back to
/// the instruction following the subroutine call 
/// instruction.
pub fn jmp(instruction: u16, vm: &mut VM) {
    let base_reg = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.registers.get(base_reg);
}

/// JSR
/// First, the incremented PC is saved in R7. This
/// is the linkage back to the calling routine.
/// Then the PC is loaded with the address of the 
/// first instruction of the subroutine, causing an
/// unconditional jump to that address. The address 
/// of the subroutine is obtained from the base
/// register (if bit [11] is 0), or the address is 
/// computed by sign-extending bits [10:0] and
/// adding this value to the incremented PC (if bit 
/// [11] is 1).
pub fn jsr(instruction: u16, vm: &mut VM) {
    let base_reg = (instruction >> 6) & 0x7;
    let long_pc_offset = sign_extend(instruction & 0x7ff, 11);

    let long_flag = (instruction >> 11) & 1;

    vm.registers.r7 = vm.registers.pc;

    if long_flag != 0 {
        let val: u32 = vm.registers.pc as u32 + long_pc_offset as u32;
        vm.registers.pc = val as u16;
    } else {
        vm.registers.pc = vm.registers.get(base_reg);
    }
}

/// LD
/// An address is computed by sign-extending bits 
/// [8:0] to 16 bits and adding this value to the
/// incremented PC. The contents of memory at this 
/// address are loaded into DR. The condition codes
/// are set, based on whether the value loaded is
/// negative, zero, or positive.
pub fn ld(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    let mem: u32 = pc_offset as u32 + vm.registers.pc as u32;

    let value = vm.read_memory(mem as u16);

    vm.registers.update(dr, value);
    vm.registers.update_r_cond_register(dr);
}

/// LDR
/// An address is computed by sign-extending bits 
/// [5:0] to 16 bits and adding this value to the 
/// contents of the register specified by bits [8:6].
/// The contents of memory at this address are loaded 
/// into DR. The condition codes are set, based 
/// on whether the value loaded is negative, zero,
/// or positive.
pub fn ldr(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let base_reg = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);
    let val: u32 = vm.registers.get(base_reg) as u32 + offset as u32;
    let mem_value = vm.read_memory(val as u16).clone();

    vm.registers.update(dr, mem_value);
    vm.registers.update_r_cond_register(dr);
}

/// ST
/// The contents of the register specified by SR 
/// are stored in the memory location whose address 
/// is computed by sign-extending bits [8:0] to 16 
/// bits and adding this value to the incremented PC.
pub fn st(instruction: u16, vm: &mut VM) {
    let sr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    vm.write_memory(val as usize, vm.registers.get(sr));
}

/// STI
/// The contents of the register specified by SR 
/// are stored in the memory location whose address 
/// is obtained as follows: Bits [8:0] are
/// sign-extended to 16 bits and added to the incremented
/// PC. What is in memory at this address is the 
/// address of the location to which the data in 
/// SR is stored.
pub fn sti(instruction: u16, vm: &mut VM) {
    let sr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1ff, 9);
    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    let address = vm.read_memory(val) as usize;

    vm.write_memory(address, vm.registers.get(sr));
}

/// STR
/// The contents of the register specified by SR are
/// stored in the memory location whose address is 
/// computed by sign-extending bits [5:0] to 16 bits
/// and adding this value to the contents of the 
/// register specified by bits [8:6].
pub fn str(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let base_reg = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let val: u32 = vm.registers.get(base_reg) as u32 + offset as u32;
    let val: u16 = val as u16;
    vm.write_memory(val as usize, vm.registers.get(dr));
}
