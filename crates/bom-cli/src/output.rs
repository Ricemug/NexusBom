use anyhow::Result;
use colored::*;
use serde::Serialize;

pub fn format_output<T: Serialize>(data: &T, format: &str) -> Result<String> {
    match format {
        "json" => Ok(serde_json::to_string_pretty(data)?),
        "csv" => {
            // CSV output for simple data
            let json = serde_json::to_value(data)?;
            if let Some(arr) = json.as_array() {
                let mut wtr = csv::Writer::from_writer(vec![]);
                let mut headers_written = false;
                for item in arr {
                    if let Some(obj) = item.as_object() {
                        let headers: Vec<String> = obj.keys().cloned().collect();
                        if !headers_written {
                            wtr.write_record(&headers)?;
                            headers_written = true;
                        }
                        let values: Vec<String> = obj
                            .values()
                            .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                            .collect();
                        wtr.write_record(&values)?;
                    }
                }
                Ok(String::from_utf8(wtr.into_inner()?)?)
            } else {
                anyhow::bail!("CSV format requires array data")
            }
        }
        _ => Ok(serde_json::to_string_pretty(data)?),
    }
}

pub fn print_table_header(headers: &[&str]) {
    println!(
        "{}",
        headers
            .iter()
            .map(|h| h.bold().cyan().to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!("{}", "─".repeat(80).dimmed());
}

pub fn print_table_row(values: &[String]) {
    println!("{}", values.join(" | "));
}
