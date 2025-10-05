use anyhow::Result;
use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::ComponentId;
use colored::*;
use serde::Serialize;

use crate::data::BomData;
use crate::output;

#[derive(Debug, Serialize)]
struct CostOutput {
    component: String,
    total_cost: String,
    material_cost: String,
    labor_cost: String,
    overhead_cost: String,
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
        .calculate_cost(&component_id)
        .map_err(|e| anyhow::anyhow!(rust_i18n::t!("errors.calculation_error", error = e.to_string())))?;

    if format == "table" {
        let mut output = String::new();
        output.push_str(&format!(
            "\n{}\n\n",
            rust_i18n::t!("commands.cost.result_header", component = component)
                .bold()
                .green()
        ));

        output.push_str(&format!(
            "{}: ${}\n",
            rust_i18n::t!("commands.cost.total_cost").bold(),
            result.total_cost
        ));
        output.push_str(&format!(
            "{}: ${}\n",
            rust_i18n::t!("commands.cost.material_cost").bold(),
            result.material_cost
        ));
        output.push_str(&format!(
            "Labor Cost: ${}\n",
            result.labor_cost
        ));
        output.push_str(&format!(
            "Overhead Cost: ${}\n",
            result.overhead_cost
        ));

        Ok(output)
    } else {
        let output_data = CostOutput {
            component: component.to_string(),
            total_cost: result.total_cost.to_string(),
            material_cost: result.material_cost.to_string(),
            labor_cost: result.labor_cost.to_string(),
            overhead_cost: result.overhead_cost.to_string(),
        };

        output::format_output(&output_data, format)
    }
}
