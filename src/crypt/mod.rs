mod error;
pub mod pwd;
pub mod token;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptContent {
    pub content: String, // Clear content
    pub salt: String, // Clear salt
}

/// Encrypt a `enc_content` with the `key`
pub fn  encrypt_into_b64u(
    key: &[u8],
    enc_content: &EncryptContent
) -> Result<String> {
    let EncryptContent { content, salt } = enc_content;

    // Create HMAC-SHA-512 from key
    let mut hmac_sha512 =
    Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    // Add content
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());


    // Finalize and b64u encode
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    //let result = base64::encode(&result_bytes);
    let result = URL_SAFE_NO_PAD.encode(&result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rand::RngCore;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
        rand::rng().fill_bytes(&mut fx_key);

        let fx_enc_content = EncryptContent {
            content: "hello world".to_string(),
            salt: "some pepper".to_string()
        };

        let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        // -- Exec
        let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;
        println!("->> {res}");

        assert_eq!(res, fx_res);

        Ok(())
    }
}