use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub enum OtpCharSet {
    Numeric,
    Alphanumeric,
    AlphanumericSpecialChars,
    Custom(String),
}

pub fn generate_otp(length: usize, char_set_type: OtpCharSet) -> Result<String, &'static str> {
    let mut rng: ThreadRng = rand::thread_rng();

    let chars: Vec<char> = match char_set_type {
        OtpCharSet::Numeric => "0123456789".chars().collect(),
        OtpCharSet::Alphanumeric => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        OtpCharSet::AlphanumericSpecialChars => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect(),
        OtpCharSet::Custom(s) => {
            if s.is_empty() {
                return Err("Custom character set cannot be empty");
            }
            s.chars().collect()
        }
    };

    if chars.is_empty() {
        return Err("Effective character set is empty");
    }

    let otp: String = (0..length)
        .map(|_| {
            *chars
                .choose(&mut rng)
                .expect("Character set should not be empty at this point")
        })
        .collect();

    Ok(otp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_otp() {
        let otp = generate_otp(6, OtpCharSet::Numeric).unwrap();
        assert_eq!(otp.len(), 6);
        assert!(otp.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn generate_alphanumeric_otp() {
        let otp = generate_otp(8, OtpCharSet::Alphanumeric).unwrap();
        assert_eq!(otp.len(), 8);
        assert!(otp.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn generate_alphanumeric_special_otp() {
        let otp = generate_otp(10, OtpCharSet::AlphanumericSpecialChars).unwrap();
        assert_eq!(otp.len(), 10);
        assert!(otp
            .chars()
            .any(|c| !c.is_alphanumeric() && !c.is_whitespace()));
    }

    #[test]
    fn generate_custom_otp_valid() {
        let custom_set = "ABC123xyz!@#".to_string();
        let otp = generate_otp(7, OtpCharSet::Custom(custom_set.clone())).unwrap();
        assert_eq!(otp.len(), 7);
        assert!(otp.chars().all(|c| custom_set.contains(c)));
    }

    #[test]
    fn generate_custom_otp_empty_set() {
        let result = generate_otp(5, OtpCharSet::Custom("".to_string()));
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Custom character set cannot be empty"));
    }

    #[test]
    fn generate_zero_length_otp() {
        let otp = generate_otp(0, OtpCharSet::Numeric).unwrap();
        assert_eq!(otp.len(), 0);
        assert_eq!(otp, "");
    }

    #[test]
    fn generate_custom_otp_only_one_char() {
        let custom_set = "A".to_string();
        let otp = generate_otp(5, OtpCharSet::Custom(custom_set.clone())).unwrap();
        assert_eq!(otp.len(), 5);
        assert_eq!(otp, "AAAAA");
    }

    fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[test]
    fn test_add_function() {
        assert_eq!(add(2, 2), 4);
    }
}
