// rcli 是一个库 crate，所以它的入口是 lib.rs 文件。
// 把整个 crate 下属的 module， 可供 main.rs 调用的，都包含进来，
// 然后旗下各个 module 中定义的 pub 属性的数据结构和函数，也都包含进来。

mod cli;
mod process;
mod utils;

// 数据结构
pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, Subcommand, TextSignFormat,
    TextSubCommand,
};
// q: in current crate, since the cli is a directory, why is it treated as a module?
// a: 在当前 crate 中，cli 是一个目录，但是在 Rust 中，目录也可以作为一个模块，所以 cli 目录也被当作一个模块来处理。

// q: what is the functionality of the above code?
// a: 这段代码用于导入 cli 模块，并将 Opts 和 Subcommand 导出。
// q: 模块的定义是一个目录，并带有mod.rs文件？
// a: 是的，模块的定义是一个目录，并带有 mod.rs 文件。
// q: explain it in english
// a: The definition of a module is a directory with a mod.rs file.
// q: 将 Opts 和 Subcommand 导出，给谁使用呢？ 是 main.rs 吗？
// a: 是的，将 Opts 和 Subcommand 导出，给 main.rs 使用。

// 函数
pub use process::*;
pub use utils::*;
