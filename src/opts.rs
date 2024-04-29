use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts), // 这是 clap mod 的特性用法之一，灵活将 enum 特性与 struct 特性结合起来，实现了更复杂的命令行参数解析。
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)] // verify_input_file 函数，写在下方
    // 不设缺省值的话，cargo run -- csv -i test.csv，需要指定 -i 参数
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    // parse_format 函数，写在下方
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)] // 避免 -h 与 默认的 help 冲突，所以去掉 short
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("Input file does not exist".to_string())
    }
}

// fn parse_format(format: &str) -> Result<OutputFormat, String> {
//     match format.to_lowercase().as_str() {
//         "json" => Ok(OutputFormat::Json),
//         "yaml" => Ok(OutputFormat::Yaml),
//         _ => Err("Invalid format".to_string()),
//     }
// }

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // q: .parse 与 FromStr trait 的关系？
    // a: .parse 是 FromStr trait 的一个方法，所以这里的 format.parse() 实际上是调用了 OutputFormat::from_str(format) 方法。
    // format.parse().map_err(|_| anyhow::anyhow!("Invalid format")) // 因为在 impl FromStr for OutputFormat 中，已经指定了 type Err = anyhow::Error;

    // format.parse::<OutputFormat>() // OutputFormat 可以实现自动推导，所以，可以去掉。
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
        //  Into::<&str>::into(*self) means that we're calling the into method from the Into trait to convert the enum variant self into a &str. This is used within the fmt method of the Display implementation to format the enum variant as a string when displaying it.
    }
}
