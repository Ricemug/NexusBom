use bom_core::{ComponentId, CostBreakdown, ExplosionResult};
use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache key types
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CacheKey {
    /// Cost calculation cache key
    Cost(ComponentId),

    /// Explosion cache key (component_id, quantity)
    Explosion(ComponentId, String), // quantity as string for hashing

    /// Where-used cache key
    WhereUsed(ComponentId),
}

/// In-memory cache using moka
/// Provides fast access to frequently used BOM calculation results
pub struct MemoryCache {
    /// Cost calculation cache
    cost_cache: Cache<ComponentId, CostBreakdown>,

    /// Explosion result cache
    explosion_cache: Cache<String, ExplosionResult>,

    /// Configuration
    _config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries in cost cache
    pub max_cost_entries: u64,

    /// Maximum number of entries in explosion cache
    pub max_explosion_entries: u64,

    /// Time-to-live for cache entries
    pub ttl: Duration,

    /// Time-to-idle (evict if not accessed)
    pub tti: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_cost_entries: 10_000,
            max_explosion_entries: 5_000,
            ttl: Duration::from_secs(3600), // 1 hour
            tti: Duration::from_secs(1800), // 30 minutes
        }
    }
}

impl MemoryCache {
    /// Create a new memory cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a new memory cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        let cost_cache = Cache::builder()
            .max_capacity(config.max_cost_entries)
            .time_to_live(config.ttl)
            .time_to_idle(config.tti)
            .build();

        let explosion_cache = Cache::builder()
            .max_capacity(config.max_explosion_entries)
            .time_to_live(config.ttl)
            .time_to_idle(config.tti)
            .build();

        Self {
            cost_cache,
            explosion_cache,
            _config: config,
        }
    }

    // Cost cache operations

    /// Get cached cost breakdown
    pub fn get_cost(&self, component_id: &ComponentId) -> Option<CostBreakdown> {
        self.cost_cache.get(component_id)
    }

    /// Put cost breakdown into cache
    pub fn put_cost(&self, component_id: ComponentId, cost: CostBreakdown) {
        self.cost_cache.insert(component_id, cost);
    }

    /// Invalidate cost cache for a component
    pub fn invalidate_cost(&self, component_id: &ComponentId) {
        self.cost_cache.invalidate(component_id);
    }

    // Explosion cache operations

    /// Get cached explosion result
    pub fn get_explosion(
        &self,
        component_id: &ComponentId,
        quantity: &rust_decimal::Decimal,
    ) -> Option<ExplosionResult> {
        let key = Self::make_explosion_key(component_id, quantity);
        self.explosion_cache.get(&key)
    }

    /// Put explosion result into cache
    pub fn put_explosion(
        &self,
        component_id: ComponentId,
        quantity: rust_decimal::Decimal,
        result: ExplosionResult,
    ) {
        let key = Self::make_explosion_key(&component_id, &quantity);
        self.explosion_cache.insert(key, result);
    }

    /// Invalidate explosion cache for a component (all quantities)
    pub fn invalidate_explosion(&self, _component_id: &ComponentId) {
        // Need to invalidate all keys that start with this component
        // Moka doesn't support prefix invalidation, so we need to track keys
        // For now, we'll invalidate the whole cache when a component changes
        // TODO: Implement key tracking for targeted invalidation
        self.explosion_cache.invalidate_all();
    }

    // General operations

    /// Clear all caches
    pub fn clear_all(&self) {
        self.cost_cache.invalidate_all();
        self.explosion_cache.invalidate_all();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        // Note: moka 0.12 doesn't have miss_count(), so we can't calculate exact hit rate
        // We just return the hit count and entry count
        CacheStats {
            cost_entry_count: self.cost_cache.entry_count(),
            cost_hit_rate: 0.0, // Not available in moka 0.12
            explosion_entry_count: self.explosion_cache.entry_count(),
            explosion_hit_rate: 0.0, // Not available in moka 0.12
        }
    }

    /// Run cache maintenance (evict expired entries)
    pub fn run_maintenance(&self) {
        self.cost_cache.run_pending_tasks();
        self.explosion_cache.run_pending_tasks();
    }

    // Helper methods

    fn make_explosion_key(component_id: &ComponentId, quantity: &rust_decimal::Decimal) -> String {
        format!("{}:{}", component_id.as_str(), quantity)
    }
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cost_entry_count: u64,
    pub cost_hit_rate: f64,
    pub explosion_entry_count: u64,
    pub explosion_hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::ComponentId;
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[test]
    fn test_memory_cache_basic() {
        let cache = MemoryCache::new();

        let component_id = ComponentId::new("TEST-001");
        let cost = CostBreakdown {
            component_id: component_id.clone(),
            material_cost: Decimal::from(100),
            labor_cost: Decimal::ZERO,
            overhead_cost: Decimal::ZERO,
            subcontract_cost: Decimal::ZERO,
            total_cost: Decimal::from(100),
            calculated_at: Utc::now(),
        };

        // Initially empty
        assert!(cache.get_cost(&component_id).is_none());

        // Put and get
        cache.put_cost(component_id.clone(), cost.clone());
        let cached = cache.get_cost(&component_id).unwrap();
        assert_eq!(cached.total_cost, Decimal::from(100));

        // Invalidate
        cache.invalidate_cost(&component_id);
        assert!(cache.get_cost(&component_id).is_none());
    }

    #[test]
    fn test_explosion_cache() {
        let cache = MemoryCache::new();

        let component_id = ComponentId::new("TEST-002");
        let quantity = Decimal::from(10);
        let result = ExplosionResult {
            root_component: component_id.clone(),
            items: vec![],
            unique_component_count: 0,
            max_depth: 0,
            calculated_at: Utc::now(),
        };

        // Initially empty
        assert!(cache.get_explosion(&component_id, &quantity).is_none());

        // Put and get
        cache.put_explosion(component_id.clone(), quantity, result.clone());
        let cached = cache.get_explosion(&component_id, &quantity).unwrap();
        assert_eq!(cached.root_component, component_id);

        // Different quantity should miss
        assert!(cache.get_explosion(&component_id, &Decimal::from(20)).is_none());
    }

    #[test]
    fn test_cache_stats() {
        let cache = MemoryCache::new();
        let component_id = ComponentId::new("TEST-003");
        let cost = CostBreakdown {
            component_id: component_id.clone(),
            material_cost: Decimal::from(100),
            labor_cost: Decimal::ZERO,
            overhead_cost: Decimal::ZERO,
            subcontract_cost: Decimal::ZERO,
            total_cost: Decimal::from(100),
            calculated_at: Utc::now(),
        };

        cache.put_cost(component_id.clone(), cost);

        // Run pending tasks to ensure cache is updated
        cache.run_maintenance();

        // Cache hit
        cache.get_cost(&component_id);

        // Cache miss
        cache.get_cost(&ComponentId::new("NOT-EXIST"));

        // Run pending tasks to ensure stats are updated
        cache.run_maintenance();

        let stats = cache.stats();
        assert_eq!(stats.cost_entry_count, 1);
        // Note: moka 0.12 doesn't support hit_rate calculation
        assert_eq!(stats.cost_hit_rate, 0.0);
    }

    #[test]
    fn test_clear_all() {
        let cache = MemoryCache::new();
        let component_id = ComponentId::new("TEST-004");
        let cost = CostBreakdown {
            component_id: component_id.clone(),
            material_cost: Decimal::from(100),
            labor_cost: Decimal::ZERO,
            overhead_cost: Decimal::ZERO,
            subcontract_cost: Decimal::ZERO,
            total_cost: Decimal::from(100),
            calculated_at: Utc::now(),
        };

        cache.put_cost(component_id.clone(), cost);
        assert!(cache.get_cost(&component_id).is_some());

        cache.clear_all();
        assert!(cache.get_cost(&component_id).is_none());
    }
}
