use std::collections::HashMap;

pub enum Token {
    DirectValue(i32),
    IndirectValue(i32),
    Register(u8),
    Opcode(String),
    LabelDef(String),
    LabelCall(String),
    ArgSeparator,
    Comment(String),
    Directive(String, String),
}

pub struct Mnemonic {
    pub opcode: u8,
    pub have_format: bool,
    pub cycles: u32,
}

pub fn get_mnemonics() -> Result<HashMap<String, Mnemonic>, String> {
    let mnemonics = HashMap::from([
        (
            "live".to_string(),
            Mnemonic {
                opcode: 0x01,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "ld".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "st".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "add".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "sub".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "and".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "or".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "xor".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "zjmp".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "ldi".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "sti".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "fork".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "lld".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "lldi".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "lfork".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
        (
            "aff".to_string(),
            Mnemonic {
                opcode: 0x02,
                have_format: false,
                cycles: 1,
            },
        ),
    ]);

    Ok(mnemonics)
}
