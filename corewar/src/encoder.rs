use crate::types::Token;
use crate::types::get_mnemonics;

pub fn encode(
    tokens: Result<Vec<Token>, String>,
    line_count: i32,
    offset: u32,
) -> Result<Vec<u8>, String> {
    let mut res = Vec::new();
    let mnemonics = get_mnemonics().unwrap();
    let mut need_enc_byte: bool = false;
    match tokens {
        Ok(vect) => {
            for token in vect {
                let _res = match token {
                    Token::DirectValue(n) => res.extend_from_slice(&n.to_be_bytes()),
                    Token::IndirectValue(n) => res.extend_from_slice(&n.to_be_bytes()),
                    Token::Register(r) => res.push(r),
                    Token::Opcode(op) => {
                        need_enc_byte = mnemonics[&op].have_format;
                        res.push(mnemonics[&op].opcode);
                    }
                    Token::LabelDef(_op) => res.push(0),
                    Token::LabelCall(_op) => res.push(0),
                    Token::ArgSeparator => res.push(0),
                    Token::Comment(_c) => res.push(0),
                    Token::Directive(_d, _v) => res.push(0),
                };
            }
        }
        Err(e) => {
            println!("Error on line {} : {}", line_count, e);
        }
    }
    if need_enc_byte {
        println!("Encoding byte needed");
    }
    Ok(res)
}
