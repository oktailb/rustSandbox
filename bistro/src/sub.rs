use std::collections::HashMap;

pub fn is_ge(a: &str, b: &str, char_to_val: &HashMap<char, i32>) -> bool {
    if a.len() > b.len() {
        return true;
    }
    if a.len() < b.len() {
        return false;
    }

    for (char_a, char_b) in a.chars().zip(b.chars()) {
        let val_a = char_to_val[&char_a];
        let val_b = char_to_val[&char_b];
        if val_a > val_b {
            return true;
        }
        if val_a < val_b {
            return false;
        }
    }

    true
}

pub fn sub_strings(a: &str, b: &str, base_str: &str) -> Result<String, String> {
    let base = i32::try_from(base_str.len()).map_err(|_| "Base too long.".to_string())?;
    if base < 2 {
        return Err("La base doit être supérieure à 1.".to_string());
    }

    let char_to_val: HashMap<char, i32> = base_str
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as i32))
        .collect();
    let zero_char = base_str.chars().next().unwrap();

    let a_is_ge_b = is_ge(a, b, &char_to_val);
    let (val1, val2) = if a_is_ge_b { (a, b) } else { (b, a) };

    let v1_digits: Vec<i32> = val1.chars().rev().map(|c| char_to_val[&c]).collect();
    let v2_digits: Vec<i32> = val2.chars().rev().map(|c| char_to_val[&c]).collect();

    let mut result_vals = Vec::new();
    let mut borrow = 0;

    for i in 0..v1_digits.len() {
        let digit1 = v1_digits[i];
        let digit2 = *v2_digits.get(i).unwrap_or(&0);

        let mut diff = digit1 - digit2 - borrow;

        if diff < 0 {
            diff += base;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result_vals.push(diff);
    }

    if let Some(first_digit_pos) = result_vals.iter().rposition(|&d| d != 0) {
        let mut final_string: String = result_vals[..=first_digit_pos]
            .iter()
            .rev()
            .map(|&val| base_str.chars().nth(val as usize).unwrap())
            .collect();

        if !a_is_ge_b {
            final_string.insert(0, '-');
        }
        Ok(final_string)
    } else {
        Ok(zero_char.to_string())
    }
}
