// cargo run -- base64
// cargo run -- base64 encode
// cargo run -- base64 decode
// cargo run -- base64 encode --format urlsafe -i Cargo.toml > fixtures/b64.txt
// cargo run -- base64 decode --format urlsafe -i fixtures/b64.txt
// cargo nextest run
// cargo nextest run -- test_process_encode

// cargo add base64?
// q: what is base64?
// a: base64 是一个编码算法，用于将二进制数据编码为文本数据。
// q: how to check whether the base64 is added to the dependencies?
// a: 可以查看 Cargo.toml 文件，查看是否有 base64 这个依赖。
// q: how to check whether the base64 is added through the command line?
// a: 可以查看 Cargo.toml 文件，查看是否有 base64 这个依赖。

use crate::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    // TODO: decoded data might not be string (but for this example, we assume it is)
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let mut reader = get_reader(input)?;
        let format = Base64Format::Standard;
        assert!(process_encode(&mut reader, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_process_decode() -> Result<()> {
        let input = "fixtures/b64.txt";
        let mut reader = get_reader(input)?;
        let format = Base64Format::UrlSafe;
        process_decode(&mut reader, format)?;

        Ok(())
    }
}
