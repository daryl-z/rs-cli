use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs::write;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, _>>()?;
    println!("CSV Options: {:?}", records);
    let json = serde_json::to_string(&records)?;
    write(output, json)?;
    Ok(())
}
