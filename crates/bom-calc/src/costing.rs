use bom_core::{BomRepository, ComponentId, CostBreakdown, Result};
use bom_graph::{level_grouping, BomGraph, NodeIndex};
use rayon::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Cost calculation engine
pub struct CostCalculator<'a, R: BomRepository> {
    graph: &'a BomGraph,
    repository: &'a R,
}

impl<'a, R: BomRepository> CostCalculator<'a, R> {
    pub fn new(graph: &'a BomGraph, repository: &'a R) -> Self {
        Self { graph, repository }
    }

    /// Calculate total cost for a component
    /// Uses cached results when available (incremental computation)
    pub fn calculate_cost(&self, component_id: &ComponentId) -> Result<CostBreakdown> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        // Check if we have cached result and node is not dirty
        if let Some(n) = self.graph.arena().node(node) {
            if !n.dirty {
                if let Some(cached_cost) = n.cache.total_material_cost {
                    // Return cached result
                    return Ok(CostBreakdown {
                        component_id: component_id.clone(),
                        material_cost: cached_cost,
                        labor_cost: Decimal::ZERO, // TODO: implement
                        overhead_cost: Decimal::ZERO, // TODO: implement
                        subcontract_cost: Decimal::ZERO, // TODO: implement
                        total_cost: cached_cost,
                        calculated_at: chrono::Utc::now(),
                    });
                }
            }
        }

        // Need to calculate
        let cost_map = self.calculate_all_costs(&[node])?;

        cost_map
            .get(component_id)
            .cloned()
            .ok_or_else(|| bom_core::BomError::CalculationError("Cost not found".to_string()))
    }

    /// Calculate costs for all components in the BOM tree
    /// Uses parallel processing at each level
    pub fn calculate_all_costs(
        &self,
        roots: &[NodeIndex],
    ) -> Result<HashMap<ComponentId, CostBreakdown>> {
        let mut cost_map: HashMap<ComponentId, CostBreakdown> = HashMap::new();

        // Get component IDs and load all components in batch
        let component_ids: Vec<ComponentId> = self
            .graph
            .arena()
            .nodes()
            .iter()
            .map(|n| n.component_id.clone())
            .collect();

        let components = self.repository.get_components(&component_ids)?;
        let component_data: HashMap<ComponentId, _> = components
            .into_iter()
            .map(|c| (c.id.clone(), c))
            .collect();

        // Process level by level (bottom-up)
        let levels = level_grouping(self.graph.arena(), roots);

        for level_nodes in levels {
            // Process all nodes in this level in parallel
            let level_costs: Vec<_> = level_nodes
                .par_iter()
                .filter_map(|&node_idx| {
                    let node = self.graph.arena().node(node_idx)?;
                    let component = component_data.get(&node.component_id)?;

                    // Get own material cost
                    let own_cost = component.standard_cost.unwrap_or(Decimal::ZERO);

                    // Sum up children's costs
                    let children_cost: Decimal = self
                        .graph
                        .arena()
                        .children(node_idx)
                        .filter_map(|(child_idx, edge)| {
                            let child_node = self.graph.arena().node(child_idx)?;
                            let child_cost_breakdown = cost_map.get(&child_node.component_id)?;

                            // Child total cost * quantity
                            Some(child_cost_breakdown.total_cost * edge.effective_quantity)
                        })
                        .sum();

                    let total_material_cost = own_cost + children_cost;

                    Some((
                        node.component_id.clone(),
                        CostBreakdown {
                            component_id: node.component_id.clone(),
                            material_cost: total_material_cost,
                            labor_cost: Decimal::ZERO, // TODO: implement
                            overhead_cost: Decimal::ZERO, // TODO: implement
                            subcontract_cost: Decimal::ZERO, // TODO: implement
                            total_cost: total_material_cost,
                            calculated_at: chrono::Utc::now(),
                        },
                    ))
                })
                .collect();

            // Add to cost map
            cost_map.extend(level_costs);
        }

        Ok(cost_map)
    }

    /// Calculate cost rollup (total cost for producing a quantity)
    pub fn calculate_rollup(
        &self,
        component_id: &ComponentId,
        quantity: Decimal,
    ) -> Result<Decimal> {
        let cost_breakdown = self.calculate_cost(component_id)?;
        Ok(cost_breakdown.total_cost * quantity)
    }

    /// Calculate where the cost comes from (cost breakdown by component)
    pub fn analyze_cost_drivers(
        &self,
        component_id: &ComponentId,
    ) -> Result<Vec<CostDriver>> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        let cost_map = self.calculate_all_costs(&[node])?;

        let total_cost = cost_map
            .get(component_id)
            .map(|c| c.total_cost)
            .unwrap_or(Decimal::ZERO);

        let mut drivers: Vec<CostDriver> = cost_map
            .into_iter()
            .filter(|(id, _)| id != component_id) // Exclude root
            .map(|(id, breakdown)| {
                let percentage = if total_cost > Decimal::ZERO {
                    (breakdown.total_cost / total_cost) * Decimal::from(100)
                } else {
                    Decimal::ZERO
                };

                CostDriver {
                    component_id: id,
                    cost: breakdown.total_cost,
                    percentage,
                }
            })
            .collect();

        // Sort by cost (descending)
        drivers.sort_by(|a, b| b.cost.cmp(&a.cost));

        Ok(drivers)
    }
}

/// Cost driver analysis result
#[derive(Debug, Clone)]
pub struct CostDriver {
    pub component_id: ComponentId,
    pub cost: Decimal,
    pub percentage: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::repository::memory::InMemoryRepository;
    use bom_core::*;
    use bom_graph::BomGraph;
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
    fn test_simple_cost_calculation() {
        let repo = InMemoryRepository::new();

        // A (cost 100) -> B (cost 50, qty 2)
        //              -> C (cost 30, qty 1)
        // Total cost of A = 100 + (50*2) + (30*1) = 230
        repo.add_component(create_test_component("A", 100));
        repo.add_component(create_test_component("B", 50));
        repo.add_component(create_test_component("C", 30));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = CostCalculator::new(&graph, &repo);

        let cost = calc.calculate_cost(&ComponentId::new("A")).unwrap();

        assert_eq!(cost.total_cost, Decimal::from(230));
    }

    #[test]
    fn test_multilevel_cost_calculation() {
        let repo = InMemoryRepository::new();

        // A (cost 100) -> B (cost 50, qty 2) -> D (cost 10, qty 3)
        //              -> C (cost 30, qty 1)
        // Cost of B = 50 + (10*3) = 80
        // Cost of A = 100 + (80*2) + (30*1) = 290
        repo.add_component(create_test_component("A", 100));
        repo.add_component(create_test_component("B", 50));
        repo.add_component(create_test_component("C", 30));
        repo.add_component(create_test_component("D", 10));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 3));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = CostCalculator::new(&graph, &repo);

        let cost_a = calc.calculate_cost(&ComponentId::new("A")).unwrap();
        let cost_b = calc.calculate_cost(&ComponentId::new("B")).unwrap();

        assert_eq!(cost_b.total_cost, Decimal::from(80));
        assert_eq!(cost_a.total_cost, Decimal::from(290));
    }

    #[test]
    fn test_cost_rollup() {
        let repo = InMemoryRepository::new();

        // A (cost 100) -> B (cost 50, qty 2)
        // Total cost = 200, for qty 10 = 2000
        repo.add_component(create_test_component("A", 100));
        repo.add_component(create_test_component("B", 50));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = CostCalculator::new(&graph, &repo);

        let rollup = calc
            .calculate_rollup(&ComponentId::new("A"), Decimal::from(10))
            .unwrap();

        assert_eq!(rollup, Decimal::from(2000));
    }
}
