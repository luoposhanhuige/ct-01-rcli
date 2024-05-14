// rcli csv -i input.csv -o output.json --header -d ','
// cargo run -- csv -i assets/juventus.csv
// cargo run -- csv -i assets/juventus.csv -o output.json

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    Subcommand,
};
// q: how does the compiler find process_csv, process_genpass, Opts, Subcommand?
// a: 编译器会根据当前目录结构，自动查找 process_csv、process_genpass、Opts、Subcommand 这几个模块，如果找到了，则会导入这几个模块，如果没有找到，则会报错。
// q: 根据我现在这个 rcli 项目的目录结构，编译器会怎么查找这几个模块呢？
// a: 根据你现在的 rcli 项目的目录结构，编译器会先查找当前目录下的 mod.rs 文件，如果没有找到，则会查找当前目录下的 lib.rs 文件，如果还没有找到，则会查找当前目录下的 main.rs 文件，如果还没有找到，则会报错。
// q: 编译器，只会查找 mod.rs lib.rs main.rs 三种类型的文件，而不会查找其他类型的文件吗？
// a: 是的，编译器只会查找 mod.rs、lib.rs、main.rs 这三种类型的文件，其他类型的文件是不会查找的。

// q: 那你告诉我， process_csv, process_genpass, Opts, Subcommand，这些都是结构体吗？compiler 都是在哪些具体的目录和文件中找到的？
// a: process_csv、process_genpass、Opts、Subcommand 都是结构体，process_csv 和 process_genpass 是在 process 模块中定义的，Opts 和 Subcommand 是在 cli 模块中定义的。

// q: cargo add or cargo install, what is the difference?
// a: cargo add 是一个 cargo 的插件，用于添加依赖，cargo install 是一个 cargo 的命令，用于安装二进制文件。

fn main() -> anyhow::Result<()> {
    // q: why does not need to import anyhow before using it?
    // a: 因为 anyhow 是一个宏，可以直接使用，不需要导入。
    // q: where is the definition of anyhow?
    // a: anyhow 的定义在 anyhow 模块中，anyhow 模块的定义在 src/anyhow.rs 文件中。
    // q: anyhow is a module or a crate?
    // a: anyhow 是一个 crate，用于处理错误。
    // q: what is the difference between a module and a crate?
    // a: 模块是一个文件，用于组织代码，而 crate 是一个包，用于组织模块。
    // q: library vs crate
    // a: library 是一个库，用于存储模块，而 crate 是一个包，用于存储库。
    // q: what kind of file type after library being compilated?
    // a: library 在编译后，会生成一个 .rlib 文件，用于存储编译后的库。

    let opts = Opts::parse();
    // q: what does Opts::parse() mean? what is the return value of Opts::parse()?
    // a: Opts::parse() 是一个函数调用，用于解析命令行参数，返回值是一个 Opts 结构体，用于存储命令行参数的值。
    // q: 返回值是一个 Opts 结构体，该结构体的定义在哪里？
    // a: Opts 结构体的定义在 cli 模块中，cli 模块的定义在 src/cli/mod.rs 文件中。
    // q: does Opts::parse() receive input from command line by user and parse it into Opts struct?
    // a: 是的，Opts::parse() 会接收用户在命令行中输入的参数，并将这些参数解析为 Opts 结构体。
    // q: does parse() require the struct to which it returns value to have field named cmd? and it should be the enum type?
    // a: 是的，parse() 函数要求返回值的结构体中必须有一个名为 cmd 的字段，而且这个字段的类型必须是一个 enum 类型。

    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                "output.json".into()
                // q: what does this mean?
                // a: 这里的 into() 是一个 trait，可以将一个类型转换为另一个类型，这里是将一个 &str 转换为 String 类型。
                // q: 怎么知道是要转换为 String 类型的？
                // a: 因为 "output.json" 是一个字符串字面量，所以这里的 into() 会将其转换为 String 类型。
                // q: 如要要转换为其他类型，如何处理？
                // a: 如果是其他类型，就需要使用 as 关键字，如：output as i32，这样就将 output 转换为 i32 类型。
            };
            process_csv(&opts.input, output, opts.format)?; // 此处申明的 opts，与上面的 opts，容易产生混淆，这是课堂中老师忽略的地方。
        }

        Subcommand::Genpass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;

            // println!("Generating password: {:?}", opts);
        }

        Subcommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    // println!("Encoding: {:?}", opts);
                    process_encode(&opts.input, opts.format)?;
                }
                Base64SubCommand::Decode(opts) => {
                    // println!("Decoding: {:?}", opts);
                    process_decode(&opts.input, opts.format)?;
                }
            }
        }
    }

    Ok(())
}
