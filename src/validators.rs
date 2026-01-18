use url::Url;

use crate::errors::AppError;

pub fn validate_and_normalize(input: &str) -> Result<String, AppError> {
    let mut parsed = Url::parse(input).map_err(|_| AppError::InvalidUrl)?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(AppError::InvalidScheme),
    }

    if parsed.path() == "/" {
        parsed.set_path("")
    }

    Ok(parsed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_http_url() {
        assert!(validate_and_normalize("http://example.com").is_ok());
    }

    #[test]
    fn valid_https_url() {
        assert!(validate_and_normalize("https://example.com").is_ok());
    }

    #[test]
    fn invalid_scheme() {
        assert!(matches!(
            validate_and_normalize("ftp://example.com"),
            Err(AppError::InvalidScheme)
        ));
    }

    #[test]
    fn invalid_url() {
        assert!(matches!(
            validate_and_normalize("example.com"),
            Err(AppError::InvalidUrl)
        ));
    }
}
