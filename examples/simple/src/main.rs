/// Simple BOM calculation example
///
/// This example demonstrates:
/// - Creating components
/// - Building BOM structure
/// - Material explosion
/// - Cost calculation
/// - Where-used analysis

use bom_calc::BomEngine;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use chrono::Utc;
use rust_decimal::Decimal;

fn main() {
    println!("=== BOM Calculation Example ===\n");

    // Create repository
    let repo = InMemoryRepository::new();

    // Define components with costs
    // Bicycle (A) = $500
    //   ├─ Frame (B) = $200, qty 1
    //   │   └─ Aluminum Tube (D) = $50, qty 2
    //   └─ Wheel Set (C) = $150, qty 2
    //       └─ Aluminum Tube (D) = $50, qty 1 (shared component!)

    println!("Creating components...");
    repo.add_component(create_component("Bicycle", "A", 500));
    repo.add_component(create_component("Frame", "B", 200));
    repo.add_component(create_component("Wheel Set", "C", 150));
    repo.add_component(create_component("Aluminum Tube", "D", 50));

    println!("Building BOM structure...");
    repo.add_bom_item(create_bom_item("A", "B", 1)); // Bicycle -> Frame
    repo.add_bom_item(create_bom_item("A", "C", 2)); // Bicycle -> Wheel Set (x2)
    repo.add_bom_item(create_bom_item("B", "D", 2)); // Frame -> Aluminum Tube (x2)
    repo.add_bom_item(create_bom_item("C", "D", 1)); // Wheel Set -> Aluminum Tube

    // Create BOM engine
    let engine = BomEngine::new(repo).unwrap();

    // Display graph statistics
    let stats = engine.stats();
    println!("\n📊 BOM Graph Statistics:");
    println!("  Components: {}", stats.node_count);
    println!("  Relationships: {}", stats.edge_count);
    println!("  Max Depth: {}", stats.max_depth);
    println!("  Root Products: {}", stats.root_count);

    // Material Explosion
    println!("\n🔧 Material Explosion (製造 10 輛自行車):");
    let explosion = engine
        .explode(&ComponentId::new("A"), Decimal::from(10))
        .unwrap();

    for item in &explosion.items {
        println!(
            "  Level {}: {} - Quantity: {} {}",
            item.level,
            get_component_name(&item.component_id),
            item.total_quantity,
            if item.is_phantom { "(Phantom)" } else { "" }
        );
    }

    // Cost Calculation
    println!("\n💰 Cost Calculation:");
    let cost = engine.calculate_cost(&ComponentId::new("A")).unwrap();
    println!("  Bicycle Total Cost: ${}", cost.total_cost);
    println!("  - Material: ${}", cost.material_cost);
    println!("  - Labor: ${}", cost.labor_cost);
    println!("  - Overhead: ${}", cost.overhead_cost);

    // Cost Drivers Analysis
    println!("\n📈 Cost Drivers (Top Contributors):");
    let drivers = engine.analyze_cost_drivers(&ComponentId::new("A")).unwrap();
    for (i, driver) in drivers.iter().take(3).enumerate() {
        println!(
            "  {}. {} - ${} ({:.1}%)",
            i + 1,
            get_component_name(&driver.component_id),
            driver.cost,
            driver.percentage
        );
    }

    // Where-Used Analysis
    println!("\n🔍 Where-Used Analysis (鋁管 D 用在哪裡):");
    let where_used = engine.where_used(&ComponentId::new("D")).unwrap();
    for item in &where_used.used_in {
        println!(
            "  - {} (qty: {}, level: {})",
            get_component_name(&item.parent_id),
            item.quantity,
            item.level
        );
    }

    // Change Impact Analysis
    println!("\n⚠️  Change Impact Analysis (如果鋁管 D 變更):");
    let impact = engine.analyze_change_impact(&ComponentId::new("D")).unwrap();
    println!("  Affected Components: {}", impact.affected_components.len());
    for comp_id in &impact.affected_components {
        println!("    - {}", get_component_name(comp_id));
    }
    println!("  Affected Root Products: {}", impact.affected_root_assemblies.len());
    for comp_id in &impact.affected_root_assemblies {
        println!("    - {}", get_component_name(comp_id));
    }

    // Shared Components
    println!("\n🔗 Shared Components Analysis:");
    let shared = engine
        .find_shared_components(&[ComponentId::new("B"), ComponentId::new("C")])
        .unwrap();
    for sc in &shared {
        println!(
            "  {} - used in {} assemblies",
            get_component_name(&sc.component_id),
            sc.used_in_count
        );
    }

    println!("\n✅ All calculations completed successfully!");
}

fn create_component(name: &str, id: &str, cost: i32) -> Component {
    Component {
        id: ComponentId::new(id),
        description: name.to_string(),
        component_type: ComponentType::FinishedProduct,
        uom: "EA".to_string(),
        standard_cost: Some(Decimal::from(cost)),
        lead_time_days: Some(7),
        procurement_type: ProcurementType::Make,
        organization: "FACTORY01".to_string(),
        version: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_bom_item(parent: &str, child: &str, qty: i32) -> BomItem {
    BomItem {
        id: uuid::Uuid::new_v4(),
        parent_id: ComponentId::new(parent),
        child_id: ComponentId::new(child),
        quantity: Decimal::from(qty),
        scrap_factor: Decimal::ZERO,
        sequence: 10,
        operation_sequence: None,
        is_phantom: false,
        effective_from: None,
        effective_to: None,
        alternative_group: None,
        alternative_priority: None,
        reference_designator: None,
        position: None,
        notes: None,
        version: 0,
    }
}

fn get_component_name(id: &ComponentId) -> &str {
    match id.as_str() {
        "A" => "Bicycle",
        "B" => "Frame",
        "C" => "Wheel Set",
        "D" => "Aluminum Tube",
        _ => id.as_str(),
    }
}
