use crate::arena::{Arena, NodeIndex};
use bom_core::{BomError, ComponentId, Result};
use std::collections::HashSet;

/// Detect cycles in the BOM graph
pub struct CycleDetector<'a> {
    arena: &'a Arena,
}

impl<'a> CycleDetector<'a> {
    pub fn new(arena: &'a Arena) -> Self {
        Self { arena }
    }

    /// Check if the graph contains any cycles
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for (idx, _) in self.arena.nodes().iter().enumerate() {
            let node = NodeIndex(idx);
            if !visited.contains(&node) {
                if self.dfs_cycle(node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    /// Find all cycles in the graph
    pub fn find_cycles(&self) -> Vec<Vec<NodeIndex>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for (idx, _) in self.arena.nodes().iter().enumerate() {
            let node = NodeIndex(idx);
            if !visited.contains(&node) {
                self.dfs_find_cycles(
                    node,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    /// Validate that adding an edge would not create a cycle
    pub fn would_create_cycle(&self, from: NodeIndex, to: NodeIndex) -> bool {
        // If there's already a path from 'to' to 'from', adding edge from->to creates cycle
        self.arena.has_path(to, from)
    }

    /// DFS to detect cycle (returns true if cycle found)
    fn dfs_cycle(
        &self,
        node: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        rec_stack: &mut HashSet<NodeIndex>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        for (child, _) in self.arena.children(node) {
            if !visited.contains(&child) {
                if self.dfs_cycle(child, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&child) {
                // Found a back edge (cycle)
                return true;
            }
        }

        rec_stack.remove(&node);
        false
    }

    /// DFS to find all cycles
    fn dfs_find_cycles(
        &self,
        node: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        rec_stack: &mut HashSet<NodeIndex>,
        path: &mut Vec<NodeIndex>,
        cycles: &mut Vec<Vec<NodeIndex>>,
    ) {
        visited.insert(node);
        rec_stack.insert(node);
        path.push(node);

        for (child, _) in self.arena.children(node) {
            if !visited.contains(&child) {
                self.dfs_find_cycles(child, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(&child) {
                // Found a cycle, extract it from path
                if let Some(cycle_start) = path.iter().position(|&n| n == child) {
                    let cycle = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        path.pop();
        rec_stack.remove(&node);
    }

    /// Get a human-readable description of a cycle
    pub fn describe_cycle(&self, cycle: &[NodeIndex]) -> Vec<ComponentId> {
        cycle
            .iter()
            .filter_map(|&idx| self.arena.node(idx))
            .map(|node| node.component_id.clone())
            .collect()
    }
}

/// Validate BOM graph for common issues
pub fn validate_graph(arena: &Arena) -> Result<()> {
    let detector = CycleDetector::new(arena);

    // Check for cycles
    if detector.has_cycle() {
        let cycles = detector.find_cycles();
        let cycle_descriptions: Vec<String> = cycles
            .iter()
            .map(|cycle| {
                detector
                    .describe_cycle(cycle)
                    .iter()
                    .map(|id| id.as_str())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            })
            .collect();

        return Err(BomError::CircularDependency(format!(
            "Found {} cycle(s): {}",
            cycles.len(),
            cycle_descriptions.join("; ")
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;
    use bom_core::{BomItem, ComponentId};
    use rust_decimal::Decimal;

    fn create_test_bom_item(parent: &str, child: &str) -> BomItem {
        BomItem {
            id: uuid::Uuid::new_v4(),
            parent_id: ComponentId::new(parent),
            child_id: ComponentId::new(child),
            quantity: Decimal::ONE,
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
    fn test_no_cycle() {
        let mut arena = Arena::new();

        // A -> B -> C (no cycle)
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(b, c, create_test_bom_item("B", "C"));

        let detector = CycleDetector::new(&arena);
        assert!(!detector.has_cycle());
        assert!(validate_graph(&arena).is_ok());
    }

    #[test]
    fn test_simple_cycle() {
        let mut arena = Arena::new();

        // A -> B -> A (cycle)
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(b, a, create_test_bom_item("B", "A"));

        let detector = CycleDetector::new(&arena);
        assert!(detector.has_cycle());

        let cycles = detector.find_cycles();
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_complex_cycle() {
        let mut arena = Arena::new();

        // A -> B -> C -> D -> B (cycle in middle)
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));
        let d = arena.add_node(ComponentId::new("D"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(b, c, create_test_bom_item("B", "C"));
        arena.add_edge(c, d, create_test_bom_item("C", "D"));
        arena.add_edge(d, b, create_test_bom_item("D", "B"));

        let detector = CycleDetector::new(&arena);
        assert!(detector.has_cycle());
    }

    #[test]
    fn test_would_create_cycle() {
        let mut arena = Arena::new();

        // A -> B -> C
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(b, c, create_test_bom_item("B", "C"));

        let detector = CycleDetector::new(&arena);

        // Adding C -> A would create cycle
        assert!(detector.would_create_cycle(c, a));

        // Adding C -> B would create cycle
        assert!(detector.would_create_cycle(c, b));

        // Adding A -> C would not create cycle (already exists as path)
        assert!(!detector.would_create_cycle(a, c));
    }
}
