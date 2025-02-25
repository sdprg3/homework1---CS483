fn validate_username(username: &str) -> bool {
    !username.contains("=") && !username.contains("\'")
}

fn validate_password(password: &str) -> bool {
    let allowed_specials = ["~", "!", "@", "$", "%", "^", "&"];
    let (mut has_digit, mut has_upper, mut has_lower, mut has_special) = (false, false, false, false);
    if password.len() >= 8 { 
        for c in password.chars() {
            if c.is_ascii_digit() { has_digit = true; }
            else if c.is_ascii_uppercase() { has_upper = true; }
            else if c.is_ascii_lowercase() { has_lower = true; }
            else if allowed_specials.contains(&c) { has_special = true; }
            else { return false; }
        }
    }
    return has_digit && has_upper && has_lower && has_special
}

fn generate_SQL(username: &str, password: &str) -> String {
    if !validate_username(username) {
        return format!("Error: Username '{}' doesn't follow user account policy", username);
    }
    if !validate_password(password) {
        return format!("Error: Password for '{}' doesn't follow user account policy", password);
    }

    return format!("SELECT * \nFROM accounts \nWHERE userid = '{}' AND pswd = '{}'", username, password);
}

// Tests (Type in cmd "cargo test -- --nocapture")
#[cfg(test)]
mod tests {
    use super::*;

    static accounts: [(&str, &str); 30] = [
        // 10 Valid accounts
        ("Username", "!Password1"), ("User_Name", "@Password2"), ("User1", "$Password3"), ("/User2/", "P@ssw0rd"), ("User Name", "1Password!"),
        ("Admin", "%Test1234"), ("Test", "Welcome@2025"), ("123456789", "Secure~K3y"), ("User!@#$%^&*", "!Qwerty123"), ("Name-valid", "T3st~!@$%^&"),
        // 20 Invalid accounts (username contains denylisted characters or/and password fails policy)
        ("User=name", "!Password1"), ("User==Name", "@Password2"), ("User'name", "$Password3"), ("'UserName'", "P@ssw0rd"), ("==Username==", "1Password!"),
        ("Username1", "Password1"),  ("Username2", "!password1"), ("Username3", "!PASSWORD1"), ("Username4", "!Password"), ("Username5", "!Pass1"),
        ("Name=Good", "Password"), ("'='", "Admin"), ("No'Good", "Test"), ("Inva'lid", "1234567890"), ("Name='Bad", "abcdefgh"),
        ("===---___", "ABCDEFGH"), ("'_'", "Test_Pass"), ("('>')", "PASS WORD"), ("Wrong'Name", "~!@#$%^&"), ("=====", "#Password1"),

    ];

    #[test]
    fn test_valid_credentials() {
        let valid_accounts = &accounts[0..10];
        
        for &(username, password) in valid_accounts {
            let expected = format!(
                "SELECT * \nFROM accounts \nWHERE userid = '{}' AND pswd = '{}'",
                username, password
            );
            assert_eq!(generate_SQL(username, password), expected);
        }
    }

    #[test]
    fn test_invalid_credentials() {
        let invalid_accounts = &accounts[10..30];
        for &(username, password) in invalid_accounts {
            let result = generate_SQL(username, password);
            assert!(
                result.starts_with("Error:"),
                "Expected an error for account ('{}', '{}'), got: \n{}",
                username, password, result
            );
        }
    }
}