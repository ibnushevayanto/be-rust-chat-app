pub fn get_canonical_email(email: &str) -> String {
    let email = email.trim().to_lowercase();

    if let Some((local_part, domain)) = email.split_once('@') {
        let domain_normalized = if domain == "googlemail.com" {
            "gmail.com"
        } else {
            domain
        };

        let local_normalized = if domain_normalized == "gmail.com" {
            let without_alias = local_part.split('+').next().unwrap_or(local_part);
            without_alias.replace('.', "")
        } else {
            let without_alias = local_part.split('+').next().unwrap_or(local_part);
            without_alias.to_string()
        };

        format!("{}@{}", local_normalized, domain_normalized)
    } else {
        email
    }
}
