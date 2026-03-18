use crate::error::AppError;

#[tracing::instrument]
pub fn normalize_phone(phone: &str) -> Result<String, AppError> {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

    let normalized = if digits.len() == 10 {
        format!("+91{digits}")
    } else if digits.len() == 11 && digits.starts_with('0') {
        format!("+91{}", &digits[1..])
    } else if digits.len() == 12 && digits.starts_with("91") {
        format!("+{digits}")
    } else if digits.len() == 13 && digits.starts_with("091") {
        format!("+91{}", &digits[3..])
    } else {
        return Err(AppError::BadRequest(format!(
            "invalid phone number: {phone}"
        )));
    };

    if normalized.len() != 13 || !normalized.starts_with("+91") {
        return Err(AppError::BadRequest(format!(
            "invalid phone number: {phone}"
        )));
    }

    Ok(normalized)
}

#[tracing::instrument]
pub fn validate_email(email: &str) -> bool {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return false;
    }

    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    if !domain.contains('.') {
        return false;
    }

    let domain_parts: Vec<&str> = domain.rsplitn(2, '.').collect();
    if domain_parts.len() != 2 || domain_parts[0].is_empty() || domain_parts[1].is_empty() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_10_digit() {
        assert_eq!(normalize_phone("9876543210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_with_leading_zero() {
        assert_eq!(normalize_phone("09876543210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_with_country_code() {
        assert_eq!(normalize_phone("919876543210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_with_plus_country_code() {
        assert_eq!(normalize_phone("+919876543210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_with_spaces_and_dashes() {
        assert_eq!(normalize_phone("+91 98765-43210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_with_zero_prefix_country_code() {
        assert_eq!(normalize_phone("0919876543210").ok(), Some("+919876543210".to_string()));
    }

    #[test]
    fn test_normalize_invalid_too_short() {
        assert!(normalize_phone("12345").is_err());
    }

    #[test]
    fn test_normalize_invalid_too_long() {
        assert!(normalize_phone("12345678901234567").is_err());
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com"));
    }

    #[test]
    fn test_validate_email_no_at() {
        assert!(!validate_email("userexample.com"));
    }

    #[test]
    fn test_validate_email_no_domain_dot() {
        assert!(!validate_email("user@example"));
    }

    #[test]
    fn test_validate_email_empty() {
        assert!(!validate_email(""));
    }
}
