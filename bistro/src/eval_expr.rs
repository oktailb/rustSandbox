use std::collections::HashSet;

use crate::add::add_strings;
use crate::div::div_strings;
use crate::div::mod_strings;
use crate::mul::mul_strings;
use crate::sub::sub_strings;

enum Token {
    Number(String),
    Operator(char),
    Function(String),
    ArgSeparator,
    LeftParen,
    RightParen,
}

fn tokenize(expr: &str, base_str: &str, base_op: &str) -> Result<Vec<Token>, String> {
    let base_digits: HashSet<char> = base_str.chars().collect();
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    let func_aplhabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    while let Some(&c) = chars.peek() {
        if base_digits.contains(&c) {
            let mut num_str = String::new();
            while let Some(&digit) = chars.peek() {
                if base_digits.contains(&digit) {
                    num_str.push(digit);
                    chars.next(); // Consume the character
                } else {
                    break; // End of number
                }
            }
            tokens.push(Token::Number(num_str));
        } else if func_aplhabet.contains(&c) {
            // this is a function name
            let mut func_str = String::new();
            while let Some(&digit) = chars.peek() {
                if func_aplhabet.contains(&digit) {
                    func_str.push(digit);
                    chars.next(); // Consume the character
                } else {
                    break; // End of func name
                }
            }
            tokens.push(Token::Function(func_str));
        } else if base_op.contains(c) {
            tokens.push(Token::Operator(c));
            chars.next(); // Consume the operator
        } else if c == ',' {
            tokens.push(Token::ArgSeparator);
            chars.next();
        } else if c == '(' {
            tokens.push(Token::LeftParen);
            chars.next();
        } else if c == ')' {
            tokens.push(Token::RightParen);
            chars.next();
        } else if c.is_whitespace() {
            chars.next(); // Ignore whitespace
        } else {
            return Err(format!("Invalid character found: {}", c));
        }
    }
    Ok(tokens)
}

fn get_precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0,
    }
}

fn infix_to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push(token),

            Token::Operator(op1) => {
                while let Some(Token::Operator(op2)) = operator_stack.last() {
                    if get_precedence(op1) <= get_precedence(*op2) {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(Token::Operator(op1));
            }

            Token::LeftParen => operator_stack.push(token),

            Token::RightParen => {
                let mut found_left_paren = false;
                while let Some(top_op) = operator_stack.pop() {
                    if let Token::LeftParen = top_op {
                        found_left_paren = true;
                        break;
                    }
                    output_queue.push(top_op);
                }
                if !found_left_paren {
                    return Err("Parenthèses discordantes.".to_string());
                }
            }
            Token::Function(f) => operator_stack.push(Token::Function(f)),
            Token::ArgSeparator => todo!(),
        }
    }

    while let Some(op) = operator_stack.pop() {
        if let Token::LeftParen = op {
            return Err("Parenthèses discordantes.".to_string());
        }
        output_queue.push(op);
    }

    Ok(output_queue)
}

fn evaluate_rpn(rpn_tokens: Vec<Token>, base_str: &str) -> Result<String, String> {
    let mut value_stack: Vec<String> = Vec::new();

    for token in rpn_tokens {
        match token {
            Token::Number(n) => value_stack.push(n),
            Token::Operator(op) => {
                let b = value_stack.pop().ok_or("Stack underflow")?;
                let a = value_stack.pop().ok_or("Stack underflow")?;

                let result = match op {
                    '+' => add_strings(&a, &b, base_str)?,
                    '-' => sub_strings(&a, &b, base_str)?,
                    '*' => mul_strings(&a, &b, base_str)?,
                    '/' => div_strings(&a, &b, base_str)?,
                    '%' => mod_strings(&a, &b, base_str)?,
                    _ => return Err(format!("Unknown operator: {}", op)),
                };
                value_stack.push(result);
            }
            _ => return Err("Unknown token in RPN expression".to_string()),
        }
    }

    value_stack.pop().ok_or("No result".to_string())
}

pub fn eval(expr: &str, base_str: &str, base_op: &str) -> Result<String, String> {
    let tokens = tokenize(expr, base_str, base_op)?;
    let rpn_expr = infix_to_rpn(tokens)?;
    evaluate_rpn(rpn_expr, base_str)
}
