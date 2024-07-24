#![allow(unused)]
use std::fs::File;
use std::env::args;
use byteorder::{BigEndian, ReadBytesExt};
use crate::hardware::vm::*;
use crate::hardware::instruction;

mod hardware;

pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

fn main() {

    // Open file
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let mut f = File::open(args[1].clone()).expect("Unable to open file");

    // Create VM
    let mut vm = VM::new();


    // Read u16 instructions from file
    let base_address = f.read_u16::<BigEndian>().expect("error");
    // Starting from the base address
    let mut address = base_address as usize;
    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                // println!("address: {:x} instruction: {:b}\n", address, instruction);
                vm.write_memory(address, instruction);
                address += 1;
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("OK");
                } else {
                    println!("failed: {}", e);
                }
                break;
            }
        }
    }

    execute_program(&mut vm);
}

pub fn execute_program(vm: &mut VM) {
    while vm.registers.pc < MEMORY_SIZE as u16 {
        // Read instruction
        let instruction = vm.read_memory(vm.registers.pc);

        // Increment PC
        vm.registers.pc += 1;

        // Extract op_code and execute operation
        instruction::execute_instruction(instruction, vm);
    }
}
