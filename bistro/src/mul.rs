use std::collections::HashMap;

pub fn mul_strings(a: &str, b: &str, base_str: &str) -> Result<String, String> {
    let base = match u32::try_from(base_str.len()) {
        Ok(0) | Ok(1) => return Err("Invalid base.".to_string()),
        Ok(val) => val,
        Err(_) => return Err("Unreachable base.".to_string()),
    };

    let char_to_val: HashMap<char, u32> = base_str
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as u32))
        .collect();

    let zero_char = base_str.chars().next().unwrap();
    if a == zero_char.to_string() || b == zero_char.to_string() {
        return Ok(zero_char.to_string());
    }

    let a_vals: Vec<u32> = a
        .chars()
        .rev()
        .map(|c| *char_to_val.get(&c).unwrap())
        .collect();
    let b_vals: Vec<u32> = b
        .chars()
        .rev()
        .map(|c| *char_to_val.get(&c).unwrap())
        .collect();

    let mut result = vec![0; a_vals.len() + b_vals.len()];

    // --- Phase 1: Multiplication ---
    for (i, &val_a) in a_vals.iter().enumerate() {
        for (j, &val_b) in b_vals.iter().enumerate() {
            result[i + j] += val_a * val_b;
        }
    }

    // --- Phase 2: Carry propagation
    let mut carry = 0;
    for digit in result.iter_mut() {
        let sum = *digit + carry;
        *digit = sum % base;
        carry = sum / base;
    }

    // --- Phase 3: Format ---
    if let Some(first_digit_pos) = result.iter().rposition(|&d| d != 0) {
        let final_digits = &result[..=first_digit_pos];
        let result_string = final_digits
            .iter()
            .rev()
            .map(|&val| base_str.chars().nth(val as usize).unwrap())
            .collect();
        Ok(result_string)
    } else {
        Ok(zero_char.to_string())
    }
}
