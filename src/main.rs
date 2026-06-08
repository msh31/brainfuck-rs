struct Interpreter {
    tape: Vec<u8>, //memory - storage for the program since no vars
    tc: usize,
    program: String, //source
    pc: usize,
}

impl Interpreter {
    fn run(&mut self) {
        while self.pc < self.program.len() {
            //index into the program with the counter in ASCII
            match self.program.as_bytes()[self.pc] {
                b'>' => self.tc += 1,
                b'<' => self.tc -= 1,

                b'+' => self.tape[self.tc] += 1,
                b'-' => self.tape[self.tc] -= 1,

                b'.' => print!("{}", self.tape[self.tc] as char),
                // b',' =>, //barely used so fuck that shit

                b'[' => {
                    if self.tape[self.tc] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            self.pc += 1;
                            match self.program.as_bytes()[self.pc] {
                                b'[' => depth += 1,
                                b']' => depth -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                b']' => {
                    if self.tape[self.tc] != 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            self.pc -= 1;
                            match self.program.as_bytes()[self.pc] {
                                b']' => depth += 1,
                                b'[' => depth -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            self.pc += 1;
        }
    }
}

const MAX_TAPE_LENGTH: usize = 30000;

fn main() {
    let mut _fuck = Interpreter {
        tape: vec![0; MAX_TAPE_LENGTH],
        program: String::from(">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
+.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
]<+."),
        pc: 0,
        tc: 0
    };

    _fuck.run();
}
