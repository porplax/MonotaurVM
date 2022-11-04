// Import needed files.
use crate::fs::open;
use crate::{MAX_SIZE, VM};

// Function to handle imports...
pub fn handle_imports(file: String) {
    let instructions = open(file).unwrap();
    let mut machine = VM::new(MAX_SIZE);

    machine.run(instructions);

    return;
}