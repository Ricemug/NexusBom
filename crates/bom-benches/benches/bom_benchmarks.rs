use bom_calc::costing::CostCalculator;
use bom_calc::explosion::ExplosionCalculator;
use bom_calc::where_used::WhereUsedAnalyzer;
use bom_core::repository::memory::InMemoryRepository;
use bom_core::{BomItem, Component, ComponentId, ComponentType, ProcurementType};
use bom_graph::BomGraph;
use chrono::Utc;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_decimal::Decimal;

// Helper function to create a component
fn create_component(id: &str, description: &str, cost: i32) -> Component {
    Component {
        id: ComponentId::new(id),
        description: description.to_string(),
        component_type: ComponentType::FinishedProduct,
        uom: "EA".to_string(),
        standard_cost: Some(Decimal::from(cost)),
        lead_time_days: Some(7),
        procurement_type: ProcurementType::Make,
        organization: "PLANT-01".to_string(),
        version: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// Helper function to create a BOM item
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

// Create a simple 2-level BOM structure for benchmarking
fn create_simple_bom() -> (InMemoryRepository, ComponentId) {
    let mut repo = InMemoryRepository::new();

    // Create components
    repo.add_component(create_component("BIKE", "Bicycle", 500));
    repo.add_component(create_component("FRAME", "Frame", 150));
    repo.add_component(create_component("WHEEL1", "Front Wheel", 50));
    repo.add_component(create_component("WHEEL2", "Rear Wheel", 50));

    // Create BOM items
    repo.add_bom_item(create_bom_item("BIKE", "FRAME", 1));
    repo.add_bom_item(create_bom_item("BIKE", "WHEEL1", 1));
    repo.add_bom_item(create_bom_item("BIKE", "WHEEL2", 1));

    (repo, ComponentId::new("BIKE"))
}

// Create a deep multi-level BOM structure
fn create_deep_bom(levels: usize, children_per_level: usize) -> (InMemoryRepository, ComponentId) {
    let mut repo = InMemoryRepository::new();
    let mut component_counter = 0;

    fn add_children(
        repo: &mut InMemoryRepository,
        parent_id: &str,
        level: usize,
        max_levels: usize,
        children_per_level: usize,
        counter: &mut usize,
    ) {
        if level >= max_levels {
            return;
        }

        for i in 0..children_per_level {
            *counter += 1;
            let child_id = format!("C{:06}", *counter);

            repo.add_component(create_component(
                &child_id,
                &format!("Component {} at level {}", i, level),
                10,
            ));

            repo.add_bom_item(create_bom_item(parent_id, &child_id, 1));

            add_children(repo, &child_id, level + 1, max_levels, children_per_level, counter);
        }
    }

    repo.add_component(create_component("ROOT", "Root Assembly", 0));
    add_children(&mut repo, "ROOT", 0, levels, children_per_level, &mut component_counter);

    (repo, ComponentId::new("ROOT"))
}

// Benchmark graph construction
fn bench_graph_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_construction");

    for &(levels, width) in &[(2, 5), (3, 4), (4, 3)] {
        let (repo, root_id) = create_deep_bom(levels, width);

        group.bench_with_input(
            BenchmarkId::new("build", format!("L{}W{}", levels, width)),
            &levels,
            |b, _| {
                b.iter(|| black_box(BomGraph::from_component(&repo, &root_id, None).unwrap()))
            },
        );
    }

    group.finish();
}

// Benchmark explosion calculation
fn bench_explosion(c: &mut Criterion) {
    let mut group = c.benchmark_group("explosion");

    for &(levels, width) in &[(2, 5), (3, 4), (4, 3)] {
        let (repo, root_id) = create_deep_bom(levels, width);
        let graph = BomGraph::from_component(&repo, &root_id, None).unwrap();
        let calculator = ExplosionCalculator::new(&graph);

        group.bench_with_input(
            BenchmarkId::new("explode", format!("L{}W{}", levels, width)),
            &levels,
            |b, _| b.iter(|| black_box(calculator.explode(&root_id, Decimal::from(10)).unwrap())),
        );
    }

    group.finish();
}

// Benchmark cost calculation
fn bench_costing(c: &mut Criterion) {
    let mut group = c.benchmark_group("costing");

    for &(levels, width) in &[(2, 5), (3, 4), (4, 3)] {
        let (repo, root_id) = create_deep_bom(levels, width);
        let graph = BomGraph::from_component(&repo, &root_id, None).unwrap();
        let calculator = CostCalculator::new(&graph, &repo);

        group.bench_with_input(
            BenchmarkId::new("calculate", format!("L{}W{}", levels, width)),
            &levels,
            |b, _| b.iter(|| black_box(calculator.calculate_cost(&root_id).unwrap())),
        );
    }

    group.finish();
}

// Benchmark where-used analysis
fn bench_where_used(c: &mut Criterion) {
    let (repo, root_id) = create_simple_bom();
    let graph = BomGraph::from_component(&repo, &root_id, None).unwrap();
    let analyzer = WhereUsedAnalyzer::new(&graph);

    let wheel_id = ComponentId::new("WHEEL1");

    c.bench_function("where_used", |b| {
        b.iter(|| black_box(analyzer.analyze(&wheel_id).unwrap()))
    });
}

criterion_group!(
    benches,
    bench_graph_construction,
    bench_explosion,
    bench_costing,
    bench_where_used
);
criterion_main!(benches);
