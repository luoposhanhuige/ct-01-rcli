// rcli csv -i input.csv -o output.json --header -d ','
// cargo run -- csv -i assets/juventus.csv
// cargo run -- csv -i assets/juventus.csv -o output.json

use std::fs;

use clap::Parser; // command line argument parser
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, Subcommand, TextSignFormat, TextSubCommand,
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
use zxcvbn::zxcvbn;
// use tracing_subscriber;
// Yes, the statement tracing_subscriber::fmt::init(); works without needing to import use tracing_subscriber; because you are using the fully qualified path to call the function. In Rust, you can call functions using their fully qualified names without importing the module.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init(); // 初始化日志记录器,  RUST_LOG=debug cargo run

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
    // Parse from std::env::args_os. the parse() is a function of clap_builder::derive::Parser
    // q: why Opts::parse() is used here?
    // a: Opts::parse() 是一个函数，用于解析命令行参数，这里使用 Opts::parse() 是为了解析命令行参数。

    match opts.cmd {
        // 第一层命令行参数，一对多关系，从一个struct的field，映射到多个变体的enum
        Subcommand::Csv(opts) => {
            // 因为枚举类型中的单个变体，就是Csv(CsvOpts)形式。再从此映射到CsvOpts这个struct的多个fields
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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;

            println!("{}", password); // 将打印出不带双引号的字符串 This prints the password using the Display trait.

            // output password strength in stderr
            let estimate = zxcvbn(&password, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }

        Subcommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    // println!("Encoding: {:?}", opts);
                    let encoded = process_encode(&opts.input, opts.format)?;
                    println!("{}", encoded);
                }
                Base64SubCommand::Decode(opts) => {
                    // println!("Decoding: {:?}", opts);
                    let decoded = process_decode(&opts.input, opts.format)?;
                    let decoded = String::from_utf8(decoded)?;
                    println!("{}", decoded);
                }
            }
        }

        Subcommand::Text(subcmd) => {
            match subcmd {
                TextSubCommand::Sign(opts) => {
                    let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                    println!("{}", signed);
                }

                TextSubCommand::Verify(opts) => {
                    let verified =
                        process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                    println!("{}", verified);
                }

                TextSubCommand::Generate(opts) => {
                    let key = process_text_generate(opts.format)?;
                    match opts.format {
                        TextSignFormat::Blake3 => {
                            let name = opts.output.join("blake3.txt");
                            fs::write(name, &key[0])?;
                        }
                        TextSignFormat::ED25519 => {
                            let name = &opts.output;
                            fs::write(name.join("ed25519.sk"), &key[0])?;
                            fs::write(name.join("ed25519.pk"), &key[1])?;
                        }
                    }

                    // println!("Generating key pair: {:?}", opts);
                }
            }
        }

        Subcommand::Http(subcmd) => {
            match subcmd {
                HttpSubCommand::Serve(opts) => {
                    // println!("{:?}", opts);
                    // println!("Serving at http://0.0.0.0:{}", opts.port);
                    process_http_serve(opts.dir, opts.port).await?;
                }
            }
        }
    }
    // opts.cmd.execute().await?;

    Ok(())
}
