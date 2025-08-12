use anyhow::Result;
use csv::{Reader, StringRecord};
// use serde::{Deserialize, Serialize};
use crate::OutputFormat;
use serde_json::Value;
use std::fs::write;

// #[derive(Debug, Serialize, Deserialize)]
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
        OutputFormat::Toml => {
            // TOML doesn't support root-level arrays, so we have options:

            // Option 1: Array of tables format [[data]] (current)
            // let wrapper = serde_json::json!({"data": rows});
            // toml::to_string_pretty(&wrapper)?

            // Option 2: Individual numbered tables [record_0], [record_1], etc.
            // let mut toml_content = String::new();
            // for (index, row) in rows.iter().enumerate() {
            //     toml_content.push_str(&format!("[record_{}]\n", index));
            //     if let Value::Object(map) = row {
            //         for (key, value) in map {
            //             if let Value::String(val) = value {
            //                 toml_content.push_str(&format!("{} = \"{}\"\n", key, val));
            //             }
            //         }
            //     }
            //     toml_content.push('\n');
            // }
            // toml_content

            // Option 3: Single table with array values (uncomment to use)
            let mut table = serde_json::Map::new();
            for (i, row) in rows.iter().enumerate() {
                table.insert(format!("{}", i), row.clone());
            }
            toml::to_string_pretty(&Value::Object(table))?
        }
    };

    let _ = write(output, content);
    Ok(())
}
