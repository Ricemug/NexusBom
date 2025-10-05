use bom_core::{BomItem, ComponentId};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Node index in the arena
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

/// Edge index in the arena
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeIndex(pub usize);

/// Node data in the BOM graph
#[derive(Debug, Clone)]
pub struct Node {
    /// Component ID
    pub component_id: ComponentId,

    /// Incoming edges (parents)
    pub incoming: Vec<EdgeIndex>,

    /// Outgoing edges (children)
    pub outgoing: Vec<EdgeIndex>,

    /// Cached computation results
    pub cache: NodeCache,

    /// Dirty flag for incremental computation
    pub dirty: bool,

    /// Version number for change tracking
    pub version: u64,
}

/// Cached computation results for a node
#[derive(Debug, Clone, Default)]
pub struct NodeCache {
    /// Cached total material cost (from all child components)
    pub total_material_cost: Option<Decimal>,

    /// Cached explosion quantity at this level
    pub explosion_quantity: Option<Decimal>,

    /// BOM level/depth (0 = leaf nodes, increases towards root)
    pub level: Option<usize>,
}

/// Edge data representing parent-child relationship
#[derive(Debug, Clone)]
pub struct Edge {
    /// Source node (parent)
    pub source: NodeIndex,

    /// Target node (child)
    pub target: NodeIndex,

    /// Original BOM item data
    pub bom_item: BomItem,

    /// Cached effective quantity
    pub effective_quantity: Decimal,
}

/// Arena-based graph structure for BOM
/// Uses contiguous memory for better cache locality
pub struct Arena {
    /// All nodes stored in a contiguous vector
    nodes: Vec<Node>,

    /// All edges stored in a contiguous vector
    edges: Vec<Edge>,

    /// Map from ComponentId to NodeIndex for fast lookup
    component_index: HashMap<ComponentId, NodeIndex>,

    /// Free list for deleted nodes (for reuse)
    free_nodes: Vec<NodeIndex>,

    /// Free list for deleted edges (for reuse)
    free_edges: Vec<EdgeIndex>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            component_index: HashMap::new(),
            free_nodes: Vec::new(),
            free_edges: Vec::new(),
        }
    }

    /// Create arena with capacity
    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_capacity),
            edges: Vec::with_capacity(edge_capacity),
            component_index: HashMap::with_capacity(node_capacity),
            free_nodes: Vec::new(),
            free_edges: Vec::new(),
        }
    }

    /// Add a new node to the arena
    pub fn add_node(&mut self, component_id: ComponentId) -> NodeIndex {
        // Check if node already exists
        if let Some(&idx) = self.component_index.get(&component_id) {
            return idx;
        }

        let index = if let Some(free_idx) = self.free_nodes.pop() {
            // Reuse a freed node
            self.nodes[free_idx.0] = Node {
                component_id: component_id.clone(),
                incoming: Vec::new(),
                outgoing: Vec::new(),
                cache: NodeCache::default(),
                dirty: true,
                version: 0,
            };
            free_idx
        } else {
            // Allocate new node
            let idx = NodeIndex(self.nodes.len());
            self.nodes.push(Node {
                component_id: component_id.clone(),
                incoming: Vec::new(),
                outgoing: Vec::new(),
                cache: NodeCache::default(),
                dirty: true,
                version: 0,
            });
            idx
        };

        self.component_index.insert(component_id, index);
        index
    }

    /// Add an edge between two nodes
    pub fn add_edge(
        &mut self,
        parent: NodeIndex,
        child: NodeIndex,
        bom_item: BomItem,
    ) -> EdgeIndex {
        let effective_quantity = bom_item.effective_quantity();

        let edge_idx = if let Some(free_idx) = self.free_edges.pop() {
            // Reuse a freed edge
            self.edges[free_idx.0] = Edge {
                source: parent,
                target: child,
                bom_item,
                effective_quantity,
            };
            free_idx
        } else {
            // Allocate new edge
            let idx = EdgeIndex(self.edges.len());
            self.edges.push(Edge {
                source: parent,
                target: child,
                bom_item,
                effective_quantity,
            });
            idx
        };

        // Update adjacency lists
        self.nodes[parent.0].outgoing.push(edge_idx);
        self.nodes[child.0].incoming.push(edge_idx);

        // Mark parent as dirty (needs recomputation)
        self.mark_dirty_recursive(parent);

        edge_idx
    }

    /// Get node by index
    #[inline]
    pub fn node(&self, index: NodeIndex) -> Option<&Node> {
        self.nodes.get(index.0)
    }

    /// Get mutable node by index
    #[inline]
    pub fn node_mut(&mut self, index: NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(index.0)
    }

    /// Get edge by index
    #[inline]
    pub fn edge(&self, index: EdgeIndex) -> Option<&Edge> {
        self.edges.get(index.0)
    }

    /// Get mutable edge by index
    #[inline]
    pub fn edge_mut(&mut self, index: EdgeIndex) -> Option<&mut Edge> {
        self.edges.get_mut(index.0)
    }

    /// Find node index by component ID
    pub fn find_node(&self, component_id: &ComponentId) -> Option<NodeIndex> {
        self.component_index.get(component_id).copied()
    }

    /// Get all nodes
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Get all edges
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len() - self.free_nodes.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len() - self.free_edges.len()
    }

    /// Mark a node and all its ancestors as dirty (for incremental computation)
    pub fn mark_dirty_recursive(&mut self, node: NodeIndex) {
        if let Some(n) = self.node_mut(node) {
            if n.dirty {
                // Already marked, stop recursion to avoid cycles
                return;
            }
            n.dirty = true;
            n.version += 1;

            // Collect incoming edges before recursive calls
            let incoming: Vec<_> = n.incoming.clone();

            for edge_idx in incoming {
                if let Some(edge) = self.edge(edge_idx) {
                    let parent = edge.source;
                    self.mark_dirty_recursive(parent);
                }
            }
        }
    }

    /// Clear all dirty flags
    pub fn clear_dirty_flags(&mut self) {
        for node in &mut self.nodes {
            node.dirty = false;
        }
    }

    /// Get children of a node
    pub fn children(&self, node: NodeIndex) -> impl Iterator<Item = (NodeIndex, &Edge)> + '_ {
        self.node(node)
            .into_iter()
            .flat_map(|n| n.outgoing.iter())
            .filter_map(|&edge_idx| {
                self.edge(edge_idx)
                    .map(|edge| (edge.target, edge))
            })
    }

    /// Get parents of a node
    pub fn parents(&self, node: NodeIndex) -> impl Iterator<Item = (NodeIndex, &Edge)> + '_ {
        self.node(node)
            .into_iter()
            .flat_map(|n| n.incoming.iter())
            .filter_map(|&edge_idx| {
                self.edge(edge_idx)
                    .map(|edge| (edge.source, edge))
            })
    }

    /// Check if there's a path from source to target (for cycle detection)
    pub fn has_path(&self, source: NodeIndex, target: NodeIndex) -> bool {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![source];

        while let Some(current) = stack.pop() {
            if current == target {
                return true;
            }

            if visited[current.0] {
                continue;
            }
            visited[current.0] = true;

            for (child, _) in self.children(current) {
                if !visited[child.0] {
                    stack.push(child);
                }
            }
        }

        false
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_basic_operations() {
        let mut arena = Arena::new();

        let comp_a = ComponentId::new("A");
        let comp_b = ComponentId::new("B");

        let node_a = arena.add_node(comp_a.clone());
        let node_b = arena.add_node(comp_b.clone());

        assert_eq!(arena.node_count(), 2);
        assert_eq!(arena.find_node(&comp_a), Some(node_a));
        assert_eq!(arena.find_node(&comp_b), Some(node_b));
    }

    #[test]
    fn test_arena_edges() {
        let mut arena = Arena::new();

        let node_a = arena.add_node(ComponentId::new("A"));
        let node_b = arena.add_node(ComponentId::new("B"));

        let bom_item = BomItem {
            id: uuid::Uuid::new_v4(),
            parent_id: ComponentId::new("A"),
            child_id: ComponentId::new("B"),
            quantity: Decimal::from(2),
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
        };

        arena.add_edge(node_a, node_b, bom_item);

        assert_eq!(arena.edge_count(), 1);
        assert_eq!(arena.children(node_a).count(), 1);
        assert_eq!(arena.parents(node_b).count(), 1);
    }
}
