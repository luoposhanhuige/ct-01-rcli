// cargo run -- csv -i assets/juventus.csv -f yaml -o output.yaml
// cargo run -- csv -i assets/juventus.csv -f json -o output.json
// cargo nextest run

// q: is this a module file? why?
// a: 是的，这是一个模块文件，因为它包含了 mod 关键字，用于定义模块。
// q: where is the keyword mod?
// a: mod 关键字在第一行，用于定义模块。

use csv::{Reader, StringRecord};
// use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

// q: serde_json? serde_json::Value?
// a: serde_json 是一个库，用于处理 json 格式的数据，serde_json::Value 是 serde_json 库中的一个枚举类型，用于存储 json 格式的数据。

// Enum serde_json::Value
// pub enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }

use crate::cli::OutputFormat;
// q: how could compiler lookfor OutputFormat?
// a: 因为在 rcli/src/cli/mod.rs 中，已经导入了 OutputFormat，所以在 rcli/src/process/csv_convert.rs 中，就可以直接使用 OutputFormat。

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     nationality: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }
// q: 貌似这段代码没有被 process_csv() 用到。
// a: 是的，这段代码没有被 process_csv() 用到，这段代码是用于定义 Player 结构体的，用于将 csv 文件中的记录转换为 Player 类型的结构体。

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?; // 用 anyhow 格式？替代 .unwrap()
    let mut results: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone(); // => 这里如果不做 .clone，那么下面的代码中 reader.records() 就无法再次使用，因为 reader.headers() 会消耗 reader 对象，所以这里需要 clone 一份 headers，以便后续使用.

    // 以下注释代码，为 Player 特制化的代码，不通用
    // for result in reader.deserialize() { // csv record 在内存中的保存，可能是一个字符串元素的 array，这个跟不同 csv crate 的具体实现有关。
    // let record: Player = result?; // 用 anyhow 格式？替代 .unwrap(), 实际上是代替了繁琐的 match statement, 也只有到了这一步，才实现了 csv record 到 player 类型的显式转换
    // results.push(record);
    // println!("{:?}", player);

    for result in reader.records() {
        // csv record 在内存中的保存，可能是一个字符串元素的 array，这个跟不同 csv crate 的具体实现有关。
        let record: StringRecord = result?; // 用 anyhow 格式？替代 .unwrap(), 实际上是代替了繁琐的 match statement, 也只有到了这一步，才实现了 csv record 到 player 类型的显式转换
                                            // q: StringRecord?
                                            // a: StringRecord 是一个类型，用于存储 csv 文件中的记录，StringRecord? 是一个 Result 类型，用于存储 StringRecord 类型的值，如果读取成功，则返回 Ok(StringRecord)，如果读取失败，则返回 Err(StringRecord)。

        let output_value = headers.iter().zip(record.iter()).collect::<Value>(); // pub enum Value of serde::Value
                                                                                 // q: is the headers is something like "Name,Position,DOB,Nationality,Kit Number", and the record is something like "Wojciech Szczesny,Goalkeeper,"Apr 18, 1990 (29)",Poland,1", for "let output_value = headers.iter().zip(record.iter()).collect::<Value>();", what would be the output_value? and what is the type of output_value?
                                                                                 // a: 是的，headers 是一个字符串数组，用于存储 csv 文件中的列名，record 是一个字符串数组，用于存储 csv 文件中的记录，output_value 是一个 Value 类型的枚举，用于存储 csv 文件中的记录，output_value 的类型是 Value::Object，用于存储 csv 文件中的记录。

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
