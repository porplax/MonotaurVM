/*
Hi! Welcome to Monotaur!
Created by @porplax

Monotaur is a virtual machine I made to learn more about them,
so this project isn't serious. But I do want to implement seriously cool features into Monotaur!
It won't probably happen, but it'll be cool if it did!

So the machine works in this cycle:

FETCH (Get instruction from vector),
DECODE (The instruction is still decimal, decode it!),
EXECUTE (Do the hard work here),

This is how pretty much how all computers work. And this is how our machine is going to work.
Iterate through the vector and work on each instruction. Simple!

For our stack (plate), no overtly complex work is needed, matter of fact, when you create a vector, that's technically a stack!

As of 10/31/22: My goal is to implement a basic plate system,
PLOP (Add to plate), TAKE (Take from plate), DEVOUR (Erase plate).

As of 11/02/22: I gave up on trying to implement a number system O_O

CHALLENGE: Take up to 4 hex values??

TODO: Update for every change Zayne!
TODO: Implement an ALU.
TODO: Implement imports.
*/

extern crate core;

// Modules to import.
use crate::machine::*;
use crate::fs::open;
use crate::cabinet::Cabinet;

use clap::{parser, Parser};

mod bytecodes; // Holds all the VM's bytecode.
mod fridge; // Holds local variables.
mod machine; // The main VM.
mod plate; // The VM's stack.
mod cabinet;
mod fs;

// TODO: Split global tables, SupportedTypes, Delectable into a separate files.
#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    file: String,

    #[arg(long, short, default_value_t = false)]
    verbose: bool,

    #[arg(long, short, default_value_t = false)]
    bypass: bool,
}

// All supported types by Monotaur.
#[derive(Debug, PartialEq)]
pub enum SupportedTypes {
    Int,
    Float,
    Str,
}

// Mainly for fridge and cabinet.
pub struct Delectable {
    pub istype: SupportedTypes,
    pub int: u32,
    pub float: f32,
    pub str: String,
}

// Current version of Monotaur.
pub const VERSION: usize = 1;

// Verbose option.
pub static mut VERBOSE: bool = false;

// Bypass option.
pub static mut BYPASS: bool = false;

// Maximum size of plate.
const MAX_SIZE: usize = 128;

fn main() {
    let args = Arguments::parse();
    // Set our options in the CLI. Unsafe however.
    unsafe {    VERBOSE = args.verbose; BYPASS = args.bypass;  }
    // let instructions: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x03, 0x00];
    let instructions = open(args.file).unwrap();
    let mut machine = VM::new(MAX_SIZE);

    machine.run(instructions);

    return;
}

