use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    if has_uppercase && has_lowercase && has_digit && has_special {
        Ok(())
    } else {
        let mut error = ValidationError::new("password_strength");
        error.message = Some(
            "Password harus mengandung kombinasi huruf besar, huruf kecil, angka, dan simbol"
                .into(),
        );
        Err(error)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDTO {
    #[validate(length(min = 5, message = "Username must be at least 5 characters long"))]
    pub username: String,

    #[validate(
        length(
            min = 8,
            max = 72,
            message = "Password must be between 8 and 72 characters long"
        ),
        custom(function = "validate_password_strength")
    )]
    pub password: String,

    #[validate(must_match(
        other = "password",
        message = "Confirm password does not match password"
    ))]
    pub confirm_password: String,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(
        min = 11,
        message = "Phone number must be at least 11 charachters long"
    ))]
    pub phone: String,

    #[validate(length(min = 2, message = "Organization must be at least 2 characters long"))]
    pub organization: String,

    #[validate(length(min = 3, message = "Device type must be at least 4 characters long"))]
    pub device_type: String,

    #[validate(length(
        min = 3,
        message = "Device location must be at least 3 characters long"
    ))]
    pub device_location: String,

    #[validate(length(min = 3, message = "Device name must be at least 3 characters long"))]
    pub device_name: String,
}
