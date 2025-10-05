use anyhow::Result;
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::ComponentId;
use colored::*;
use serde::Serialize;

use crate::data::BomData;
use crate::output;

#[derive(Debug, Serialize)]
struct WhereUsedOutput {
    parent: String,
    quantity: String,
    level: usize,
}

pub fn execute(bom_data: &BomData, component: &str, format: &str) -> Result<String> {
    let (components, bom_items) = bom_data.to_core()?;

    // Create repository and add data
    let repo = InMemoryRepository::new();
    for component in components {
        repo.add_component(component);
    }
    for bom_item in bom_items {
        repo.add_bom_item(bom_item);
    }

    let engine = BomEngine::new(repo)?;
    let component_id = ComponentId::new(component);
    let result = engine
        .where_used(&component_id)
        .map_err(|e| anyhow::anyhow!(rust_i18n::t!("errors.calculation_error", error = e.to_string())))?;

    if format == "table" {
        let mut output = String::new();
        output.push_str(&format!(
            "\n{}\n\n",
            rust_i18n::t!("commands.where_used.result_header", component = component)
                .bold()
                .green()
        ));

        output.push_str(&format!(
            "{}\n\n",
            rust_i18n::t!("commands.where_used.used_in", count = result.used_in.len())
        ));

        output.push_str(&format!(
            "{} | {}\n",
            rust_i18n::t!("commands.where_used.parent").bold().cyan(),
            rust_i18n::t!("commands.where_used.usage_qty").bold().cyan()
        ));
        output.push_str(&format!("{}\n", "─".repeat(80).dimmed()));

        for item in &result.used_in {
            output.push_str(&format!(
                "{} | {} | {}\n",
                item.parent_id.as_str(),
                item.quantity,
                item.level
            ));
        }

        Ok(output)
    } else {
        let output_data: Vec<WhereUsedOutput> = result
            .used_in
            .iter()
            .map(|item| WhereUsedOutput {
                parent: item.parent_id.as_str().to_string(),
                quantity: item.quantity.to_string(),
                level: item.level,
            })
            .collect();

        output::format_output(&output_data, format)
    }
}
