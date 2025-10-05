use anyhow::{Context, Result};
use bom_core::*;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BomData {
    pub components: Vec<ComponentData>,
    pub bom_items: Vec<BomItemData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentData {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub component_type: String,
    pub standard_cost: Option<String>,
    #[serde(default = "default_uom")]
    pub uom: String,
    #[serde(default)]
    pub procurement_type: String,
    #[serde(default)]
    pub organization: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BomItemData {
    pub parent_id: String,
    pub child_id: String,
    pub quantity: String,
    #[serde(default)]
    pub scrap_factor: String,
    #[serde(default = "default_sequence")]
    pub sequence: i32,
}

fn default_uom() -> String {
    "EA".to_string()
}

fn default_sequence() -> i32 {
    10
}

pub fn load_bom(path: &Path) -> Result<BomData> {
    let content = std::fs::read_to_string(path)
        .with_context(|| rust_i18n::t!("errors.file_not_found", path = path.display()))?;

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match extension {
        "json" => {
            serde_json::from_str(&content).with_context(|| rust_i18n::t!("errors.parse_error", error = "JSON"))
        }
        "csv" => load_csv(&content),
        _ => anyhow::bail!(rust_i18n::t!("errors.invalid_format", format = extension)),
    }
}

fn load_csv(content: &str) -> Result<BomData> {
    // Simple CSV format: parent,child,qty,cost
    let mut components_map: HashMap<String, ComponentData> = HashMap::new();
    let mut bom_items = Vec::new();

    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    for result in rdr.records() {
        let record = result?;
        if record.len() < 3 {
            continue;
        }

        let parent = record[0].to_string();
        let child = record[1].to_string();
        let qty = record[2].to_string();
        let cost = record.get(3).map(|s| s.to_string());

        // Add components if not exists
        components_map.entry(parent.clone()).or_insert_with(|| ComponentData {
            id: parent.clone(),
            description: parent.clone(),
            component_type: "FinishedProduct".to_string(),
            standard_cost: cost.clone(),
            uom: "EA".to_string(),
            procurement_type: "Make".to_string(),
            organization: "DEFAULT".to_string(),
        });

        components_map.entry(child.clone()).or_insert_with(|| ComponentData {
            id: child.clone(),
            description: child.clone(),
            component_type: "RawMaterial".to_string(),
            standard_cost: cost,
            uom: "EA".to_string(),
            procurement_type: "Buy".to_string(),
            organization: "DEFAULT".to_string(),
        });

        bom_items.push(BomItemData {
            parent_id: parent,
            child_id: child,
            quantity: qty,
            scrap_factor: "0".to_string(),
            sequence: 10,
        });
    }

    Ok(BomData {
        components: components_map.into_values().collect(),
        bom_items,
    })
}

impl BomData {
    pub fn to_core(&self) -> Result<(Vec<Component>, Vec<BomItem>)> {
        let components: Vec<Component> = self
            .components
            .iter()
            .map(|c| {
                Ok(Component {
                    id: ComponentId::new(&c.id),
                    description: c.description.clone(),
                    component_type: match c.component_type.as_str() {
                        "FinishedProduct" => ComponentType::FinishedProduct,
                        "SemiFinished" => ComponentType::SemiFinished,
                        "RawMaterial" => ComponentType::RawMaterial,
                        _ => ComponentType::RawMaterial,
                    },
                    uom: c.uom.clone(),
                    standard_cost: c.standard_cost.as_ref().and_then(|s| s.parse().ok()),
                    lead_time_days: Some(7),
                    procurement_type: match c.procurement_type.as_str() {
                        "Make" => ProcurementType::Make,
                        "Buy" => ProcurementType::Buy,
                        _ => ProcurementType::Buy,
                    },
                    organization: c.organization.clone(),
                    version: 0,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let bom_items: Vec<BomItem> = self
            .bom_items
            .iter()
            .map(|item| {
                Ok(BomItem {
                    id: Uuid::new_v4(),
                    parent_id: ComponentId::new(&item.parent_id),
                    child_id: ComponentId::new(&item.child_id),
                    quantity: item.quantity.parse()?,
                    scrap_factor: item.scrap_factor.parse().unwrap_or(Decimal::ZERO),
                    sequence: item.sequence as u32,
                    effective_from: None,
                    effective_to: None,
                    alternative_group: None,
                    is_phantom: false,
                    reference_designator: None,
                    notes: None,
                    operation_sequence: None,
                    alternative_priority: None,
                    position: None,
                    version: 0,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok((components, bom_items))
    }
}
