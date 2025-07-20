use crate::sub::{is_ge, sub_strings};
use std::collections::HashMap;

fn perform_division(
    dividend: &str,
    divisor: &str,
    base_str: &str,
) -> Result<(String, String), String> {
    let char_to_val: HashMap<char, i32> = base_str
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as i32))
        .collect();
    let zero_char = base_str.chars().next().unwrap();

    // --- Edge Case Handling ---
    if divisor == zero_char.to_string() {
        return Err("Division by zero.".to_string());
    }

    if !is_ge(dividend, divisor, &char_to_val) {
        // If dividend < divisor, quotient is "0" and remainder is the dividend.
        return Ok((zero_char.to_string(), dividend.to_string()));
    }

    // --- Long Division Algorithm (The one and only implementation) ---
    let mut quotient = String::new();
    let mut current_part = String::new();

    for digit_char in dividend.chars() {
        current_part.push(digit_char);
        // Clean up leading zeros, e.g., "05" -> "5"
        while current_part.starts_with(zero_char) && current_part.len() > 1 {
            current_part.remove(0);
        }

        let mut q_digit = 0;
        while is_ge(&current_part, divisor, &char_to_val) {
            // Your sub_strings function already returns a Result<String, String>
            // but you only need the positive result. Let's assume it's the first element.
            current_part = sub_strings(&current_part, divisor, base_str)?;
            q_digit += 1;
        }
        quotient.push(base_str.chars().nth(q_digit as usize).unwrap());
    }

    // --- Final Formatting ---
    while quotient.starts_with(zero_char) && quotient.len() > 1 {
        quotient.remove(0);
    }

    // The final remainder might be empty if it was "0" and got cleaned.
    if current_part.is_empty() {
        current_part.push(zero_char);
    }

    Ok((quotient, current_part))
}

// public wrapper for division
pub fn div_strings(dividend: &str, divisor: &str, base_str: &str) -> Result<String, String> {
    // Call the worker function once.
    let result = perform_division(dividend, divisor, base_str);
    // Return only the quotient (the first part of the tuple).
    result.map(|(quotient, _remainder)| quotient)
}

// public wrapper for modulo
pub fn mod_strings(dividend: &str, divisor: &str, base_str: &str) -> Result<String, String> {
    // Call the same worker function once.
    let result = perform_division(dividend, divisor, base_str);
    // Return only the remainder (the second part of the tuple).
    result.map(|(_quotient, remainder)| remainder)
}
