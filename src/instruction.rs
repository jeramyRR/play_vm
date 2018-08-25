#[derive(Debug, PartialEq)]
pub enum Opcode {
  ADD,
  DIV,
  HLT,  // Halt
  LOAD, // Load
  MUL,  // Multiply
  SUB,  // Subtract
  JMP,  // Jump Absolute
  JMPF, // Jump Forward
  JMPB, // Jump Backward
  EQ,   // Equal
  NEQ,  // Not equal
  GT,   // Greater Than
  LT,   // Less Than
  GTQ,  // Greather than or Equal to
  LTQ,  // Less than or Equal to
  JEQ,  // Jump if Equal
  JNEQ, // Jump if Not Equal
  IGL,  // Illegal
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => Opcode::LOAD,
      1 => Opcode::ADD,
      2 => Opcode::SUB,
      3 => Opcode::MUL,
      4 => Opcode::DIV,
      5 => Opcode::HLT,
      6 => Opcode::JMP,
      7 => Opcode::JMPF,
      8 => Opcode::JMPB,
      9 => Opcode::EQ,
      10 => Opcode::NEQ,
      11 => Opcode::GT,
      12 => Opcode::LT,
      13 => Opcode::GTQ,
      14 => Opcode::LTQ,
      15 => Opcode::JEQ,
      16 => Opcode::JNEQ,
      _ => Opcode::IGL,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction { opcode: opcode }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_instruction() {
    let opcode = Opcode::HLT;
    let instruction = Instruction::new(opcode);

    assert_eq!(instruction.opcode, Opcode::HLT);
  }
}
