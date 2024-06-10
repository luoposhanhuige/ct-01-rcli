use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a file to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a file from base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
// q: what does default_value = "-" mean?
// a: default_value = "-" 表示如果用户没有指定 input 参数，则 input 参数的值为 "-"，即标准输入。
// q: 标准输入？
// a: 标准输入是一个特殊的文件，用于接收用户输入的数据，通常是键盘输入。

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    // match s {
    //     "standard" => Ok(Base64Format::Standand),
    //     "urlsafe" => Ok(Base64Format::UrlSafe),
    //     _ => Err(anyhow::anyhow!("Invalid base64 format")),
    // }
    format.parse()
}

// q: the error message shows that: the trait bound 'Base64Format: FromStr' is not satisfied. why?
// a: 因为 Base64Format 没有实现 FromStr 这个 trait，所以不能使用 parse() 方法。
// q: 为什么必须实现 FromStr 这个 trait，才能使用 parse() 方法？
// a: 因为 parse() 方法是 FromStr 这个 trait 的一个方法，所以必须实现 FromStr 这个 trait，才能使用 parse() 方法。
// q: FromStr 这个 trait 的 parse() 方法，是用来做什么的？
// a: FromStr 这个 trait 的 parse() 方法，是用来将一个字符串解析为一个结构体的。

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> &'static str {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
// q: what is the difference between impl Fromstr and impl From?
// a: FromStr 是一个 trait，用于将一个字符串解析为一个结构体，而 From 是一个 trait，用于将一个结构体转换为另一个结构体。
// q: impl From<Base64Format> for &'static str, what does this mean?
// a: impl From<Base64Format> for &'static str 表示实现了 From trait，用于将一个 Base64Format 结构体转换为一个 &'static str 类型。

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}

// q: what is the difference between fmt::Display and fmt::Debug?
// a: fmt::Display 是一个 trait，用于将一个结构体格式化为一个字符串，而 fmt::Debug 是一个 trait，用于将一个结构体格式化为一个调试信息。

// q: what is the difference between write! and writeln!?
// a: write! 是一个宏，用于将一个结构体格式化为一个字符串，而 writeln! 是一个宏，用于将一个结构体格式化为一个字符串，并在最后添加一个换行符。

// q: what is the difference between writeln! and println!?
// a: The writeln! macro is similar to println!, but it appends a newline character (\n) to the output.

// The write! macro is used to write formatted text to a specified output stream without appending a newline character (\n) at the end.
// The print! macro is similar to write!, but it writes formatted text directly to the standard output (stdout) without the need to specify the output stream explicitly.

// The writeln! macro is similar to write!, but it appends a newline character (\n) to the output.
