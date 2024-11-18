// rcli csv -i input.csv -o output.json --header -d ','
// cargo run -- csv -i assets/juventus.csv
// cargo run -- csv -i assets/juventus.csv -o output.json

use clap::Parser;
use rcli::{CmdExector, Opts}; // command line argument parser
                              // q: cargo add or cargo install, what is the difference?
                              // a: cargo add 是一个 cargo 的插件，用于添加依赖，cargo install 是一个 cargo 的命令，用于安装二进制文件。
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
    opts.cmd.execute().await?;

    Ok(())
}
