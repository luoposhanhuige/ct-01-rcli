use std::{fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a new key pair")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    //, default_value = "-"，会与input的default_value冲突，共同从stdin读取
    pub key: String,

    #[arg(short, long)]
    pub sig: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf, // 一个路径
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    ED25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
    // q: what is .parse()? which struct or trait does this function belong to?
    // q: what is .parse()?
    // a: .parse() is a method that converts a string into another type.
    // q: which struct or trait does the .parse() belong to?
    // a: The .parse() method belongs to the FromStr trait.
    // q: in this case, the .parse() converts a string to TextSignFormat?
    // a: Yes, in this case, the .parse() converts a string to TextSignFormat.
    // .parse 的定义，内部做了一个 FromStr::from_str(self) 的调用：
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
    //     FromStr::from_str(self)
    // }
}
// q: what does this function parse_format do?
// a: This function converts a string into a TextSignFormat enum.

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> &'static str {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::ED25519 => "ed25519",
        }
    }
}
// 1, Explicit Conversion using from:
// let format = TextSignFormat::Blake3;
// let format_str: &'static str = From::from(format);

// 2, Implicit Conversion using into:
// let format = TextSignFormat::ED25519;
// let format_str: &'static str = format.into();

// 3,
// fn print_format(format: &'static str) {
//     println!("TextSignFormat is: {}", format);
// }
// let format = TextSignFormat::Blake3;
// print_format(format.into()); // Automatically converts using From

// 4,
// let format = TextSignFormat::Blake3;
// let message = format!("Selected format: {}", <&'static str>::from(format));
// println!("{}", message);

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::ED25519 => write!(f, "ed25519"),
        }
    }
}
