mod encoder;
mod tokenizer;
mod types;

use crate::encoder::encode;
use crate::tokenizer::tokenize;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let asm_file = &args[1];
        if let Ok(lines) = read_lines(asm_file) {
            let mut line_count = 0;
            let mut offset = 0;
            for line in lines.map_while(Result::ok) {
                let tokens = tokenize(&line);
                let encoded = encode(tokens, line_count, offset);
                match encoded {
                    Ok(res) => offset = offset + res.len() as u32,
                    Err(e) => println!("Encoding error: {}", e),
                }

                line_count = line_count + 1;
            }
        }
    } else {
        println!("Usage: {} input.asm", &args[0]);
    }
}
