// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

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
    }

    Ok(())
}
