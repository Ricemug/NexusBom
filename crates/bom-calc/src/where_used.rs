use bom_core::{ComponentId, WhereUsedItem, WhereUsedResult, Result};
use bom_graph::{find_all_paths, BomGraph, NodeIndex};
use rayon::prelude::*;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

/// Where-used analyzer (反查分析)
/// Finds all parent assemblies that use a specific component
pub struct WhereUsedAnalyzer<'a> {
    graph: &'a BomGraph,
}

impl<'a> WhereUsedAnalyzer<'a> {
    pub fn new(graph: &'a BomGraph) -> Self {
        Self { graph }
    }

    /// Find all assemblies that use this component
    pub fn analyze(&self, component_id: &ComponentId) -> Result<WhereUsedResult> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        // Find all parents (immediate)
        let direct_parents: Vec<(NodeIndex, Decimal)> = self
            .graph
            .arena()
            .parents(node)
            .map(|(parent_idx, edge)| (parent_idx, edge.effective_quantity))
            .collect();

        // For each parent, find all paths to roots
        let used_in: Vec<WhereUsedItem> = direct_parents
            .par_iter()
            .flat_map(|&(parent_idx, quantity)| {
                let parent_node = self.graph.arena().node(parent_idx)?;

                // Find all paths from roots to this parent
                let mut all_paths_idx = Vec::new();
                for &root in self.graph.roots() {
                    let paths = find_all_paths(self.graph.arena(), root, parent_idx);
                    all_paths_idx.extend(paths);
                }

                // Convert NodeIndex paths to ComponentId paths
                let all_paths: Vec<Vec<ComponentId>> = all_paths_idx
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

                // Calculate max level
                let level = all_paths
                    .iter()
                    .map(|path| path.len())
                    .max()
                    .unwrap_or(1);

                Some(WhereUsedItem {
                    parent_id: parent_node.component_id.clone(),
                    quantity,
                    level,
                    paths: all_paths,
                })
            })
            .collect();

        Ok(WhereUsedResult {
            component: component_id.clone(),
            used_in,
            queried_at: chrono::Utc::now(),
        })
    }

    /// Find all top-level assemblies (roots) that use this component
    pub fn find_root_assemblies(&self, component_id: &ComponentId) -> Result<Vec<ComponentId>> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        let mut root_assemblies = HashSet::new();

        // Find all paths from roots to this component
        for &root in self.graph.roots() {
            let paths = find_all_paths(self.graph.arena(), root, node);
            if !paths.is_empty() {
                if let Some(root_node) = self.graph.arena().node(root) {
                    root_assemblies.insert(root_node.component_id.clone());
                }
            }
        }

        Ok(root_assemblies.into_iter().collect())
    }

    /// Analyze impact of changing this component
    /// Returns all components that would be affected
    pub fn analyze_change_impact(&self, component_id: &ComponentId) -> Result<ImpactAnalysis> {
        let node = self
            .graph
            .find_node(component_id)
            .ok_or_else(|| bom_core::BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        // Find all ancestors (components that use this one, directly or indirectly)
        let mut affected_components = HashSet::new();
        let mut queue = vec![node];
        let mut visited = HashSet::new();

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }

            for (parent_idx, _) in self.graph.arena().parents(current) {
                if let Some(parent_node) = self.graph.arena().node(parent_idx) {
                    affected_components.insert(parent_node.component_id.clone());
                    queue.push(parent_idx);
                }
            }
        }

        // Find all root assemblies affected
        let mut affected_roots = HashSet::new();
        for &root in self.graph.roots() {
            if let Some(root_node) = self.graph.arena().node(root) {
                if affected_components.contains(&root_node.component_id) {
                    affected_roots.insert(root_node.component_id.clone());
                }
            }
        }

        Ok(ImpactAnalysis {
            changed_component: component_id.clone(),
            affected_components: affected_components.into_iter().collect(),
            affected_root_assemblies: affected_roots.into_iter().collect(),
            analyzed_at: chrono::Utc::now(),
        })
    }

    /// Find all components that are common to multiple assemblies
    /// Useful for identifying shared components
    pub fn find_shared_components(&self, assembly_ids: &[ComponentId]) -> Result<Vec<SharedComponent>> {
        let assembly_nodes: Vec<NodeIndex> = assembly_ids
            .iter()
            .filter_map(|id| self.graph.find_node(id))
            .collect();

        if assembly_nodes.is_empty() {
            return Ok(Vec::new());
        }

        // For each assembly, collect all descendant components
        let assembly_descendants: Vec<HashSet<ComponentId>> = assembly_nodes
            .par_iter()
            .map(|&assembly| {
                let mut descendants = HashSet::new();
                let mut stack = vec![assembly];
                let mut visited = HashSet::new();

                while let Some(current) = stack.pop() {
                    if !visited.insert(current) {
                        continue;
                    }

                    if let Some(node) = self.graph.arena().node(current) {
                        if current != assembly {
                            descendants.insert(node.component_id.clone());
                        }
                    }

                    for (child_idx, _) in self.graph.arena().children(current) {
                        stack.push(child_idx);
                    }
                }

                descendants
            })
            .collect();

        // Find components that appear in multiple assemblies
        let mut component_usage: HashMap<ComponentId, Vec<usize>> = HashMap::new();

        for (idx, descendants) in assembly_descendants.iter().enumerate() {
            for component_id in descendants {
                component_usage
                    .entry(component_id.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }

        let shared: Vec<SharedComponent> = component_usage
            .into_iter()
            .filter(|(_, assemblies)| assemblies.len() > 1)
            .map(|(component_id, assembly_indices)| {
                let used_in_assemblies: Vec<ComponentId> = assembly_indices
                    .into_iter()
                    .map(|idx| assembly_ids[idx].clone())
                    .collect();

                SharedComponent {
                    component_id,
                    used_in_count: used_in_assemblies.len(),
                    used_in_assemblies,
                }
            })
            .collect();

        Ok(shared)
    }
}

/// Impact analysis result
#[derive(Debug, Clone)]
pub struct ImpactAnalysis {
    pub changed_component: ComponentId,
    pub affected_components: Vec<ComponentId>,
    pub affected_root_assemblies: Vec<ComponentId>,
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

/// Shared component information
#[derive(Debug, Clone)]
pub struct SharedComponent {
    pub component_id: ComponentId,
    pub used_in_count: usize,
    pub used_in_assemblies: Vec<ComponentId>,
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
    fn test_where_used_simple() {
        let repo = InMemoryRepository::new();

        // A -> B
        // C -> B (B is used by both A and C)
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("C", "B", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let analyzer = WhereUsedAnalyzer::new(&graph);

        let result = analyzer.analyze(&ComponentId::new("B")).unwrap();

        // B should be used by both A and C
        assert_eq!(result.used_in.len(), 2);
        assert!(result
            .used_in
            .iter()
            .any(|item| item.parent_id.as_str() == "A"));
        assert!(result
            .used_in
            .iter()
            .any(|item| item.parent_id.as_str() == "C"));
    }

    #[test]
    fn test_find_root_assemblies() {
        let repo = InMemoryRepository::new();

        // A -> B -> D
        // C -> D
        // D is used by root assemblies A and C
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));
        repo.add_component(create_test_component("D"));

        repo.add_bom_item(create_test_bom_item("A", "B", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 1));
        repo.add_bom_item(create_test_bom_item("C", "D", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let analyzer = WhereUsedAnalyzer::new(&graph);

        let roots = analyzer
            .find_root_assemblies(&ComponentId::new("D"))
            .unwrap();

        // D should be in both A and C assemblies
        assert_eq!(roots.len(), 2);
        assert!(roots.iter().any(|id| id.as_str() == "A"));
        assert!(roots.iter().any(|id| id.as_str() == "C"));
    }

    #[test]
    fn test_change_impact_analysis() {
        let repo = InMemoryRepository::new();

        // A -> B -> D
        //   -> C
        // Changing D should affect B and A
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));
        repo.add_component(create_test_component("D"));

        repo.add_bom_item(create_test_bom_item("A", "B", 1));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let analyzer = WhereUsedAnalyzer::new(&graph);

        let impact = analyzer
            .analyze_change_impact(&ComponentId::new("D"))
            .unwrap();

        // Should affect B and A, but not C
        assert!(impact
            .affected_components
            .iter()
            .any(|id| id.as_str() == "B"));
        assert!(impact
            .affected_components
            .iter()
            .any(|id| id.as_str() == "A"));
        assert!(!impact
            .affected_components
            .iter()
            .any(|id| id.as_str() == "C"));
    }

    #[test]
    fn test_find_shared_components() {
        let repo = InMemoryRepository::new();

        // A -> B -> D
        //   -> C
        // E -> F -> D
        // D is shared between A and E
        repo.add_component(create_test_component("A"));
        repo.add_component(create_test_component("B"));
        repo.add_component(create_test_component("C"));
        repo.add_component(create_test_component("D"));
        repo.add_component(create_test_component("E"));
        repo.add_component(create_test_component("F"));

        repo.add_bom_item(create_test_bom_item("A", "B", 1));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 1));
        repo.add_bom_item(create_test_bom_item("E", "F", 1));
        repo.add_bom_item(create_test_bom_item("F", "D", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();
        let analyzer = WhereUsedAnalyzer::new(&graph);

        let shared = analyzer
            .find_shared_components(&[ComponentId::new("A"), ComponentId::new("E")])
            .unwrap();

        // D should be shared
        assert!(shared
            .iter()
            .any(|sc| sc.component_id.as_str() == "D" && sc.used_in_count == 2));
    }
}
