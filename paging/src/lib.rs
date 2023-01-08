use std::cmp::max;
use nanoid::nanoid;

const URL_SAFE_ENGINE: base64::engine::fast_portable::FastPortable =
    base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD);

const TOKEN_LENGTH: usize = 48;
const SEPARATOR: &str = ".";

pub type Token = String;

pub fn encode_token<T: std::fmt::Display>(value: T) -> String {
    let pad_len = max(TOKEN_LENGTH - value.to_string().len(), 0);
    let v = format!("{}{}{}", value, SEPARATOR, nanoid!(pad_len));
    base64::encode_engine(v.as_bytes(), &URL_SAFE_ENGINE)
}

pub fn decode_token<T: std::str::FromStr>(token: Token) -> Option<T> {
    let decoded = base64::decode_engine(token.as_bytes(), &URL_SAFE_ENGINE)
        .ok()?
        .iter()
        .map(|&s| s as char)
        .collect::<String>();
    let key = decoded.split(SEPARATOR).next()?;
    T::from_str(key).ok()
}

pub struct Paging {
    pub token: Token,
    pub limit: usize,
}

impl Default for Paging {
    fn default()-> Self {
        Self {
            token: "".to_string(),
            limit: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_token() {
        let token = encode_token("token".to_string());
        assert_eq!(decode_token::<String>(token).unwrap(), "token");
    }

    #[test]
    fn default_paging() {
        let paging = Paging::default();
        assert_eq!(paging.token, "".to_string());
        assert_eq!(paging.limit, 100);
    }
}
