use crate::types::Token;
use crate::types::get_mnemonics;
use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

fn parse_integer(chars: &mut Peekable<Chars<'_>>, base_digits: String, col: &mut i32) -> i32 {
    //fn parse_indirect(chars: &Iterator<char>, col: &i32) -> i32 {
    let base_digits: HashSet<char> = base_digits.chars().collect();
    let mut num_str = String::new();
    while let Some(&digit) = chars.peek() {
        *col = *col + 1;
        if base_digits.contains(&digit) {
            num_str.push(digit);
            chars.next(); // Consume the character
        } else {
            break; // End of number
        }
    }
    num_str.parse::<i32>().unwrap()
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let _octal = "01234567";
    let decimal = "0123456789";
    let _hexadecimal = "0123456789ABCDEF";
    let base_digits: HashSet<char> = decimal.chars().collect();
    let func_aplhabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let mut chars = expr.chars().peekable();
    let mut opcode_catch = false;
    let mut label_found = false;
    let mut col = -1;
    while let Some(&c) = chars.peek() {
        col = col + 1;
        if base_digits.contains(&c) {
            tokens.push(Token::IndirectValue(parse_integer(
                &mut chars,
                decimal.to_string(),
                &mut col,
            )));
        } else if c == '%' {
            chars.next(); // Consume the character
            let next = chars.peek().unwrap();
            if base_digits.contains(&next) {
                tokens.push(Token::DirectValue(parse_integer(
                    &mut chars,
                    decimal.to_string(),
                    &mut col,
                )));
            } else if *next == ':' {
                chars.next();
                let mut label_str = String::new();
                while let Some(&digit) = chars.peek() {
                    col = col + 1;
                    if func_aplhabet.contains(&digit) {
                        label_str.push(digit);
                        chars.next(); // Consume the character
                    } else {
                        break; // End of func name
                    }
                }
                tokens.push(Token::LabelCall(label_str));
            } else {
                return Err(format!(
                    "Invalid direct operand found: {} on col {}",
                    expr, col
                ));
            }
        } else if func_aplhabet.contains(&c) {
            if opcode_catch == false {
                // should be the opcode
                let mnemonics = get_mnemonics().unwrap();
                let mut opcode_str = String::new();
                while let Some(&digit) = chars.peek() {
                    col = col + 1;
                    if func_aplhabet.contains(&digit) {
                        opcode_str.push(digit);
                        chars.next(); // Consume the character
                    } else {
                        if digit == ':' {
                            label_found = true;
                            chars.next(); // Consume the character
                        }
                        break; // End of func name
                    }
                }
                if label_found {
                    tokens.push(Token::LabelDef(opcode_str));
                } else {
                    if mnemonics.contains_key(&opcode_str) {
                        tokens.push(Token::Opcode(opcode_str));
                    } else {
                        return Err(format!("Invalid opcode found: {}", opcode_str));
                    }
                    opcode_catch = true;
                }
            } else if c == 'r' {
                // should be a register
                chars.next(); // skip 'r' 
                tokens.push(Token::Register(
                    parse_integer(&mut chars, decimal.to_string(), &mut col) as u8,
                ));
            } else {
                return Err(format!("Invalid identifier found: {}", c));
            }
        } else if c == ',' {
            tokens.push(Token::ArgSeparator);
            chars.next();
        } else if c == '#' {
            chars.next();
            let mut comment_str = String::new();
            while let Some(&com) = chars.peek() {
                col = col + 1;
                comment_str.push(com);
                chars.next(); // Consume the character
            }
            tokens.push(Token::Comment(comment_str));
        } else if c == '.' {
            chars.next();
            let mut directive_str = String::new();
            let mut val_str = String::new();
            let mut val_start = false;
            while let Some(&com) = chars.peek() {
                col = col + 1;
                if com == ' ' {
                    val_start = true;
                }
                if val_start == false {
                    directive_str.push(com);
                } else {
                    val_str.push(com);
                }
                chars.next(); // Consume the character
            }
            tokens.push(Token::Directive(directive_str, val_str));
        } else if c.is_whitespace() {
            chars.next(); // Ignore whitespace
        } else {
            let mut shift = String::from("");
            let mut to = 0;
            while to < col {
                shift.push('-');
                to = to + 1;
            }
            return Err(format!(
                "Invalid character '{}' found on col {} : \n'{}'\n {}^",
                c, col, expr, shift
            ));
        }
    }
    Ok(tokens)
}
