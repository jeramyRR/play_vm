pub mod opcode_parser;

use instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
  Op { code: Opcode },
  Register { reg_num: u8 },
}
