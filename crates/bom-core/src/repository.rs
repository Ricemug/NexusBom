use crate::{BomHeader, BomItem, Component, ComponentId, Result};
use chrono::{DateTime, Utc};

/// Repository trait for BOM data access
/// PLM/ERP systems implement this trait to provide data
pub trait BomRepository: Send + Sync {
    /// Get a component by ID
    fn get_component(&self, id: &ComponentId) -> Result<Component>;

    /// Get multiple components by IDs (batch operation for performance)
    fn get_components(&self, ids: &[ComponentId]) -> Result<Vec<Component>>;

    /// Get BOM header for a component
    fn get_bom_header(
        &self,
        component_id: &ComponentId,
        alternative: Option<&str>,
        effective_date: Option<DateTime<Utc>>,
    ) -> Result<BomHeader>;

    /// Get BOM items (direct children) for a component
    fn get_bom_items(
        &self,
        component_id: &ComponentId,
        effective_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<BomItem>>;

    /// Get all parent-child relationships (for building the full graph)
    fn get_all_bom_items(&self) -> Result<Vec<BomItem>>;

    /// Find all parents of a component (for where-used)
    fn find_parents(&self, component_id: &ComponentId) -> Result<Vec<BomItem>>;
}

/// In-memory repository for testing and simple use cases
pub mod memory {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    #[derive(Clone)]
    pub struct InMemoryRepository {
        components: Arc<RwLock<HashMap<ComponentId, Component>>>,
        bom_headers: Arc<RwLock<HashMap<ComponentId, Vec<BomHeader>>>>,
        bom_items: Arc<RwLock<Vec<BomItem>>>,
    }

    impl InMemoryRepository {
        pub fn new() -> Self {
            Self {
                components: Arc::new(RwLock::new(HashMap::new())),
                bom_headers: Arc::new(RwLock::new(HashMap::new())),
                bom_items: Arc::new(RwLock::new(Vec::new())),
            }
        }

        pub fn add_component(&self, component: Component) {
            let mut components = self.components.write().unwrap();
            components.insert(component.id.clone(), component);
        }

        pub fn add_bom_header(&self, header: BomHeader) {
            let mut headers = self.bom_headers.write().unwrap();
            headers
                .entry(header.component_id.clone())
                .or_insert_with(Vec::new)
                .push(header);
        }

        pub fn add_bom_item(&self, item: BomItem) {
            let mut items = self.bom_items.write().unwrap();
            items.push(item);
        }
    }

    impl Default for InMemoryRepository {
        fn default() -> Self {
            Self::new()
        }
    }

    impl BomRepository for InMemoryRepository {
        fn get_component(&self, id: &ComponentId) -> Result<Component> {
            let components = self.components.read().unwrap();
            components
                .get(id)
                .cloned()
                .ok_or_else(|| crate::BomError::ComponentNotFound(id.0.clone()))
        }

        fn get_components(&self, ids: &[ComponentId]) -> Result<Vec<Component>> {
            let components = self.components.read().unwrap();
            ids.iter()
                .map(|id| {
                    components
                        .get(id)
                        .cloned()
                        .ok_or_else(|| crate::BomError::ComponentNotFound(id.0.clone()))
                })
                .collect()
        }

        fn get_bom_header(
            &self,
            component_id: &ComponentId,
            alternative: Option<&str>,
            effective_date: Option<DateTime<Utc>>,
        ) -> Result<BomHeader> {
            let headers = self.bom_headers.read().unwrap();
            let component_headers = headers
                .get(component_id)
                .ok_or_else(|| crate::BomError::BomNotFound(component_id.0.clone()))?;

            let effective_date = effective_date.unwrap_or_else(Utc::now);

            component_headers
                .iter()
                .find(|h| {
                    // Match alternative
                    let alt_match = match alternative {
                        Some(alt) => h.alternative.as_deref() == Some(alt),
                        None => h.alternative.is_none(),
                    };

                    // Check effectivity
                    let after_start = h.effective_from.as_ref().map_or(true, |from| &effective_date >= from);
                    let before_end = h.effective_to.as_ref().map_or(true, |to| &effective_date <= to);

                    alt_match && after_start && before_end
                })
                .cloned()
                .ok_or_else(|| crate::BomError::BomNotFound(component_id.0.clone()))
        }

        fn get_bom_items(
            &self,
            component_id: &ComponentId,
            effective_date: Option<DateTime<Utc>>,
        ) -> Result<Vec<BomItem>> {
            let items = self.bom_items.read().unwrap();
            let effective_date = effective_date.unwrap_or_else(Utc::now);

            Ok(items
                .iter()
                .filter(|item| {
                    item.parent_id == *component_id && item.is_effective_at(&effective_date)
                })
                .cloned()
                .collect())
        }

        fn get_all_bom_items(&self) -> Result<Vec<BomItem>> {
            let items = self.bom_items.read().unwrap();
            Ok(items.clone())
        }

        fn find_parents(&self, component_id: &ComponentId) -> Result<Vec<BomItem>> {
            let items = self.bom_items.read().unwrap();
            Ok(items
                .iter()
                .filter(|item| item.child_id == *component_id)
                .cloned()
                .collect())
        }
    }
}
