use crate::arena::{Arena, NodeIndex};
use bom_core::{BomError, BomItem, BomRepository, ComponentId, Result};
use std::collections::HashMap;

/// BOM Graph - main interface for BOM operations
pub struct BomGraph {
    /// Underlying arena storage
    arena: Arena,

    /// Root nodes (components that are not children of any other component)
    roots: Vec<NodeIndex>,
}

impl BomGraph {
    /// Create a new empty BOM graph
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            roots: Vec::new(),
        }
    }

    /// Create graph with capacity hint
    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            arena: Arena::with_capacity(node_capacity, edge_capacity),
            roots: Vec::new(),
        }
    }

    /// Build graph from a repository
    pub fn from_repository<R: BomRepository>(repo: &R) -> Result<Self> {
        let all_items = repo.get_all_bom_items()?;

        // Estimate capacity
        let mut component_ids = std::collections::HashSet::new();
        for item in &all_items {
            component_ids.insert(item.parent_id.clone());
            component_ids.insert(item.child_id.clone());
        }

        let node_capacity = component_ids.len();
        let edge_capacity = all_items.len();

        let mut graph = Self::with_capacity(node_capacity, edge_capacity);

        // Add all edges (nodes will be created automatically)
        for item in all_items {
            graph.add_bom_item(item)?;
        }

        // Identify root nodes
        graph.identify_roots();

        Ok(graph)
    }

    /// Build graph for a specific component (load only its BOM tree)
    pub fn from_component<R: BomRepository>(
        repo: &R,
        component_id: &ComponentId,
        effective_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Self> {
        let mut graph = Self::new();
        graph.load_component_tree(repo, component_id, effective_date)?;
        graph.identify_roots();
        Ok(graph)
    }

    /// Recursively load a component's BOM tree
    fn load_component_tree<R: BomRepository>(
        &mut self,
        repo: &R,
        component_id: &ComponentId,
        effective_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<()> {
        // Add the component node
        let _node = self.arena.add_node(component_id.clone());

        // Get BOM items for this component
        let items = repo.get_bom_items(component_id, effective_date)?;

        for item in items {
            // Add the edge
            self.add_bom_item(item.clone())?;

            // Recursively load child if not already loaded
            if self.arena.find_node(&item.child_id).is_none() {
                self.load_component_tree(repo, &item.child_id, effective_date)?;
            }
        }

        Ok(())
    }

    /// Add a BOM item to the graph
    pub fn add_bom_item(&mut self, item: BomItem) -> Result<NodeIndex> {
        // Create or get parent node
        let parent_node = self.arena.add_node(item.parent_id.clone());

        // Create or get child node
        let child_node = self.arena.add_node(item.child_id.clone());

        // Check for self-reference
        if parent_node == child_node {
            return Err(BomError::CircularDependency(format!(
                "Self-reference detected: {}",
                item.parent_id.as_str()
            )));
        }

        // Check for circular dependency (child -> parent path exists)
        if self.arena.has_path(child_node, parent_node) {
            return Err(BomError::CircularDependency(format!(
                "Circular dependency: {} -> {} would create a cycle",
                item.parent_id.as_str(),
                item.child_id.as_str()
            )));
        }

        // Add edge
        self.arena.add_edge(parent_node, child_node, item);

        Ok(parent_node)
    }

    /// Identify root nodes (nodes with no incoming edges)
    fn identify_roots(&mut self) {
        self.roots.clear();
        for (idx, node) in self.arena.nodes().iter().enumerate() {
            if node.incoming.is_empty() {
                self.roots.push(NodeIndex(idx));
            }
        }
    }

    /// Get the underlying arena
    pub fn arena(&self) -> &Arena {
        &self.arena
    }

    /// Get mutable arena
    pub fn arena_mut(&mut self) -> &mut Arena {
        &mut self.arena
    }

    /// Get root nodes
    pub fn roots(&self) -> &[NodeIndex] {
        &self.roots
    }

    /// Find node by component ID
    pub fn find_node(&self, component_id: &ComponentId) -> Option<NodeIndex> {
        self.arena.find_node(component_id)
    }

    /// Get statistics about the graph
    pub fn stats(&self) -> GraphStats {
        GraphStats {
            node_count: self.arena.node_count(),
            edge_count: self.arena.edge_count(),
            root_count: self.roots.len(),
            max_depth: self.calculate_max_depth(),
        }
    }

    /// Calculate maximum depth of the graph
    fn calculate_max_depth(&self) -> usize {
        let mut max_depth = 0;

        for &root in &self.roots {
            let depth = self.calculate_node_depth(root, &mut HashMap::new());
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    /// Calculate depth of a specific node (with memoization)
    fn calculate_node_depth(
        &self,
        node: NodeIndex,
        memo: &mut HashMap<NodeIndex, usize>,
    ) -> usize {
        if let Some(&depth) = memo.get(&node) {
            return depth;
        }

        let depth = self
            .arena
            .children(node)
            .map(|(child, _)| self.calculate_node_depth(child, memo) + 1)
            .max()
            .unwrap_or(0);

        memo.insert(node, depth);
        depth
    }

    /// Clear all cached computation results
    pub fn clear_cache(&mut self) {
        let node_count = self.arena.nodes().len();
        for idx in 0..node_count {
            if let Some(node) = self.arena.node_mut(NodeIndex(idx)) {
                node.cache = crate::arena::NodeCache::default();
            }
        }
    }

    /// Mark a component and its ancestors as dirty for incremental computation
    pub fn mark_dirty(&mut self, component_id: &ComponentId) -> Result<()> {
        let node = self
            .find_node(component_id)
            .ok_or_else(|| BomError::ComponentNotFound(component_id.as_str().to_string()))?;

        self.arena.mark_dirty_recursive(node);
        Ok(())
    }
}

impl Default for BomGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Graph statistics
#[derive(Debug, Clone)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub root_count: usize,
    pub max_depth: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::repository::memory::InMemoryRepository;
    use bom_core::*;
    use chrono::Utc;
    use rust_decimal::Decimal;

    fn create_test_component(id: &str, comp_type: ComponentType) -> Component {
        Component {
            id: ComponentId::new(id),
            description: format!("Component {}", id),
            component_type: comp_type,
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
    fn test_simple_bom_graph() {
        let repo = InMemoryRepository::new();

        // A -> B (qty 2)
        // A -> C (qty 1)
        repo.add_component(create_test_component("A", ComponentType::FinishedProduct));
        repo.add_component(create_test_component("B", ComponentType::RawMaterial));
        repo.add_component(create_test_component("C", ComponentType::RawMaterial));

        repo.add_bom_item(create_test_bom_item("A", "B", 2));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));

        let graph = BomGraph::from_repository(&repo).unwrap();

        let stats = graph.stats();
        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.edge_count, 2);
        assert_eq!(stats.root_count, 1);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = BomGraph::new();

        // Try to create A -> B -> A (circular)
        let item1 = create_test_bom_item("A", "B", 1);
        let item2 = create_test_bom_item("B", "A", 1);

        graph.add_bom_item(item1).unwrap();
        let result = graph.add_bom_item(item2);

        assert!(matches!(result, Err(BomError::CircularDependency(_))));
    }

    #[test]
    fn test_multilevel_bom() {
        let repo = InMemoryRepository::new();

        // A -> B -> D
        //   -> C -> D (shared component)
        repo.add_component(create_test_component("A", ComponentType::FinishedProduct));
        repo.add_component(create_test_component("B", ComponentType::SemiFinished));
        repo.add_component(create_test_component("C", ComponentType::SemiFinished));
        repo.add_component(create_test_component("D", ComponentType::RawMaterial));

        repo.add_bom_item(create_test_bom_item("A", "B", 1));
        repo.add_bom_item(create_test_bom_item("A", "C", 1));
        repo.add_bom_item(create_test_bom_item("B", "D", 2));
        repo.add_bom_item(create_test_bom_item("C", "D", 3));

        let graph = BomGraph::from_repository(&repo).unwrap();

        let stats = graph.stats();
        assert_eq!(stats.node_count, 4);
        assert_eq!(stats.edge_count, 4);
        assert_eq!(stats.max_depth, 2);
    }
}
