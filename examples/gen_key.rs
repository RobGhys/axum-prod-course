use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits = 64 bytes
    rand::rng().fill_bytes(&mut key);
    println!("\nGenerated key for HMAC: \n{key:?}");

    let b64_u = URL_SAFE_NO_PAD.encode(&key);
    println!("\nKey b64u encoded:\n {b64_u}");

    Ok(())
}