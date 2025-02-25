// Normalizes string input by removing non-alphanumeric characters (spaces/punctuations)
// and converts uppercase letters to lowercase. (adds 32 to the char's ASCII value)
fn normalize(input: &str) -> String {
    let mut normalized = String::new();
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            if c.is_ascii_uppercase() {
                normalized.push((c as u8 + 32) as char);
            } 
            else {
                normalized.push(c);
            }
        }
    }
    return normalized;
}

// Returns true if the input string (that gets normalized) is a palindrome.
// Compares byte (ASCII value) with corresponding byte in the end. 
fn is_palindrome(input: &str) -> bool {
    let normalized = normalize(input);
    let bytes = normalized.as_bytes();
    let len = bytes.len();
    for i in 0..(len / 2) {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    return true;
}

// Tests (Type in cmd "cargo test -- --nocapture")
#[cfg(test)]
mod tests {
    use super::*;

    static test_cases: [(&str, bool); 30] = [
        // Expected true for valid palindromes 
        ("Racecar", true), ("22/02/2022", true), ("Was it a car or a cat I saw?", true), 
        ("A man, a plan, a canal, Panama!", true), ("A dog! A panic in a pagoda!", true), 
        ("A Toyota's a Toyota", true), ("Never odd or even", true),
        ("2220222", true), ("Dog, God", true), ("Reviver", true),

        // Expected false for invalid palindromes 
        ("Hello, world!", false), ("Palindrome", false), ("Testing", false), ("Computer Science", false), ("Banana", false),
        ("Chocolate", false), ("Vanilla", false), ("Ice cream", false), ("Hamburger", false), ("Cheeseburger", false),
        ("French Fries", false), ("Marvel", false), ("Rivals", false), ("Pokemon", false), ("Kansas City", false),
        ("Snowing", false), ("Spring", false), ("Rust", false), ("Rust is very cool", false), ("Rust is a programming language", false),
    ];

    #[test]
    fn test_palindromes() {
        let test_palindrome = &test_cases[0..30];
        for &(input, expect) in test_palindrome {
            let result = is_palindrome(input);
            assert_eq!(result, expect, "Failed on: {}", input);
        }
    }
}

