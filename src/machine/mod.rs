/*
TODO LIST:
ALU Implementation.
Make VM inherit the instructions.
*/

mod imports;

// Import needed crates here.
use crate::*;
use crate::bytecodes::*;
use crate::bytecodes::Bytecodes::*;
use crate::fridge::*;
use crate::machine::imports::handle_imports;
use crate::plate::*;

// Headers will hold chow's magic numbers and version.
struct Headers {
    magic0: u8,
    magic1: u8,
    magic2: u8,
    magic3: u8,
    version: usize,
}

// This will be our struct to construct our virtual machine.
pub struct VM {
    // Is it running?
    state: bool,
    // Program counter.
    pc: usize,
    // Our plate.
    plate: Plate,
    // Our magic numbers and version.
    headers: Headers,
    // Increments for integers.
    integer: u32,
    // Increments for floats.
    float: f32,
    // Holds strings.
    string: String,
    // Our fridge.
    fridge: Fridge,
}

impl VM {
    pub fn new(capacity: usize) -> VM {
        // Set it all here.
        return VM {
            state: true,
            pc: 0,
            plate: Plate::new(capacity),
            integer: 0, // TODO: Make a struct for integers. (ALU)
            float: 0.0, // TODO: Make a struct for floats. (ALU)
            string: String::from(""),
            headers: Headers {
                magic0: 0,
                magic1: 0,
                magic2: 0,
                magic3: 0,
                version: 0,
            },
            fridge: Fridge::new(capacity),
        };
    }

    // Is it running?
    pub fn is_running(&self) -> bool {
        return match self.state {
            true => true,
            _ => false
        };
    }

    // Set our state to...
    pub fn set_running(&mut self, state: bool) {
        return self.state = state;
    }

    // Our main cycle.
    pub fn run(&mut self, program: Vec<u8>) {
        self.validate(program.clone());
        let mut fetched: u8;
        let mut decode: Bytecodes;
        loop {
            if self.is_running() == false {
                return;
            }

            if self.pc == program.len() {
                panic!("\u{001b}[31mAt {}: Hit end of program, no HALT detected.\u{001b}[0m", self.pc);
            }

            // Safe sex is great sex, thanks rust ;-;
            fetched = self.fetch(program.clone());
            decode = self.decode(fetched);
            self.execute(decode);

            // Loop again!
        }
    }

    // Validate and scan our program for any preventable errors.
    fn validate(&mut self, program: Vec<u8>) {
        // CHECK ONE; Ensure we're not reading a list less than 4 instructions.
        if program.len() <= 4 {
            panic!("\u{001b}[31mThere are not enough instructions in the program.\u{001b}[0m")
        }

        let mut fetched: u8;

        let mut rv: Vec<u8> = vec![]; // RV = Requires validation.

        // We get the first five bytes of the program.
        loop {
            if self.pc == 5 {
                break;
            }
            fetched = self.fetch(program.clone());
            rv.push(fetched);
        }

        // CHECK TWO; Check if first four bytes are 0xBEEFBABE.
        if rv[0] != 190 || rv[1] != 239 || rv[2] != 186 || rv[3] != 190 {
            panic!("\u{001b}[31mThe program couldn't be verified. The first four bytes need to be \u{001b}[33m190\u{001b}[31m, \u{001b}[33m239\u{001b}[31m, \u{001b}[33m186\u{001b}[31m, \u{001b}[33m190\u{001b}[31m (0xBEEFBABE)\nInstead, they're \u{001b}[33m{}\u{001b}[0m, \u{001b}[33m{}\u{001b}[0m, \u{001b}[33m{}\u{001b}[0m, \u{001b}[33m{}\u{001b}[0m", rv[0], rv[1], rv[2], rv[3])
        }

        // CHECK THREE; Check if chow's version is higher than Monotaur's version, if so, exit.
        unsafe {
            if rv[4] as usize > VERSION && BYPASS != true {
                panic!("\u{001b}[31mChow's version (\u{001b}[33m{}\u{001b}[31m) is higher than Monotaur's version (\u{001b}[33m{}\u{001b}[31m).\nConsider updating to version \u{001b}[33m{}\u{001b}[31m to bypass this incompatibility, or use a version of the Chow file to fit with this release of Monotaur.\nUsing '--bypass' can bypass this difference but is not recommended.\u{001b}[0m", rv[4], VERSION, rv[4])
            }
        }

        // END OF CHECKS.
        return;
    }

    // Fetch the program!
    fn fetch(&mut self, program: Vec<u8>) -> u8 {
        let current = program[self.pc];
        self.pc = self.pc + 1;
        return current;
    }

    // Decooooooode it.....
    fn decode(&mut self, program: u8) -> Bytecodes {
        // Since hexes 1-19 is not occupied, we will use them here. 
        return match program {
            0 => HALT,

            1 => PLOP,
            2 => PLUMMET,
            3 => SCALE,

            4 => TAKE,
            5 => DEVOUR,

            6 => INC,
            7 => DNC,

            8 => ISTORE,
            9 => ILOAD,
            10 => FSTORE,
            11 => FLOAD,

            14 => ADD,
            15 => SUB,
            16 => MUL,
            17 => DIV,

            // Hex to decimal starts actin' strange around 9-19. So the instruction is at 22.
            // 11/02: nvm 9-19 was 0A-0F im a dumbbell.
            22 => IMPORT,

            /*
            This is where stuff like symbols and letters will start.

            To prevent the bytecodes mod.rs file looking like a bad hair day, I think it's a good idea to
            instantly decode into pushing to string.

            Maybe it's not recommended but it'll do.

            A..Z, a..z, symbols...
             */

            32 => { self.string.push(' '); NULL }
            33 => { self.string.push('!'); NULL }
            46 => { self.string.push('.'); NULL }
            47 => { self.string.push('/'); NULL }

            63 => { self.string.push('?'); NULL }
            65 => { self.string.push('A'); NULL }
            66 => { self.string.push('B'); NULL }
            67 => { self.string.push('C'); NULL }
            68 => { self.string.push('D'); NULL }
            69 => { self.string.push('E'); NULL }
            70 => { self.string.push('F'); NULL }
            71 => { self.string.push('G'); NULL }
            72 => { self.string.push('H'); NULL }
            73 => { self.string.push('I'); NULL }
            74 => { self.string.push('J'); NULL }
            75 => { self.string.push('K'); NULL }
            76 => { self.string.push('L'); NULL }
            77 => { self.string.push('M'); NULL }
            78 => { self.string.push('N'); NULL }
            79 => { self.string.push('O'); NULL }
            80 => { self.string.push('P'); NULL }
            81 => { self.string.push('Q'); NULL }
            82 => { self.string.push('R'); NULL }
            83 => { self.string.push('S'); NULL }
            84 => { self.string.push('T'); NULL }
            85 => { self.string.push('U'); NULL }
            86 => { self.string.push('V'); NULL }
            87 => { self.string.push('W'); NULL }
            88 => { self.string.push('X'); NULL }
            89 => { self.string.push('Y'); NULL }
            90 => { self.string.push('Z'); NULL }

            97 => { self.string.push('a'); NULL }
            98 => { self.string.push('b'); NULL }
            99 => { self.string.push('c'); NULL }
            100 => { self.string.push('d'); NULL }
            101 => { self.string.push('e'); NULL }
            102 => { self.string.push('f'); NULL }
            103 => { self.string.push('g'); NULL }
            104 => { self.string.push('h'); NULL }
            105 => { self.string.push('i'); NULL }
            106 => { self.string.push('j'); NULL }
            107 => { self.string.push('k'); NULL }
            108 => { self.string.push('l'); NULL }
            109 => { self.string.push('m'); NULL }
            110 => { self.string.push('n'); NULL }
            111 => { self.string.push('o'); NULL }
            112 => { self.string.push('p'); NULL }
            113 => { self.string.push('q'); NULL }
            114 => { self.string.push('r'); NULL }
            115 => { self.string.push('s'); NULL }
            116 => { self.string.push('t'); NULL }
            117 => { self.string.push('u'); NULL }
            118 => { self.string.push('v'); NULL }
            119 => { self.string.push('w'); NULL }
            120 => { self.string.push('x'); NULL }
            121 => { self.string.push('y'); NULL }
            122 => { self.string.push('z'); NULL }

            _ => {
                panic!("\u{001b}[31mDecoding failed. {} is not known\u{001b}[0m", program);
            }
        };
    }

    // Finally, execute it.
    // TODO: Merge ILOAD, FLOAD into one function. Makes no sense of me not to make use of the 'istype' value. Could free up space.
    fn execute(&mut self, program: Bytecodes) {
        // IF User has verbose enabled, show every command being executed.
        unsafe {
            // TODO: Match/case maybe?
            if VERBOSE == true {
                println!("( Now executing \u{001b}[33m{:?}\u{001b}[0m! | PC: {} | Integer, Float, String: \u{001b}[34m{}\u{001b}[0m, \u{001b}[35m{}\u{001b}[0m, '\u{001b}[36m{}\u{001b}[0m' )", program, self.pc, self.integer, self.float, self.string);
            }
        }
        match program {
            HALT => self.set_running(false),

            PLOP => {
                let seasoning = GetValueOfType {
                    int: self.integer,
                    float: 0.0,
                    str: String::from(""),
                };

                let edible = PackedIngredients {
                    istype: SupportedTypes::Int,
                    value: seasoning,
                };

                self.plate.plop(edible);
                self.integer = 0;
            }
            PLUMMET => {
                let seasoning = GetValueOfType {
                    int: 0,
                    float: self.float,
                    str: String::from(""),
                };

                let edible = PackedIngredients {
                    istype: SupportedTypes::Float,
                    value: seasoning,
                };

                self.plate.plop(edible);
                self.float = 0.0;
            }
            SCALE => {
                let seasoning = GetValueOfType {
                    int: 0,
                    float: 0.0,
                    str: (&self.string).to_string(),
                };

                let edible = PackedIngredients {
                    istype: SupportedTypes::Str,
                    value: seasoning,
                };

                self.plate.plop(edible);
                self.string = String::from("");
            }

            TAKE => self.plate.take(),
            DEVOUR => self.plate.devour(),

            INC => {
                self.integer = self.integer + 1;
                self.float = self.float + 0.1;
            }
            DNC => {
                self.integer = self.integer - 1;
                self.float = self.float - 0.1;
            }

            ISTORE => {
                let delectable = Delectable {
                    istype: SupportedTypes::Int,
                    int: self.plate.peek().value.int,
                    float: 0.0,
                    str: String::from(""),
                };

                self.fridge.insert(self.integer, delectable);
            }
            ILOAD => {
                let delectable = self.fridge.retreive(&self.integer);
                let seasoning = GetValueOfType {
                    int: delectable.int,
                    float: 0.0,
                    str: String::from(""),
                };

                let edible = PackedIngredients {
                    istype: SupportedTypes::Int,
                    value: seasoning,
                };

                self.plate.plop(edible);
                self.integer = 0;
            }
            FSTORE => {
                let delectable = Delectable {
                    istype: SupportedTypes::Float,
                    int: 0,
                    float: self.plate.peek().value.float,
                    str: String::from(""),
                };

                self.fridge.insert(self.integer, delectable);
            }
            FLOAD => {
                let delectable = self.fridge.retreive(&self.integer);
                let seasoning = GetValueOfType {
                    int: 0,
                    float: delectable.float,
                    str: String::from(""),
                };

                let edible = PackedIngredients {
                    istype: SupportedTypes::Float,
                    value: seasoning,
                };

                self.plate.plop(edible);
                self.integer = 0;
            }

            IMPORT => {
                /*
                This is a confusing function, internally speaking...

                Should I start a thread? Execute at run-time? How the heck do we handle globals?
                So as of 11/02 the Import function only executes Monotaur's scripts at run-time at the moment.

                TODO: Stop being so dzamn lazy.
                */

                handle_imports((&self.string).to_string());
                self.string = String::from("");
            }

            NULL => {} // silence........

            _ => {
                panic!("\u{001b}[31mExecuting failed. {:?} is either used in a undisclosed manner, or is a removed keyword.\u{001b}[0m", program);
            }
        }
    }
}