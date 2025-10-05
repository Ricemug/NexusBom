use bom_core::{BomRepository, ComponentId, CostBreakdown, ExplosionResult, Result, WhereUsedResult};
use bom_graph::BomGraph;
use rust_decimal::Decimal;

use crate::{CostCalculator, ExplosionCalculator, WhereUsedAnalyzer, ImpactAnalysis, SharedComponent};

/// Unified calculation engine that combines all BOM calculations
/// This is the main entry point for BOM computations
pub struct BomEngine<R: BomRepository> {
    graph: BomGraph,
    repository: R,
}

impl<R: BomRepository> BomEngine<R> {
    /// Create a new BOM engine from a repository
    pub fn new(repository: R) -> Result<Self> {
        let graph = BomGraph::from_repository(&repository)?;
        Ok(Self { graph, repository })
    }

    /// Create engine for a specific component (loads only its BOM tree)
    pub fn for_component(
        repository: R,
        component_id: &ComponentId,
        effective_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Self> {
        let graph = BomGraph::from_component(&repository, component_id, effective_date)?;
        Ok(Self { graph, repository })
    }

    /// Get graph statistics
    pub fn stats(&self) -> bom_graph::GraphStats {
        self.graph.stats()
    }

    // === Material Explosion ===

    /// Explode BOM to calculate material requirements
    pub fn explode(&self, component_id: &ComponentId, quantity: Decimal) -> Result<ExplosionResult> {
        let calculator = ExplosionCalculator::new(&self.graph);
        calculator.explode(component_id, quantity)
    }

    /// Single-level explosion (immediate children only)
    pub fn explode_single_level(
        &self,
        component_id: &ComponentId,
        quantity: Decimal,
    ) -> Result<Vec<bom_core::ExplosionItem>> {
        let calculator = ExplosionCalculator::new(&self.graph);
        calculator.explode_single_level(component_id, quantity)
    }

    /// Get flattened BOM (all components with total quantities)
    pub fn flatten(&self, component_id: &ComponentId) -> Result<std::collections::HashMap<ComponentId, Decimal>> {
        let calculator = ExplosionCalculator::new(&self.graph);
        calculator.flatten(component_id)
    }

    // === Cost Calculation ===

    /// Calculate cost breakdown for a component
    pub fn calculate_cost(&self, component_id: &ComponentId) -> Result<CostBreakdown> {
        let calculator = CostCalculator::new(&self.graph, &self.repository);
        calculator.calculate_cost(component_id)
    }

    /// Calculate costs for all components in the BOM
    pub fn calculate_all_costs(&self) -> Result<std::collections::HashMap<ComponentId, CostBreakdown>> {
        let calculator = CostCalculator::new(&self.graph, &self.repository);
        calculator.calculate_all_costs(self.graph.roots())
    }

    /// Calculate total cost for producing a quantity
    pub fn calculate_rollup(&self, component_id: &ComponentId, quantity: Decimal) -> Result<Decimal> {
        let calculator = CostCalculator::new(&self.graph, &self.repository);
        calculator.calculate_rollup(component_id, quantity)
    }

    /// Analyze cost drivers (what contributes most to cost)
    pub fn analyze_cost_drivers(&self, component_id: &ComponentId) -> Result<Vec<crate::CostDriver>> {
        let calculator = CostCalculator::new(&self.graph, &self.repository);
        calculator.analyze_cost_drivers(component_id)
    }

    // === Where-Used Analysis ===

    /// Find where a component is used
    pub fn where_used(&self, component_id: &ComponentId) -> Result<WhereUsedResult> {
        let analyzer = WhereUsedAnalyzer::new(&self.graph);
        analyzer.analyze(component_id)
    }

    /// Find root assemblies that use a component
    pub fn find_root_assemblies(&self, component_id: &ComponentId) -> Result<Vec<ComponentId>> {
        let analyzer = WhereUsedAnalyzer::new(&self.graph);
        analyzer.find_root_assemblies(component_id)
    }

    /// Analyze impact of changing a component
    pub fn analyze_change_impact(&self, component_id: &ComponentId) -> Result<ImpactAnalysis> {
        let analyzer = WhereUsedAnalyzer::new(&self.graph);
        analyzer.analyze_change_impact(component_id)
    }

    /// Find components shared across multiple assemblies
    pub fn find_shared_components(&self, assembly_ids: &[ComponentId]) -> Result<Vec<SharedComponent>> {
        let analyzer = WhereUsedAnalyzer::new(&self.graph);
        analyzer.find_shared_components(assembly_ids)
    }

    // === Graph Operations ===

    /// Get the underlying graph (for advanced operations)
    pub fn graph(&self) -> &BomGraph {
        &self.graph
    }

    /// Get the repository
    pub fn repository(&self) -> &R {
        &self.repository
    }

    /// Validate the BOM for circular dependencies and other issues
    pub fn validate(&self) -> Result<()> {
        bom_graph::validate_graph(self.graph.arena())
    }

    /// Mark a component as dirty for incremental recomputation
    pub fn mark_dirty(&mut self, component_id: &ComponentId) -> Result<()> {
        self.graph.mark_dirty(component_id)
    }

    /// Clear all cached computation results
    pub fn clear_cache(&mut self) {
        self.graph.clear_cache()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::repository::memory::InMemoryRepository;
    use bom_core::*;
    use chrono::Utc;

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

    #[test]
    fn test_integrated_workflow() {
        let repo = InMemoryRepository::new();

        // Build a simple BOM:
        // A (cost 100) -> B (cost 50, qty 2) -> D (cost 10, qty 3)
        //              -> C (cost 30, qty 1)
        repo.add_component(create_test_component("A", 100));
        repo.add_component(create_test_component("B", 50));
        repo.add_component(create_test_component("C", 30));
        repo.add_component(create_test_component("D", 10));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 3));

        let engine = BomEngine::new(repo).unwrap();

        // Test explosion
        let explosion = engine.explode(&ComponentId::new("A"), Decimal::ONE).unwrap();
        assert_eq!(explosion.unique_component_count, 4);

        // Test cost calculation
        let cost = engine.calculate_cost(&ComponentId::new("A")).unwrap();
        assert!(cost.total_cost > Decimal::ZERO);

        // Test where-used
        let where_used = engine.where_used(&ComponentId::new("D")).unwrap();
        assert!(!where_used.used_in.is_empty());

        // Test validation
        assert!(engine.validate().is_ok());
    }

    #[test]
    fn test_validation_catches_cycles() {
        let repo = InMemoryRepository::new();

        repo.add_component(create_test_component("A", 100));
        repo.add_component(create_test_component("B", 50));

        // Create a cycle: A -> B -> A
        repo.add_bom_item(create_test_bom_item("A", "B", 1));
        repo.add_bom_item(create_test_bom_item("B", "A", 1));

        // Should fail to create engine due to cycle
        let result = BomEngine::new(repo);
        assert!(result.is_err());
    }
}
