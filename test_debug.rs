use bom_core::repository::memory::InMemoryRepository;
use bom_core::*;
use bom_graph::BomGraph;
use bom_calc::CostCalculator;
use chrono::Utc;
use rust_decimal::Decimal;

fn create_test_component(id: &str, cost: i32) -> Component {
    Component {
        id: ComponentId::new(id),
        description: format!("Component {}", id),
        component_type: ComponentType::FinishedProduct,
        uom: "EA".to_string(),
        standard_cost: Some(Decimal::from(cost)),
        lead_time_days: Some(7),
        procurement_type: ProcurementType::Make,
        organization: "ORG01".to_string(),
        version: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_test_bom_item(parent: &str, child: &str, qty: i32) -> BomItem {
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

fn main() {
    let repo = InMemoryRepository::new();

    repo.add_component(create_test_component("A", 100));
    repo.add_component(create_test_component("B", 50));
    repo.add_component(create_test_component("C", 30));

    repo.add_bom_item(create_test_bom_item("A", "B", 2));
    repo.add_bom_item(create_test_bom_item("A", "C", 1));

    let graph = BomGraph::from_repository(&repo).unwrap();
    
    println!("Graph stats: {:?}", graph.stats());
    println!("Roots: {:?}", graph.roots().len());
    
    let calc = CostCalculator::new(&graph, &repo);
    let all_costs = calc.calculate_all_costs(graph.roots()).unwrap();
    
    for (id, cost) in &all_costs {
        println!("{}: {}", id.as_str(), cost.total_cost);
    }
    
    let cost = calc.calculate_cost(&ComponentId::new("A")).unwrap();
    println!("\nFinal cost of A: {}", cost.total_cost);
}
