//! BOM Caching Layer
//!
//! Provides two-tier caching for BOM calculations:
//! - L1: Fast in-memory cache using moka
//! - L2: Persistent cache using redb

pub mod memory_cache;
pub mod persistent_cache;

pub use memory_cache::*;
pub use persistent_cache::*;

use bom_core::{ComponentId, CostBreakdown, ExplosionResult};
use rust_decimal::Decimal;

/// Combined cache with L1 (memory) and L2 (persistent) tiers
pub struct TieredCache {
    memory: MemoryCache,
    persistent: Option<PersistentCache>,
}

impl TieredCache {
    /// Create a new tiered cache with only memory cache
    pub fn memory_only() -> Self {
        Self {
            memory: MemoryCache::new(),
            persistent: None,
        }
    }

    /// Create a new tiered cache with both memory and persistent cache
    pub fn with_persistent(
        memory_config: CacheConfig,
        persistent_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, PersistentCacheError> {
        Ok(Self {
            memory: MemoryCache::with_config(memory_config),
            persistent: Some(PersistentCache::new(persistent_path)?),
        })
    }

    /// Get cost with L1/L2 cache lookup
    pub fn get_cost(&self, component_id: &ComponentId) -> Option<CostBreakdown> {
        // Try L1 first
        if let Some(cost) = self.memory.get_cost(component_id) {
            return Some(cost);
        }

        // Try L2 if available
        if let Some(persistent) = &self.persistent {
            if let Ok(Some(cost)) = persistent.get_cost(component_id) {
                // Promote to L1
                self.memory.put_cost(component_id.clone(), cost.clone());
                return Some(cost);
            }
        }

        None
    }

    /// Put cost into both L1 and L2 caches
    pub fn put_cost(&self, component_id: ComponentId, cost: CostBreakdown) {
        self.memory.put_cost(component_id.clone(), cost.clone());

        if let Some(persistent) = &self.persistent {
            let _ = persistent.put_cost(&component_id, &cost);
        }
    }

    /// Invalidate cost in both caches
    pub fn invalidate_cost(&self, component_id: &ComponentId) {
        self.memory.invalidate_cost(component_id);

        if let Some(persistent) = &self.persistent {
            let _ = persistent.remove_cost(component_id);
        }
    }

    /// Get explosion with L1/L2 cache lookup
    pub fn get_explosion(
        &self,
        component_id: &ComponentId,
        quantity: &Decimal,
    ) -> Option<ExplosionResult> {
        // Try L1 first
        if let Some(result) = self.memory.get_explosion(component_id, quantity) {
            return Some(result);
        }

        // Try L2 if available
        if let Some(persistent) = &self.persistent {
            if let Ok(Some(result)) = persistent.get_explosion(component_id, quantity) {
                // Promote to L1
                self.memory
                    .put_explosion(component_id.clone(), *quantity, result.clone());
                return Some(result);
            }
        }

        None
    }

    /// Put explosion into both L1 and L2 caches
    pub fn put_explosion(
        &self,
        component_id: ComponentId,
        quantity: Decimal,
        result: ExplosionResult,
    ) {
        self.memory
            .put_explosion(component_id.clone(), quantity, result.clone());

        if let Some(persistent) = &self.persistent {
            let _ = persistent.put_explosion(&component_id, quantity, &result);
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.memory.clear_all();

        if let Some(persistent) = &self.persistent {
            let _ = persistent.clear_all();
        }
    }

    /// Get combined cache statistics
    pub fn stats(&self) -> TieredCacheStats {
        let memory_stats = self.memory.stats();
        let persistent_stats = self
            .persistent
            .as_ref()
            .and_then(|p| p.stats().ok());

        TieredCacheStats {
            memory: memory_stats,
            persistent: persistent_stats,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TieredCacheStats {
    pub memory: CacheStats,
    pub persistent: Option<PersistentCacheStats>,
}
