# VortexOTP

VortexOTP is a simple, flexible, and dependency-light One-Time Password (OTP) generation library for Rust. It allows you to generate OTPs of various lengths and character sets, including numeric, alphanumeric, alphanumeric with special characters, or even your own custom character sets.

## Features

- Generate OTPs of a specified length.
- Choose from predefined character sets:
  - Numeric
  - Alphanumeric (uppercase and lowercase)
  - Alphanumeric with common special characters
- Provide a custom character set for OTP generation.
- Minimal dependencies (only `rand`).
- Clear error handling for invalid inputs (e.g., empty custom character set).

## Installation

Add VortexOTP to your `Cargo.toml`:

```toml
[dependencies]
VortexOTP = "0.1.0"
```

Or, if you are using it from a local path or git repository:

## Usage

Here's a quick example of how to use VortexOTP:

```rust
use VortexOTP::{generate_otp, OtpCharSet};

fn main() {
    match generate_otp(6, OtpCharSet::Numeric) {
        Ok(otp) => println!("Numeric OTP: {}", otp),
        Err(e) => eprintln!("Error: {}", e),
    }

    match generate_otp(8, OtpCharSet::Alphanumeric) {
        Ok(otp) => println!("Alphanumeric OTP: {}", otp),
        Err(e) => eprintln!("Error: {}", e),
    }

    match generate_otp(10, OtpCharSet::AlphanumericSpecialChars) {
        Ok(otp) => println!("Alphanumeric Special OTP: {}", otp),
        Err(e) => eprintln!("Error: {}", e),
    }

    let custom_chars = "ABCDEF123456!@#".to_string();
    match generate_otp(7, OtpCharSet::Custom(custom_chars)) {
        Ok(otp) => println!("Custom OTP: {}", otp),
        Err(e) => eprintln!("Error: {}", e),
    }

    match generate_otp(5, OtpCharSet::Custom("".to_string())) {
        Ok(otp) => println!("This won't print: {}", otp),
        Err(e) => eprintln!("Error generating custom OTP: {}", e),
    }
}
```

## Character Sets

- `OtpCharSet::Numeric`: `0123456789`
- `OtpCharSet::Alphanumeric`: `0-9`, `a-z`, `A-Z`
- `OtpCharSet::AlphanumericSpecialChars`: `0-9`, `a-z`, `A-Z`, `!@#$%^&*()_+-=[]{}|;:,.<>?`
- `OtpCharSet::Custom(String)`: Any `String` you provide.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

VortexOTP is licensed under the MIT License. See the `LICENSE` file (you would need to create this file if you choose MIT and want to include the full license text) in the repository for more details.
