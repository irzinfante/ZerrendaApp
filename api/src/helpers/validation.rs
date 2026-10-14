const EMAIL_MAX_LEN: usize = 254;
const EMAIL_LOCAL_MIN_LEN: usize = 2;
const EMAIL_LOCAL_MAX_LEN: usize = 64;
const EMAIL_LABEL_MAX_LEN: usize = 63;
const EMAIL_LOCAL_SPECIAL_CHARS: &str = ".-_+";

pub const PASSWORD_MIN_CHARS: usize = 8;
// bcrypt only uses the first 72 bytes of the password.
pub const PASSWORD_MAX_BYTES: usize = 72;

/// Trims and lowercases the email and checks its format. The format is never
/// looser than `emailPattern` in the app, so the app doesn't block an email
/// that the API would accept.
/// Returns `None` if the email is not valid.
pub fn normalize_email(email: &str) -> Option<String> {
    let email = email.trim().to_lowercase();
    if email.len() > EMAIL_MAX_LEN {
        return None;
    }

    let (local, domain) = email.split_once('@')?;
    if !is_valid_email_local(local) || !is_valid_email_domain(domain) {
        return None;
    }

    Some(email)
}

fn is_valid_email_local(local: &str) -> bool {
    local.len() >= EMAIL_LOCAL_MIN_LEN
        && local.len() <= EMAIL_LOCAL_MAX_LEN
        && local.starts_with(|c: char| c.is_ascii_alphanumeric())
        && local.ends_with(|c: char| c.is_ascii_alphanumeric())
        && !local.contains("..")
        && local
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || EMAIL_LOCAL_SPECIAL_CHARS.contains(c))
}

fn is_valid_email_domain(domain: &str) -> bool {
    let labels: Vec<&str> = domain.split('.').collect();
    let is_valid_label = |label: &&str| {
        !label.is_empty()
            && label.len() <= EMAIL_LABEL_MAX_LEN
            && !label.starts_with('-')
            && !label.ends_with('-')
            && !label.contains("--")
            && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
    };
    let is_valid_tld = |tld: &&str| tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic());

    labels.len() >= 2
        && labels.iter().all(is_valid_label)
        && labels.last().is_some_and(is_valid_tld)
}

pub fn is_valid_password(password: &str) -> bool {
    password.chars().count() >= PASSWORD_MIN_CHARS && password.len() <= PASSWORD_MAX_BYTES
}
