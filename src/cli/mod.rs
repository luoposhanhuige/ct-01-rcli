mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};
// q: what is difference between mod and use keyword?
// a: mod 是用于定义模块，use 是用于导入模块。
// q: 上述 mod csv, mod genpass, mod base64，是什么意思？
// a: 上述 mod csv、mod genpass、mod base64 是用于定义 csv、genpass、base64 这三个模块。
// q: 为什么要在这个文件中定义这三个模块？
// a: 在这个文件中定义这三个模块，是为了方便管理这三个模块，将这三个模块放在一个文件中，方便查看和维护。

use clap::Parser;

// 因为 Csv(CsvOpts) 和 Genpass(GenpassOpts) 没有 subcommand，所以直接引入 CsvOpts 和 GenpassOpts。
use self::{csv::CsvOpts, genpass::GenpassOpts};

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};
// q: what does pub use mean?
// a: pub use 是用于导出模块中的结构体和方法，使其可以在其他模块中使用。

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
    // 这里的 Subcommand 是一个 enum，用于存储子命令的值。
    // 在 main.rs 中调用，let opts = Opts::parse();
}
// q: what is #[derive(Debug, Parser)]?
// a: #[derive(Debug, Parser)] 是一个宏，用于为结构体或枚举类型实现 Debug 和 Parser 特性。
// q: what is Parser?
// a: Parser 是一个 trait，用于解析命令行参数。
// q: Parser 不是一个宏吗？
// a: Parser 是一个 trait，但是 #[derive(Parser)] 是一个宏，用于为结构体或枚举类型实现 Parser 特性。

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts), // 这是 clap mod 的特性用法之一，灵活将 enum 特性与 struct 特性结合起来，实现了更复杂的命令行参数解析。
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    // 验证，并对"-"和真实存在的文件的名称，做个to_string转换，然后return
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        // #[arg(short, long, value_parser = verify_file, default_value = "-")]
        Ok(filename.to_string())
        // filename.to_string() or filename.into()?
        // filename.to_string() 和 filename.into() 是等价的，都是将一个 &str 转换为 String 类型。
    } else {
        Err("Input file does not exist".to_string())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    // 验证，并将一个 &str 转换为 PathBuf 类型
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
        // q: what does path.into() mean?
        // a: path.into() 是一个 trait，可以将一个类型转换为另一个类型，这里是将一个 &str 转换为 PathBuf 类型。
    } else {
        Err("Path does not exist or is not a directory".to_string())
    }
}
// q: what is difference between Path and PathBuf?
// a: Path 是一个引用，用于表示一个路径，PathBuf 是一个类型，用于表示一个路径缓冲区。

// what is nextest?
// nextest 是一个 cargo 的插件，用于运行测试。
// how to install nextest?
// cargo install nextest
// how can i find it installed successfully?
// cargo --list
// i want to show all versions of cargo --list
// cargo --list --verbose
// how to know the version of nextest?
// cargo nextest --version
// what is the difference between cargo install and cargo add?
// cargo install 是用于安装二进制文件，cargo add 是用于添加依赖。

// cargo nextest run
// cargo nextest run -- test_verify_input_file
#[cfg(test)]
mod tests {
    use super::*;
    // what is super::*?
    // super::* 是一个通配符，用于导入父模块中的所有结构体和方法。
    // 在当前环境下，父模块中的所有结构体和方法，具体指的是哪些内容？
    // 在当前环境下，父模块中的所有结构体和方法，指的是 cli 模块中的所有结构体和方法。

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_file("*"),
            Err("Input file does not exist".to_string())
        );
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(
            verify_file("not-exist"),
            Err("Input file does not exist".to_string())
        );
    }
}
// what is #[cfg(test)]?
// #[cfg(test)] 是一个属性，用于指定测试模块，只有在测试模式下才会编译测试模块。
