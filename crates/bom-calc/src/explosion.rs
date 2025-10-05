use bom_core::{ComponentId, ExplosionItem, ExplosionResult, Result};
use bom_graph::{level_grouping, BomGraph, NodeIndex};
use rayon::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Material explosion calculator
/// Explodes a BOM to calculate total quantities needed
pub struct ExplosionCalculator<'a> {
    graph: &'a BomGraph,
}

impl<'a> ExplosionCalculator<'a> {
    pub fn new(graph: &'a BomGraph) -> Self {
        Self { graph }
    }

    /// Explode BOM for a component with given quantity
    /// This performs a full material explosion, calculating total quantities needed
    pub fn explode(
        &self,
        component_id: &ComponentId,
        quantity: Decimal,
    ) -> Result<ExplosionResult> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        let mut quantities: HashMap<NodeIndex, Decimal> = HashMap::new();
        let mut paths: HashMap<NodeIndex, Vec<Vec<NodeIndex>>> = HashMap::new();

        // Initialize root
        quantities.insert(node, quantity);
        paths.insert(node, vec![vec![node]]);

        // Get level grouping for parallel processing
        let levels = level_grouping(self.graph.arena(), &[node]);

        // Process each level from top to bottom (reverse of level_grouping order)
        // Level grouping returns [level 0 = leaves, ..., level N = roots]
        // We need to process from roots to leaves
        for (_level_idx, level_nodes) in levels.iter().rev().enumerate() {
            // Process all nodes in this level in parallel
            let level_results: Vec<_> = level_nodes
                .par_iter()
                .filter_map(|&parent_node| {
                    // Get quantity for this parent
                    let parent_qty = quantities.get(&parent_node)?;

                    // Collect children data
                    let children_data: Vec<_> = self
                        .graph
                        .arena()
                        .children(parent_node)
                        .map(|(child_node, edge)| {
                            let child_qty = edge.effective_quantity * parent_qty;

                            // Build paths: prepend parent to all parent's paths
                            let mut child_paths = Vec::new();
                            if let Some(parent_paths) = paths.get(&parent_node) {
                                for parent_path in parent_paths {
                                    let mut new_path = parent_path.clone();
                                    new_path.push(child_node);
                                    child_paths.push(new_path);
                                }
                            }

                            (child_node, child_qty, child_paths, edge.bom_item.is_phantom)
                        })
                        .collect();

                    Some((parent_node, children_data))
                })
                .collect();

            // Aggregate results (must be done sequentially due to HashMap)
            for (_parent_node, children_data) in level_results {
                for (child_node, child_qty, child_paths, _is_phantom) in children_data {
                    // Accumulate quantity
                    *quantities.entry(child_node).or_insert(Decimal::ZERO) += child_qty;

                    // Accumulate paths
                    paths.entry(child_node).or_insert_with(Vec::new).extend(child_paths);
                }
            }
        }

        // Build result
        let mut items: Vec<ExplosionItem> = quantities
            .into_iter()
            .filter_map(|(node_idx, total_quantity)| {
                let node = self.graph.arena().node(node_idx)?;

                // Calculate level (max path length - 1)
                let level = paths
                    .get(&node_idx)
                    .and_then(|p| p.iter().map(|path| path.len()).max())
                    .map(|len| len.saturating_sub(1))
                    .unwrap_or(0);

                // Convert NodeIndex paths to ComponentId paths
                let component_paths: Vec<Vec<ComponentId>> = paths
                    .get(&node_idx)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|path| {
                        let comp_path: Vec<ComponentId> = path
                            .into_iter()
                            .filter_map(|idx| {
                                self.graph.arena().node(idx).map(|n| n.component_id.clone())
                            })
                            .collect();
                        if comp_path.is_empty() { None } else { Some(comp_path) }
                    })
                    .collect();

                Some(ExplosionItem {
                    component_id: node.component_id.clone(),
                    total_quantity,
                    level,
                    paths: component_paths,
                    is_phantom: false, // TODO: get from component data
                })
            })
            .collect();

        // Sort by level (root first)
        items.sort_by_key(|item| item.level);

        let unique_component_count = items.len();
        let max_depth = items.iter().map(|item| item.level).max().unwrap_or(0);

        Ok(ExplosionResult {
            root_component: component_id.clone(),
            items,
            unique_component_count,
            max_depth,
            calculated_at: chrono::Utc::now(),
        })
    }

    /// Explode BOM only for direct children (single level)
    pub fn explode_single_level(
        &self,
        component_id: &ComponentId,
        quantity: Decimal,
    ) -> Result<Vec<ExplosionItem>> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        let parent_node = self.graph.arena().node(node).unwrap();

        let items: Vec<ExplosionItem> = self
            .graph
            .arena()
            .children(node)
            .map(|(child_node, edge)| {
                let child = self.graph.arena().node(child_node).unwrap();
                let total_quantity = edge.effective_quantity * quantity;

                ExplosionItem {
                    component_id: child.component_id.clone(),
                    total_quantity,
                    level: 1,
                    paths: vec![vec![parent_node.component_id.clone(), child.component_id.clone()]],
                    is_phantom: edge.bom_item.is_phantom,
                }
            })
            .collect();

        Ok(items)
    }

    /// Get flattened BOM (all components at all levels with total quantities)
    /// This is optimized for large BOMs using parallel processing
    pub fn flatten(&self, component_id: &ComponentId) -> Result<HashMap<ComponentId, Decimal>> {
        let result = self.explode(component_id, Decimal::ONE)?;

        let flattened: HashMap<ComponentId, Decimal> = result
            .items
            .into_iter()
            .map(|item| (item.component_id, item.total_quantity))
            .collect();

        Ok(flattened)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::repository::memory::InMemoryRepository;
    use bom_core::*;
    use bom_graph::BomGraph;
    use chrono::Utc;

    fn create_test_component(id: &str) -> Component {
        Component {
            id: ComponentId::new(id),
            description: format!("Component {}", id),
            component_type: ComponentType::FinishedProduct,
            uom: "EA".to_string(),
            standard_cost: Some(Decimal::from(100)),
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
    fn test_simple_explosion() {
        let repo = InMemoryRepository::new();

        // A -> B (qty 2)
        // A -> C (qty 3)
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 3));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = ExplosionCalculator::new(&graph);

        let result = calc.explode(&ComponentId::new("A"), Decimal::from(10)).unwrap();

        // Should have 3 items: A (10), B (20), C (30)
        assert_eq!(result.unique_component_count, 3);

        let b_item = result
            .items
            .iter()
            .find(|item| item.component_id.as_str() == "B")
            .unwrap();
        assert_eq!(b_item.total_quantity, Decimal::from(20));

        let c_item = result
            .items
            .iter()
            .find(|item| item.component_id.as_str() == "C")
            .unwrap();
        assert_eq!(c_item.total_quantity, Decimal::from(30));
    }

    #[test]
    fn test_multilevel_explosion() {
        let repo = InMemoryRepository::new();

        // A -> B (qty 2) -> D (qty 3)
        //   -> C (qty 1) -> D (qty 2)
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));
        repo.add_component(create_test_component("D"));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 3));
        repo.add_bom_item(create_test_bom_item("C", "D", 2));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = ExplosionCalculator::new(&graph);

        let result = calc.explode(&ComponentId::new("A"), Decimal::ONE).unwrap();

        // D should be: (1*2*3) + (1*1*2) = 6 + 2 = 8
        let d_item = result
            .items
            .iter()
            .find(|item| item.component_id.as_str() == "D")
            .unwrap();
        assert_eq!(d_item.total_quantity, Decimal::from(8));

        // D should have 2 paths
        assert_eq!(d_item.paths.len(), 2);
    }

    #[test]
    fn test_single_level_explosion() {
        let repo = InMemoryRepository::new();

        // A -> B (qty 2)
        //   -> C (qty 3)
        // B -> D (qty 5)
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));
        repo.add_component(create_test_component("D"));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 3));
        repo.add_bom_item(create_test_bom_item("B", "D", 5));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let calc = ExplosionCalculator::new(&graph);

        let result = calc
            .explode_single_level(&ComponentId::new("A"), Decimal::ONE)
            .unwrap();

        // Should only have B and C, not D
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|item| item.component_id.as_str() == "B"));
        assert!(result.iter().any(|item| item.component_id.as_str() == "C"));
        assert!(!result.iter().any(|item| item.component_id.as_str() == "D"));
    }
}
