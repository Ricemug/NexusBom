use crate::arena::{Arena, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

/// Traversal order for BOM graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalOrder {
    /// Depth-first search
    DepthFirst,
    /// Breadth-first search
    BreadthFirst,
    /// Topological order (bottom-up: leaves first)
    TopologicalBottomUp,
    /// Reverse topological order (top-down: roots first)
    TopologicalTopDown,
}

/// Iterator for traversing the BOM graph
pub struct Traversal<'a> {
    arena: &'a Arena,
    order: TraversalOrder,
    visited: HashSet<NodeIndex>,
    stack: Vec<NodeIndex>,
    queue: VecDeque<NodeIndex>,
}

impl<'a> Traversal<'a> {
    /// Create a new traversal starting from given roots
    pub fn new(arena: &'a Arena, roots: &[NodeIndex], order: TraversalOrder) -> Self {
        let mut traversal = Self {
            arena,
            order,
            visited: HashSet::new(),
            stack: Vec::new(),
            queue: VecDeque::new(),
        };

        match order {
            TraversalOrder::DepthFirst => {
                traversal.stack.extend(roots.iter().rev());
            }
            TraversalOrder::BreadthFirst => {
                traversal.queue.extend(roots);
            }
            TraversalOrder::TopologicalBottomUp | TraversalOrder::TopologicalTopDown => {
                // Will be computed lazily
                let topo = topological_sort(arena, roots);
                if order == TraversalOrder::TopologicalBottomUp {
                    traversal.stack.extend(topo.into_iter().rev());
                } else {
                    traversal.stack.extend(topo);
                }
            }
        }

        traversal
    }
}

impl<'a> Iterator for Traversal<'a> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.order {
            TraversalOrder::DepthFirst | TraversalOrder::TopologicalBottomUp | TraversalOrder::TopologicalTopDown => {
                while let Some(node) = self.stack.pop() {
                    if self.visited.insert(node) {
                        // Add children to stack (for DFS)
                        if self.order == TraversalOrder::DepthFirst {
                            for (child, _) in self.arena.children(node) {
                                if !self.visited.contains(&child) {
                                    self.stack.push(child);
                                }
                            }
                        }
                        return Some(node);
                    }
                }
                None
            }
            TraversalOrder::BreadthFirst => {
                while let Some(node) = self.queue.pop_front() {
                    if self.visited.insert(node) {
                        // Add children to queue
                        for (child, _) in self.arena.children(node) {
                            if !self.visited.contains(&child) {
                                self.queue.push_back(child);
                            }
                        }
                        return Some(node);
                    }
                }
                None
            }
        }
    }
}

/// Compute topological sort of the graph (bottom-up: leaves first)
/// Only includes nodes reachable from the given roots
/// Uses Kahn's algorithm
pub fn topological_sort(arena: &Arena, roots: &[NodeIndex]) -> Vec<NodeIndex> {
    // First, find all nodes reachable from roots
    let mut reachable = HashSet::new();
    let mut stack: Vec<NodeIndex> = roots.to_vec();

    while let Some(node) = stack.pop() {
        if reachable.insert(node) {
            for (child, _) in arena.children(node) {
                if !reachable.contains(&child) {
                    stack.push(child);
                }
            }
        }
    }

    // Calculate in-degrees only for reachable nodes
    let mut in_degree: HashMap<NodeIndex, usize> = HashMap::new();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    for &node_idx in &reachable {
        if let Some(node) = arena.node(node_idx) {
            // Count incoming edges from other reachable nodes
            let degree = node.incoming.iter()
                .filter(|&&edge_idx| {
                    arena.edge(edge_idx)
                        .map(|e| reachable.contains(&e.source))
                        .unwrap_or(false)
                })
                .count();

            in_degree.insert(node_idx, degree);

            if degree == 0 {
                queue.push_back(node_idx);
            }
        }
    }

    // Kahn's algorithm (produces top-down order)
    while let Some(node) = queue.pop_front() {
        result.push(node);

        for (child, _) in arena.children(node) {
            if !reachable.contains(&child) {
                continue;
            }

            if let Some(degree) = in_degree.get_mut(&child) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(child);
                }
            }
        }
    }

    // Reverse to get bottom-up order (leaves first)
    result.reverse();
    result
}

/// Group nodes by level (0 = leaves, increasing towards roots)
/// Nodes at the same level can be processed in parallel
pub fn level_grouping(arena: &Arena, roots: &[NodeIndex]) -> Vec<Vec<NodeIndex>> {
    let mut levels: HashMap<NodeIndex, usize> = HashMap::new();
    let mut max_level = 0;

    // Calculate levels using topological sort
    let topo = topological_sort(arena, roots);

    for node in topo {
        // Level is max(children_levels) + 1
        let level = arena
            .children(node)
            .filter_map(|(child, _)| levels.get(&child))
            .max()
            .map(|l| l + 1)
            .unwrap_or(0);

        levels.insert(node, level);
        max_level = max_level.max(level);
    }

    // Group nodes by level
    let mut result = vec![Vec::new(); max_level + 1];
    for (node, level) in levels {
        result[level].push(node);
    }

    result
}

/// Find all paths from source to target
pub fn find_all_paths(
    arena: &Arena,
    source: NodeIndex,
    target: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();
    let mut current_path = Vec::new();
    let mut visited = HashSet::new();

    dfs_paths(arena, source, target, &mut current_path, &mut visited, &mut paths);

    paths
}

fn dfs_paths(
    arena: &Arena,
    current: NodeIndex,
    target: NodeIndex,
    path: &mut Vec<NodeIndex>,
    visited: &mut HashSet<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);
    visited.insert(current);

    if current == target {
        paths.push(path.clone());
    } else {
        for (child, _) in arena.children(current) {
            if !visited.contains(&child) {
                dfs_paths(arena, child, target, path, visited, paths);
            }
        }
    }

    path.pop();
    visited.remove(&current);
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
    fn test_topological_sort() {
        let mut arena = Arena::new();

        // A -> B -> D
        //   -> C -> D
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));
        let d = arena.add_node(ComponentId::new("D"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(a, c, create_test_bom_item("A", "C"));
        arena.add_edge(b, d, create_test_bom_item("B", "D"));
        arena.add_edge(c, d, create_test_bom_item("C", "D"));

        let topo = topological_sort(&arena, &[a]);

        // D should come before B and C, B and C should come before A
        let d_pos = topo.iter().position(|&n| n == d).unwrap();
        let b_pos = topo.iter().position(|&n| n == b).unwrap();
        let c_pos = topo.iter().position(|&n| n == c).unwrap();
        let a_pos = topo.iter().position(|&n| n == a).unwrap();

        assert!(d_pos < b_pos);
        assert!(d_pos < c_pos);
        assert!(b_pos < a_pos);
        assert!(c_pos < a_pos);
    }

    #[test]
    fn test_level_grouping() {
        let mut arena = Arena::new();

        // A -> B -> D
        //   -> C -> D
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));
        let d = arena.add_node(ComponentId::new("D"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(a, c, create_test_bom_item("A", "C"));
        arena.add_edge(b, d, create_test_bom_item("B", "D"));
        arena.add_edge(c, d, create_test_bom_item("C", "D"));

        let levels = level_grouping(&arena, &[a]);

        // Level 0: D (leaf)
        // Level 1: B, C
        // Level 2: A (root)
        assert_eq!(levels.len(), 3);
        assert!(levels[0].contains(&d));
        assert!(levels[1].contains(&b));
        assert!(levels[1].contains(&c));
        assert!(levels[2].contains(&a));
    }

    #[test]
    fn test_find_all_paths() {
        let mut arena = Arena::new();

        // A -> B -> D
        //   -> C -> D
        let a = arena.add_node(ComponentId::new("A"));
        let b = arena.add_node(ComponentId::new("B"));
        let c = arena.add_node(ComponentId::new("C"));
        let d = arena.add_node(ComponentId::new("D"));

        arena.add_edge(a, b, create_test_bom_item("A", "B"));
        arena.add_edge(a, c, create_test_bom_item("A", "C"));
        arena.add_edge(b, d, create_test_bom_item("B", "D"));
        arena.add_edge(c, d, create_test_bom_item("C", "D"));

        let paths = find_all_paths(&arena, a, d);

        // Should find 2 paths: A->B->D and A->C->D
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|p| p.len() == 3 && p[1] == b));
        assert!(paths.iter().any(|p| p.len() == 3 && p[1] == c));
    }
}
