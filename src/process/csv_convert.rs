use crate::OutputFormat;
use anyhow::{Context, Result};
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs::write;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers: StringRecord = reader.headers()?.clone();
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
            let mut table = serde_json::Map::new();
            for (i, row) in rows.iter().enumerate() {
                table.insert(format!("{}", i), row.clone());
            }
            toml::to_string_pretty(&Value::Object(table))?
        }
    };
    write(&output, content).with_context(|| format!("写入输出文件失败: {}", output))?;
    Ok(())
}
