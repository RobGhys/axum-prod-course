use crate::config;
use super::{encrypt_into_b64u, EncryptContent, Error, Result};

/// encrypt password with a default scheme
pub fn encrypt_pw(encrypt_content: &EncryptContent) -> Result<String> {
    let key = &config().PWD_KEY;

    let encrypted = encrypt_into_b64u(key, encrypt_content)?;

    // encryption scheme 01
    Ok(format!("#01{encrypted}"))
}

pub fn validate_pw(
    encrypt_content: &EncryptContent,
    pwd_ref: &str
) -> Result<()> {
    let pwd = encrypt_pw(encrypt_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PasswordNotMatching)
    }
}