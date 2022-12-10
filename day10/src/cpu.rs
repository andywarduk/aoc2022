pub struct Cpu<'a> {
    instructions: &'a [Instruction],
    x_reg: isize,
    pc: usize,
    cycles: usize,
    instruction_cycle: usize,
    cur_instruction: Option<&'a Instruction>,
}

impl<'a> Cpu<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            x_reg: 1,
            pc: 0,
            cycles: 0,
            instruction_cycle: 0,
            cur_instruction: None,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.cycles += 1;

        match self.cur_instruction {
            None => {
                // Fetch first instruction
                self.cur_instruction = Some(&self.instructions[self.pc]);
                self.instruction_cycle = 1;
            }
            Some(instruction) => match instruction.tick(self) {
                // Tick the instruction
                InstructionAction::Retire => {
                    // Fetch the next instruction
                    self.pc += 1;

                    if self.pc >= self.instructions.len() {
                        return false;
                    }

                    self.cur_instruction = Some(&self.instructions[self.pc]);
                    self.instruction_cycle = 1;
                }
                InstructionAction::Executing => self.instruction_cycle += 1,
            },
        }

        true
    }

    pub fn x_reg(&self) -> isize {
        self.x_reg
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }
}

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
