#[derive(Debug)]
pub enum Instruction {
    NoOp,
    AddX(isize),
}

impl Instruction {
    fn tick(&self, cpu: &mut Cpu) -> InstructionAction {
        use Instruction::*;

        match self {
            NoOp => match cpu.instruction_cycle {
                1 => InstructionAction::Retire,
                _ => unreachable!(),
            },
            AddX(amt) => match cpu.instruction_cycle {
                1 => InstructionAction::Executing,
                2 => {
                    cpu.x_reg += amt;
                    InstructionAction::Retire
                }
                _ => unreachable!(),
            },
        }
    }
}

enum InstructionAction {
    Retire,
    Executing,
}

pub struct Cpu<'a> {
    instructions: &'a [Instruction],
    x_reg: isize,
    pc: usize,
    cycles: usize,
    instruction_cycle: usize,
    cur_instruction: &'a Instruction,
}

impl<'a> std::fmt::Debug for Cpu<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("x_reg", &self.x_reg)
            .field("pc", &self.pc)
            .field("cycles", &self.cycles)
            .field("instruction_cycle", &self.instruction_cycle)
            .field("cur_instruction", &self.cur_instruction)
            .finish()
    }
}

impl<'a> Cpu<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            x_reg: 1,
            pc: 0,
            cycles: 0,
            instruction_cycle: 0,
            cur_instruction: &instructions[0],
        }
    }

    pub fn tick(&mut self) -> TickAction {
        self.cycles += 1;

        //println!("{:?}", self);
        if self.instruction_cycle == 0 {
            self.instruction_cycle += 1;
            TickAction::Executing
        } else {
            match self.cur_instruction.tick(self) {
                InstructionAction::Retire => {
                    self.pc += 1;
                    self.instruction_cycle = 1;

                    if self.pc < self.instructions.len() {
                        self.cur_instruction = &self.instructions[self.pc];
                        TickAction::Executing
                    } else {
                        TickAction::Halt
                    }
                }
                InstructionAction::Executing => {
                    self.instruction_cycle += 1;
                    TickAction::Executing
                }
            }
        }
    }

    pub fn x_reg(&self) -> isize {
        self.x_reg
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }
}

pub enum TickAction {
    Halt,
    Executing,
}
