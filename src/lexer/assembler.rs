use crate::instructions::Opcode;
use crate::lexer::Token;

use super::Instruction;
pub fn get_instruction_token(input: &str) -> Result<Token, usize> {
    match input.to_lowercase().as_str() {
        "add" => Ok(Token::Op { code: Opcode::ADD }),
        "sub" => Ok(Token::Op { code: Opcode::SUB }),
        "load" => Ok(Token::Op { code: Opcode::LOAD }),
        "mult" => Ok(Token::Op { code: Opcode::MULT }),
        "div" => Ok(Token::Op { code: Opcode::DIV }),
        "jmp" => Ok(Token::Op { code: Opcode::JMP }),
        "jmpf" => Ok(Token::Op { code: Opcode::JMPF }),
        "jmpb" => Ok(Token::Op { code: Opcode::JMPB }),
        "eq" => Ok(Token::Op { code: Opcode::EQ }),
        "neq" => Ok(Token::Op { code: Opcode::NEQ }),
        "gt" => Ok(Token::Op { code: Opcode::GT }),
        "lt" => Ok(Token::Op { code: Opcode::LT }),
        "gte" => Ok(Token::Op { code: Opcode::GTE }),
        "lte" => Ok(Token::Op { code: Opcode::LTE }),
        "jee" => Ok(Token::Op { code: Opcode::JEE }),
        "print" => Ok(Token::Op {
            code: Opcode::PRINT,
        }),
        _ => Err(0),
    }
}
pub fn get_int_token(input: &str) -> Result<Token, usize> {
    //check if # is dirs char
    if input.starts_with('#') {
        let int_str = input.chars().skip(1).collect::<String>();
        let int_val = int_str.parse::<i32>();
        match int_val {
            Ok(val) => Ok(Token::IntegerOperand { value: val }),
            Err(_) => Err(0),
        }
    } else {
        Err(0)
    }
}
pub fn get_register_token(input: &str) -> Result<Token, usize> {
    //make sure reg starts with $
    if !input.starts_with('$') {
        return Err(0);
    }
    //make sure reg is a number
    let reg_num = input[1..].parse::<u8>();
    match reg_num {
        Ok(num) => Ok(Token::Register { reg_num: num }),
        Err(err) => {
            println!("Error: {:?}", err);
            Err(1)
        }
    }
}
pub fn get_token(input: &str) -> Result<Instruction, usize> {
    let split = input.split(' ').collect::<Vec<&str>>();
    if split.len() >= 5 {
        return Err(0);
    }
    let mut instruction: Instruction = Instruction {
        opcode: Token::Op { code: Opcode::ADD },
        operand1: None,
        operand2: None,
        operand3: None,
    };
    instruction.opcode = get_instruction_token(split[0]).expect("Invalid instruction");
    if split.len() < 2 {
        return Err(0);
    }
    if instruction.opcode == (Token::Op { code: Opcode::LOAD }) {
        instruction.operand1 = Some(get_register_token(split[1]).expect("Invalid register"));
        instruction.operand2 = Some(get_int_token(split[2]).expect("Invalid integer"));
    } else {
        //check if opcode is JMP JMPF JMPB or PRINT
        if instruction.opcode == (Token::Op { code: Opcode::JMP })
            || instruction.opcode == (Token::Op { code: Opcode::JMPF })
            || instruction.opcode == (Token::Op { code: Opcode::JMPB })
            || instruction.opcode
                == (Token::Op {
                    code: Opcode::PRINT,
                })
        {
            instruction.operand1 = Some(get_register_token(split[1]).expect("Invalid register"));
        } else {
            instruction.operand1 = Some(get_register_token(split[1]).expect("Invalid register"));
            instruction.operand2 = Some(get_register_token(split[2]).expect("Invalid register"));
            //operand 3 is optional
            if split.len() > 3 {
                instruction.operand3 =
                    Some(get_register_token(split[3]).expect("Invalid register"));
            }
        }
    }
    Ok(instruction)
}
//testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruction_token() {
        let mut token = get_instruction_token("ADD");
        assert_eq!(token, Ok(Token::Op { code: Opcode::ADD }));
        token = get_instruction_token("SUB");
        assert_eq!(token, Ok(Token::Op { code: Opcode::SUB }));
    }
    #[test]
    fn test_get_register_token() {
        let mut token = get_register_token("$1");
        assert_eq!(token, Ok(Token::Register { reg_num: 1 }));
        token = get_register_token("$2");
        assert_eq!(token, Ok(Token::Register { reg_num: 2 }));
        token = get_register_token("$a");
        assert_eq!(token, Err(1));
        token = get_register_token("a");
        assert_eq!(token, Err(0));
    }
    #[test]
    fn test_get_int_token() {
        let mut token = get_int_token("#1");
        assert_eq!(token, Ok(Token::IntegerOperand { value: 1 }));
        token = get_int_token("#2");
        assert_eq!(token, Ok(Token::IntegerOperand { value: 2 }));
        token = get_int_token("#a");
        assert_eq!(token, Err(0));
        token = get_int_token("a");
        assert_eq!(token, Err(0));
    }
    #[test]
    fn test_get_token() {
        let mut token = get_token("LOAD $1 #4");
        assert_eq!(
            token,
            Ok(Instruction {
                opcode: Token::Op { code: Opcode::LOAD },
                operand1: Some(Token::Register { reg_num: 1 }),
                operand2: Some(Token::IntegerOperand { value: 4 }),
                operand3: None
            })
        );
        token = get_token("ADD $1 $2 $3");
        assert_eq!(
            token,
            Ok(Instruction {
                opcode: Token::Op { code: Opcode::ADD },
                operand1: Some(Token::Register { reg_num: 1 }),
                operand2: Some(Token::Register { reg_num: 2 }),
                operand3: Some(Token::Register { reg_num: 3 })
            })
        );
        token = get_token("ADD $1 $2");
        assert_eq!(
            token,
            Ok(Instruction {
                opcode: Token::Op { code: Opcode::ADD },
                operand1: Some(Token::Register { reg_num: 1 }),
                operand2: Some(Token::Register { reg_num: 2 }),
                operand3: None
            })
        );
        token = get_token("ADD $1 $2 $3 $4");
        assert_eq!(token, Err(0));
    }
}
