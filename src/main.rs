use std::collections::HashMap;

// enum for instruction types
#[derive(Clone, Copy,Debug)]
enum InstructionType {
    Inc,
    Dec,
    Push,
    Pop,
    Jmp,
    Jz,
    Jnz,
    Call,
    Ret,
    Nand,
    Halt,
    Pick,
    Poke,
    Swap,
    Load,
    Store,
}

#[derive(Clone, Copy,Debug)]
struct Instruction{
    instruction: InstructionType,
    operand: u8,
}

struct Frame{
    stack: Vec<u8>,
    instructions: Vec<Instruction>,
    pc: u8,
    retstack: Vec<u8>,
    labels: HashMap<String, u8>,
    memory: Vec<u8>,
}

impl Frame {
    fn new() -> Self {
        Frame {
            stack: Vec::new(),
            instructions: Vec::new(),
            pc: 0,
            retstack: Vec::new(),
            labels: HashMap::new(),
            memory: vec![0; 256],
        }
    }

    fn assembler(&mut self, code: &str, pass: u8) {
        self.pc = 0;
        // split the code into lines
        let lines = code.split("\n");
        // iterate over the lines
        for line in lines {
            // split the line into words as a vector to be indexed
            let words = line.split_whitespace();
            let words = words.collect::<Vec<&str>>();
            if words.len() == 0 {
                // if the line is empty, skip it
                continue;
            }
            if words[0].starts_with(";") {
                // if the first word starts with a semicolon, it is a comment
                // skip the line
                continue;
            }
            if words[0].ends_with(":") {
                // if the first word ends with a colon, it is a label
                // remove the colon and add the label to the labels hashmap
                let label = words[0].trim_end_matches(":");
                self.labels.insert(label.to_string(), self.pc);
                continue;
            } else {
                // if the first word does not end with a colon, it is an instruction
                // increment the program counter
                self.pc += 1;
            }
            if pass == 1 {
                // if this is the first pass, continue to the next line
                continue;
            }
            // match first word to instruction and push instruction with operand to instructions vector
            match words[0] {
                "inc" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Inc, operand: 0 });
                }
                "dec" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Dec, operand: 0 });
                }
                "push" => {
                    let operand = words[1].parse().unwrap();
                    self.instructions.push(Instruction { instruction: InstructionType::Push, operand });
                }
                "pop" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Pop, operand: 0 });
                }
                "jmp" => {
                    let operand = self.labels[words[1]];
                    self.instructions.push(Instruction { instruction: InstructionType::Jmp, operand });
                }
                "jz" => {
                    let operand = self.labels[words[1]];
                    self.instructions.push(Instruction { instruction: InstructionType::Jz, operand });
                }
                "jnz" => {
                    let operand = self.labels[words[1]];
                    self.instructions.push(Instruction { instruction: InstructionType::Jnz, operand });
                }
                "call" => {
                    let operand = self.labels[words[1]];
                    self.instructions.push(Instruction { instruction: InstructionType::Call, operand });
                }
                "ret" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Ret, operand: 0 });
                }
                "nand" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Nand, operand: 0 });
                }
                "halt" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Halt, operand: 0 });
                }
                "pick" => {
                    let operand = words[1].parse().unwrap();
                    self.instructions.push(Instruction { instruction: InstructionType::Pick, operand });
                }
                "poke" => {
                    let operand = words[1].parse().unwrap();
                    self.instructions.push(Instruction { instruction: InstructionType::Poke, operand });
                }
                "swap" => {
                    self.instructions.push(Instruction { instruction: InstructionType::Swap, operand: 0 });
                }
                "load" => {
                    let operand = words[1].parse().unwrap();
                    self.instructions.push(Instruction { instruction: InstructionType::Load, operand });
                }
                "store" => {
                    let operand = words[1].parse().unwrap();
                    self.instructions.push(Instruction { instruction: InstructionType::Store, operand });
                }
                _ => {
                    panic!("Invalid instruction");
                }
            }
        }
    }

    // method to execute an instruction
    fn execute_single(&mut self, instruction: Instruction) {
        match instruction.instruction {
            InstructionType::Push => {
                // push the next byte onto the stack
                self.pc += 1;
                self.stack.push(instruction.operand);
            }
            InstructionType::Pop => {
                self.pc +=1;
                self.stack.pop();
            }
            InstructionType::Inc => {
                // increment the top of the stack
                self.pc += 1;
                let top = self.stack.last_mut().unwrap();
                *top += 1;
            }
            InstructionType::Dec => {
                self.pc +=1;
                // decrement the top of the stack
                let top = self.stack.last_mut().unwrap();
                *top -= 1;
            }
            InstructionType::Jmp => {
                // jump to the address in the next byte
                self.pc = instruction.operand;
            }
            InstructionType::Jz => {
                // jump to the address in the next byte if the top of the stack is zero
                if self.stack.last().unwrap() == &0 {
                    self.pc = instruction.operand;
                }
                else {
                    self.pc += 1;
                }
            }
            InstructionType::Jnz => {
                // jump to the address in the next byte if the top of the stack is not zero
                if self.stack.last().unwrap() != &0 {
                    self.pc = instruction.operand;
                }
                else {
                    self.pc += 1;
                }
            }
            InstructionType::Call => {
                // push the current pc onto the retstack and jump to the address in the next byte
                self.pc +=1;
                self.retstack.push(self.pc as u8);
                self.pc = instruction.operand;
            }
            InstructionType::Ret => {
                // pop the top of the retstack and jump to that address
                self.pc = self.retstack.pop().unwrap() as u8;
            }
            InstructionType::Nand => {
                // pop the top two values from the stack, perform a NAND operation, and push the result
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(!(a & b));
                self.pc += 1;
            }
            InstructionType::Halt => {
                // halt the program
                std::process::exit(0);
            }
            InstructionType::Pick => {
                // push the nth value on the stack onto the stack
                let n = instruction.operand;
                let value = self.stack[self.stack.len() - n as usize - 1];
                self.stack.push(value);
                self.pc += 1;
            }
            InstructionType::Poke => {
                // pop the top value from the stack and set the nth value on the stack to that value
                let n = instruction.operand;
                let value = self.stack.pop().unwrap();
                let len = self.stack.len();
                self.stack[len - n as usize - 1] = value;
                self.pc += 1;
            }
            InstructionType::Swap => {
                // swap the top two values on the stack
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a);
                self.stack.push(b);
                self.pc += 1;
            }
            InstructionType::Load => {
                // push the value at the nth address in memory onto the stack
                let n = instruction.operand;
                let value = self.memory[n as usize];
                self.stack.push(value);
                self.pc += 1;
            }
            InstructionType::Store => {
                // pop the top value from the stack and set the nth address in memory to that value
                let n = instruction.operand;
                let value = self.stack.pop().unwrap();
                self.memory[n as usize] = value;
                self.pc += 1;
            }
        }
    }

    fn execute(&mut self) {
        // loop over the instructions vector and execute each instruction
        self.pc = 0;
        while self.pc < self.instructions.len() as u8 {
            println!("pc: {}", self.pc);
            println!("stack: {:?}", self.stack);
            let instruction = self.instructions[self.pc as usize];
            println!("instruction: {:?}", instruction);
            println!();
            self.execute_single(instruction);
            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }


}

fn main() {
    let _frame = Frame::new();
    let mut frame = Frame::new();
    // read files from command line arguments
    let args: Vec<String> = std::env::args().collect();
    // read code from file passed as command line argument
    let code = std::fs::read_to_string(&args[1]).unwrap();
    frame.assembler(&code, 1);
    frame.assembler(&code, 2);
    frame.execute();
}
