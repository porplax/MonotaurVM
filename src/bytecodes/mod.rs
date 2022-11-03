// All bytecodes that our machine can recognize. Including letters and such.
#[derive(Debug)]
pub enum Bytecodes {
    HALT,

    PLOP,
    PLUMMET,
    SCALE,

    TAKE,
    DEVOUR,

    INC,
    DNC,

    ISTORE,
    ILOAD, // Marked for removal.
    FSTORE,
    FLOAD, // Marked for removal.

    ADD,
    SUB,
    MUL,
    DIV,

    NULL,

    IMPORT
}