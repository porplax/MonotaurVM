// All bytecodes that our machine can recognize. Including letters and such.
#[derive(Debug)]
pub enum Bytecodes {
    // OTB Operations.
    HALT,

    // Increasing plate operations.
    PLOP,
    PLUMMET,
    SCALE,

    // Decreasing plate operations.
    TAKE,
    DEVOUR,

    // Integer, float operations.
    INC,
    DNC,

    // Storage operations.
    ISTORE,
    ILOAD, // Marked for removal.
    FSTORE,
    FLOAD, // Marked for removal.
    MERGE,

    // Math operations.
    ADD,
    SUB,
    MUL,
    DIV,
    
    NULL,

    IMPORT,

    // System calls.
    WRITE
}