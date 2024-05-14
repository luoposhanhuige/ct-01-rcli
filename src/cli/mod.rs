mod base64;
mod csv;
mod genpass;

use std::path::Path;
// q: what is difference between mod and use keyword?
// a: mod 是用于定义模块，use 是用于导入模块。
// q: 上述 mod csv, mod genpass, mod base64，是什么意思？
// a: 上述 mod csv、mod genpass、mod base64 是用于定义 csv、genpass、base64 这三个模块。
// q: 为什么要在这个文件中定义这三个模块？
// a: 在这个文件中定义这三个模块，是为了方便管理这三个模块，将这三个模块放在一个文件中，方便查看和维护。

use clap::Parser;

use self::{csv::CsvOpts, genpass::GenpassOpts};
// q: why need to use the above code to import CsvOpts and GenpassOpts while the whole module of both csv and genpass are imported?
// a: 这里需要使用 use self::{csv::CsvOpts, genpass::GenpassOpts}; 来导入 CsvOpts 和 GenpassOpts，是因为 CsvOpts 和 GenpassOpts 是在 csv 和 genpass 模块中定义的，而不是在 cli 模块中定义的。
// q: "mod csv; mod genpass;" and "use self::{csv::CsvOpts, genpass::GenpassOpts};" has the same functionality?
// a: "mod csv; mod genpass;" 是用于导入 csv 和 genpass 模块，"use self::{csv::CsvOpts, genpass::GenpassOpts};" 是用于导入 CsvOpts 和 GenpassOpts 结构体，两者的功能是不同的。
// q: 导入模块，难道不是把该模块中的结构体和方法，都导入了吗？
// a: 导入模块只是导入了模块，没有导入模块中的结构体和方法，如果要导入模块中的结构体和方法，需要使用 use 关键字。
// q: 那导入模块的意义在哪?
// a: 导入模块的意义在于，可以将模块中的结构体和方法导入到当前模块中，方便使用。

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
// q: what does pub use mean?
// a: pub use 是用于导出模块中的结构体和方法，使其可以在其他模块中使用。
// q: pub use or pub, what is the difference between them?
// a: pub use 是用于导出模块中的结构体和方法，使其可以在其他模块中使用，pub 是用于定义模块，使其可以被其他模块引用。
// q: 是不是 pub use 只能在 mod.rs 文件中使用？ 而 pub 可以在任何地方使用？
// a: 是的，pub use 只能在 mod.rs 文件中使用，pub 可以在任何地方使用。

// q: pub use 可以在 除了 mod.rs lib.rs main.rs 之外的文件中使用吗？
// a: 不可以，pub use 只能在 mod.rs、lib.rs、main.rs 这三种类型的文件中使用。
// q: 以cli目录为例，csv.rs genpass.rs 和 mod.rs 都分别是什么类型的文件？
// a: csv.rs、genpass.rs 和 mod.rs 都是模块文件。
// q: 但 mod.rs 明显比 csv.rs genpass.rs 特殊。
// a: 是的，mod.rs 是一个特殊的模块文件，用于定义模块的根模块，而 csv.rs 和 genpass.rs 是普通的模块文件，用于定义模块的子模块。
// q: 为什么 mod.rs 是特殊的？
// a: mod.rs 是特殊的，是因为 mod.rs 是模块的根模块，用于定义模块的结构，而 csv.rs 和 genpass.rs 是模块的子模块，用于定义模块的子结构。
// q: 为什么要有根模块和子模块的区分？
// a: 有根模块和子模块的区分，是为了方便管理模块的结构，将模块的结构分为根结构和子结构，使模块的结构更加清晰。

// q: what is the functionality of the above code?
// a: 这段代码用于导入 csv、genpass、base64 模块，并将 OutputFormat、Base64Format、Base64SubCommand 导出。
// q: could you explain the above question and answer in english?
// a: The above code is used to import the csv, genpass, and base64 modules, and export OutputFormat, Base64Format, and Base64SubCommand.
// q: what do you mean export? what is the destination of it explort?
// a: Export means that the OutputFormat, Base64Format, and Base64SubCommand are available to other modules, and can be used in other modules.
// q: so, if i want to export something, i need to use pub keyword? or put use keyword in the root of the module? what is the difference between put and put use?
// a: Yes, if you want to export something, you need to use the pub keyword, and put the use keyword in the root of the module. The difference between put and put use is that put use is used to import modules, and put is used to define modules.
// q: you mean the "pub use self::{ base64::{Base64Format, Base64SubCommand}, csv::OutputFormat,};" is used to export the OutputFormat, Base64Format, and Base64SubCommand, right?" but what is the destination of the export?
// a: Yes, the "pub use self::{ base64::{Base64Format, Base64SubCommand}, csv::OutputFormat,};" is used to export the OutputFormat, Base64Format, and Base64SubCommand, and the destination of the export is the root of the module.
// q: in current situation, which one is the root of the module?
// a: In the current situation, the root of the module is the cli module.
// q: what is the difference between the root of the module and the root of the crate?
// a: The root of the module is the module that contains the other modules, and the root of the crate is the main module that contains the other modules.
// q: what is the purpose of lib.rs and which is connection between lib.rs and the root of the crate?
// a: The purpose of lib.rs is to define the main module of the crate, and the connection between lib.rs and the root of the crate is that lib.rs is the root of the crate.

// q: what is the difference between use self::csv::CsvOpts and use crate::cli::csv::CsvOpts?
// a: use self::csv::CsvOpts 是相对路径，use crate::cli::csv::CsvOpts 是绝对路径，两者的功能是一样的，只是写法不同。
// q: how could compiler know the csv in crate::cli::csv::CsvOpts is a directory or a module?
// a: 编译器会根据当前目录结构，自动识别 csv 是一个目录还是一个模块，如果 csv 是一个目录，则会自动查找该目录下的 mod.rs 文件，如果 csv 是一个模块，则会自动查找 csv.rs 文件。

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
    // 这里的 Subcommand 是一个 enum，用于存储子命令的值。
    // 在 main.rs 中调用，let opts = Opts::parse();
}
// q: is it true that the pub struct defined in the mod.rs would be automatically exported?
// a: 是的，mod.rs 中定义的 pub struct 会自动导出。
// q: what kind of structs and functions are necessarily defined in the mod.rs better than in the other files?
// a: 在 mod.rs 中，最好定义 pub struct 和 pub enum，这样可以方便其他模块使用。
// q: how to define the pub struct and pub enum only used in the same crate?
// a: 在 mod.rs 中定义 pub struct 和 pub enum，这样可以方便其他模块使用，但是只能在同一个 crate 中使用。
// q: how to define the pub struct and pub enum used in the other crates?
// a: 在 lib.rs 中定义 pub struct 和 pub enum，这样可以方便其他 crate 使用。

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts), // 这是 clap mod 的特性用法之一，灵活将 enum 特性与 struct 特性结合起来，实现了更复杂的命令行参数解析。
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
        // filename.to_string() or filename.into()?
        // filename.to_string() 和 filename.into() 是等价的，都是将一个 &str 转换为 String 类型。
    } else {
        Err("Input file does not exist".to_string())
    }
}

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
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_input_file("*"),
            Err("Input file does not exist".to_string())
        );
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(
            verify_input_file("not-exist"),
            Err("Input file does not exist".to_string())
        );
    }
}
// what is #[cfg(test)]?
// #[cfg(test)] 是一个属性，用于指定测试模块，只有在测试模式下才会编译测试模块。
