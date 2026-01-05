mod error;

use base64::{alphabet, engine, Engine};
use base64::engine::general_purpose;
pub use self::error::{Result, Error};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    // TODO check if unwrap is safe
    time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_|
        Error::DateFailParse(moment.to_string()))
}

pub fn b64u_encode(content: &str) -> String {
    let b64_u = URL_SAFE_NO_PAD.encode(&content);
    b64_u
}

pub fn b64u_decode(b64u: &str) -> Result<String> {
    let decoded_string = engine::GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD)
        .decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB64uDecode)?;

    Ok(decoded_string)
}