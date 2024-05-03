use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?; // 用 anyhow 格式？替代 .unwrap()
    let mut results: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone(); // => 这里如果不做 .clone，那么下面的代码中 reader.records() 就无法再次使用，因为 reader.headers() 会消耗 reader 对象，所以这里需要 clone 一份 headers，以便后续使用.
                                             // 为 Player 特制化的代码，不通用
                                             // for result in reader.deserialize() { // csv record 在内存中的保存，可能是一个字符串元素的 array，这个跟不同 csv crate 的具体实现有关。
                                             //     let record: Player = result?; // 用 anyhow 格式？替代 .unwrap(), 实际上是代替了繁琐的 match statement, 也只有到了这一步，才实现了 csv record 到 player 类型的显式转换
                                             //     results.push(record);
                                             //     // println!("{:?}", player);
                                             // }

    for result in reader.records() {
        // csv record 在内存中的保存，可能是一个字符串元素的 array，这个跟不同 csv crate 的具体实现有关。
        let record: StringRecord = result?; // 用 anyhow 格式？替代 .unwrap(), 实际上是代替了繁琐的 match statement, 也只有到了这一步，才实现了 csv record 到 player 类型的显式转换
        let output_value = headers.iter().zip(record.iter()).collect::<Value>(); // pub enum Value of serde::value
                                                                                 // println!("{:?}", jason_value);
        results.push(output_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&results)?,
        OutputFormat::Yaml => serde_yaml::to_string(&results)?,
    };

    fs::write(output, content)?; // => ();

    Ok(())
}
