use clap::Parser;
use std::{fmt, str::FromStr};

use crate::CmdExector;

use super::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    // 输出什么格式的文件？
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)] // verify_input_file 函数，写在下方
    // 不设缺省值的话，cargo run -- csv -i test.csv，需要指定 -i 参数
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,
    // q: why does the output field have the Option type?
    // a: output 字段有 Option 类型，是因为 output 字段是可选的，如果用户没有指定 output 参数，则 output 字段的值为 None，如果用户指定了 output 参数，则 output 字段的值为 Some(output)。
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    // parse_format 函数，写在下方
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)] // 避免 -h 与 默认的 help 冲突，所以去掉 short
    pub header: bool,
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 因为枚举类型中的单个变体，就是Csv(CsvOpts)形式。再从此映射到CsvOpts这个struct的多个fields
        let output = if let Some(output) = self.output {
            output
        } else {
            // "output.json".into()
            format!("output.{}", self.format)
            // q: what does this mean?
            // a: 这里的 into() 是一个 trait，可以将一个类型转换为另一个类型，这里是将一个 &str 转换为 String 类型。
            // q: 怎么知道是要转换为 String 类型的？
            // a: 因为 "output.json" 是一个字符串字面量，所以这里的 into() 会将其转换为 String 类型。
            // q: 如要要转换为其他类型，如何处理？
            // a: 如果是其他类型，就需要使用 as 关键字，如：output as i32，这样就将 output 转换为 i32 类型。
        };
        crate::process_csv(&self.input, output, self.format)?; // 此处申明的 opts，与上面的 opts，容易产生混淆，这是课堂中老师忽略的地方。

        Ok(())
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

// q: what is the connection between the above two impl block and the fn parse_format()?
// a: The above two impl blocks are used to convert the OutputFormat enum variant to a string, and the fn parse_format() is used to convert a string to an OutputFormat enum variant. The connection between them is that they are used together to convert between the OutputFormat enum variant and a string.

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
        //  Into::<&str>::into(*self) means that we're calling the into method from the Into trait to convert the enum variant self into a &str. This is used within the fmt method of the Display implementation to format the enum variant as a string when displaying it.
    }
}
