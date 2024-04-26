use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?; // 用 anyhow 格式？替代 .unwrap()
    let mut results: Vec<Player> = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?; // 用 anyhow 格式？替代 .unwrap()
        results.push(record);
        // println!("{:?}", player);
    }

    let json = serde_json::to_string_pretty(&results)?;
    fs::write(output, json)?; // => ();

    Ok(())
}
