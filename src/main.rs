// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?, // 此处申明的 opts，与上面的 opts，容易产生混淆，这是课堂中老师忽略的地方。
    }

    Ok(())
}
