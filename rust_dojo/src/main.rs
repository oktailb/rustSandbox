#[derive(Debug, PartialEq)]
enum ParseError {
    TooShort,
    InvalidMagic,
    TruncatedPayload { expected: usize, actual: usize },
}

#[derive(Debug, PartialEq)]
struct Frame<'a> {
    frame_type: u8,
    payload: &'a [u8], // Emprunte directement dans le buffer d'entree !
}

fn parse_frame<'a>(buf: &'a [u8]) -> Result<Frame<'a>, ParseError> {
    // A TOI DE JOUER :
    let frame = Frame {
	frame_type: 0,
	payload : buf,
    };
    // 1. Verifie que buf contient au moins le header (5 octets), sinon -> Err(ParseError::TooShort)
    if frame.payload.len() < 5 {
	return Result(Err(ParseError::TooShort));
    }
    // 2. Verifie le magic (0xCA, 0xFE), sinon -> Err(ParseError::InvalidMagic)
    // 3. Extrais frame_type et payload_len (indice : regarde u16::from_be_bytes)
    // 4. Verifie que le buffer contient assez d'octets pour le payload,
    //    sinon -> Err(ParseError::TruncatedPayload { expected: ..., actual: ... })
    // 5. Retourne Ok(Frame { ... }) avec la bonne sous-slice pour payload.
}

fn main() {
    let raw_ok = [0xCA, 0xFE, 0x01, 0x00, 0x03, 0xAA, 0xBB, 0xCC, 0x99]; // 0x99 = trailing byte ignore
    let raw_bad_magic = [0xDE, 0xAD, 0x01, 0x00, 0x00];
    let raw_truncated = [0xCA, 0xFE, 0x02, 0x00, 0x10, 0xAA];

    println!("OK: {:?}", parse_frame(&raw_ok));
    println!("Bad magic: {:?}", parse_frame(&raw_bad_magic));
    println!("Truncated: {:?}", parse_frame(&raw_truncated));
}
