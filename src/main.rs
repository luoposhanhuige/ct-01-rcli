// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
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
        } // q: what does the above code mean?
          // a: 这段代码是一个 match 表达式，用于匹配 opts.cmd 的值，如果 opts.cmd 的值是 Subcommand::Csv，则执行 process_csv() 函数；如果 opts.cmd 的值是 Subcommand::Genpass，则执行 process_genpass() 函数。
          // q: what does the question mark mean?
          // a: 问号是一个语法糖，用于简化错误处理，如果表达式的结果是 Ok，则返回 Ok 的值，如果结果是 Err，则返回 Err 的值，并将 Err 的值转换为当前函数的返回值。
    }

    Ok(())
}
