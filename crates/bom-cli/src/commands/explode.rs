use anyhow::Result;
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::ComponentId;
use colored::*;
use rust_decimal::Decimal;
use serde::Serialize;
use std::str::FromStr;

use crate::data::BomData;
use crate::output;

#[derive(Debug, Serialize)]
struct ExplosionOutput {
    component: String,
    quantity: String,
    level: usize,
}

pub fn execute(bom_data: &BomData, component: &str, quantity_str: &str, format: &str) -> Result<String> {
    let quantity = Decimal::from_str(quantity_str)
        .map_err(|_| anyhow::anyhow!(rust_i18n::t!("errors.invalid_quantity", qty = quantity_str)))?;

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
        .explode(&component_id, quantity)
        .map_err(|e| anyhow::anyhow!(rust_i18n::t!("errors.calculation_error", error = e.to_string())))?;

    if format == "table" {
        let mut output = String::new();
        output.push_str(&format!(
            "\n{}\n\n",
            rust_i18n::t!("commands.explode.result_header", component = component, qty = quantity)
                .bold()
                .green()
        ));

        output.push_str(&format!(
            "{} | {} | {}\n",
            rust_i18n::t!("commands.explode.level").bold().cyan(),
            rust_i18n::t!("commands.explode.component").bold().cyan(),
            rust_i18n::t!("commands.explode.quantity").bold().cyan()
        ));
        output.push_str(&format!("{}\n", "─".repeat(80).dimmed()));

        for item in &result.items {
            output.push_str(&format!(
                "{} | {} | {}\n",
                item.level, item.component_id.as_str(), item.total_quantity
            ));
        }

        output.push_str(&format!(
            "\n{}\n",
            rust_i18n::t!("commands.explode.total_items", count = result.items.len()).dimmed()
        ));

        Ok(output)
    } else {
        let output_data: Vec<ExplosionOutput> = result
            .items
            .iter()
            .map(|item| ExplosionOutput {
                component: item.component_id.as_str().to_string(),
                quantity: item.total_quantity.to_string(),
                level: item.level,
            })
            .collect();

        output::format_output(&output_data, format)
    }
}
