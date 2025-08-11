use anyhow::Result;
use csv::{Reader, StringRecord};
// use serde::{Deserialize, Serialize};
use crate::OutputFormat;
use serde_json::Value;
use std::fs::write;

// #[derive(Debug, Serialize, Deseri alize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     #[serde(rename = "Name")]
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit_number: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers: StringRecord = reader.headers()?.clone();
    // let mut json_rows: Vec<Value> = Vec::new();
    // for record_result in reader.records() {
    //     let record = record_result?;
    //     let mut obj = serde_json::Map::with_capacity(headers.len());
    //     for (key, value) in headers.iter().zip(record.iter()) {
    //         obj.insert(key.to_string(), Value::String(value.to_string()));
    //     }
    //     json_rows.push(Value::Object(obj));
    // }

    let rows: Vec<Value> = reader
        .records()
        .map(|record_res| {
            record_res.map(|record| {
                Value::Object(
                    headers
                        .iter()
                        .zip(record.iter())
                        .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
                        .collect(),
                )
            })
        })
        .collect::<Result<Vec<_>, csv::Error>>()?;

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&rows)?,
        OutputFormat::Yaml => serde_yaml::to_string(&rows)?,
        OutputFormat::Toml => toml::to_string(&rows)?,
    };

    let _ = write(output, content);
    Ok(())
}
