use std::collections::HashMap;

pub fn add_strings(a: &str, b: &str, base: &str) -> Result<String, String> {
    let base_len = match u32::try_from(base.len()) {
        Ok(val) => val,
        Err(_) => return Err("Base too long.".to_string()),
    };

    let char_to_val: HashMap<char, u32> = base
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as u32))
        .collect();

    let mut a_digits = a.chars().rev();
    let mut b_digits = b.chars().rev();

    let mut result_chars = Vec::new();
    let mut carry = 0;

    loop {
        let digit_a = a_digits.next();
        let digit_b = b_digits.next();

        if digit_a.is_none() && digit_b.is_none() && carry == 0 {
            break;
        }

        let val_a = match digit_a {
            Some(d) => *char_to_val
                .get(&d)
                .ok_or(format!("Caractère invalide '{}' dans le premier nombre", d))?,
            None => 0,
        };
        let val_b = match digit_b {
            Some(d) => *char_to_val
                .get(&d)
                .ok_or(format!("Caractère invalide '{}' dans le second nombre", d))?,
            None => 0,
        };

        let sum = val_a + val_b + carry;
        let new_digit_val = sum % base_len;
        carry = sum / base_len;

        let new_char = base.chars().nth(new_digit_val as usize).unwrap();
        result_chars.push(new_char);
    }

    let result_string: String = result_chars.into_iter().rev().collect();

    Ok(if result_string.is_empty() {
        base.chars().next().unwrap_or('0').to_string()
    } else {
        result_string
    })
}
