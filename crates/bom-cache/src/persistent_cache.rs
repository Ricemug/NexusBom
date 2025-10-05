use bom_core::{ComponentId, CostBreakdown, ExplosionResult};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use std::path::Path;

/// Persistent cache using redb
/// Survives application restarts
pub struct PersistentCache {
    db: Database,
}

// Define table schemas
const COST_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("cost_cache");
const EXPLOSION_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("explosion_cache");

impl PersistentCache {
    /// Create or open a persistent cache at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PersistentCacheError> {
        let db = Database::create(path)?;

        // Initialize tables
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(COST_TABLE)?;
            let _ = write_txn.open_table(EXPLOSION_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db })
    }

    /// Create an in-memory persistent cache (for testing)
    pub fn in_memory() -> Result<Self, PersistentCacheError> {
        let db = Database::builder().create_with_backend(redb::backends::InMemoryBackend::new())?;

        // Initialize tables
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(COST_TABLE)?;
            let _ = write_txn.open_table(EXPLOSION_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db })
    }

    // Cost cache operations

    /// Get cached cost breakdown
    pub fn get_cost(&self, component_id: &ComponentId) -> Result<Option<CostBreakdown>, PersistentCacheError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(COST_TABLE)?;

        match table.get(component_id.as_str())? {
            Some(value) => {
                let bytes = value.value();
                let cost: CostBreakdown = rmp_serde::from_slice(bytes)?;
                Ok(Some(cost))
            }
            None => Ok(None),
        }
    }

    /// Put cost breakdown into cache
    pub fn put_cost(
        &self,
        component_id: &ComponentId,
        cost: &CostBreakdown,
    ) -> Result<(), PersistentCacheError> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(COST_TABLE)?;
            let bytes = rmp_serde::to_vec(cost)?;
            table.insert(component_id.as_str(), bytes.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// Remove cost from cache
    pub fn remove_cost(&self, component_id: &ComponentId) -> Result<(), PersistentCacheError> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(COST_TABLE)?;
            table.remove(component_id.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    // Explosion cache operations

    /// Get cached explosion result
    pub fn get_explosion(
        &self,
        component_id: &ComponentId,
        quantity: &rust_decimal::Decimal,
    ) -> Result<Option<ExplosionResult>, PersistentCacheError> {
        let key = Self::make_explosion_key(component_id, quantity);
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(EXPLOSION_TABLE)?;

        match table.get(key.as_str())? {
            Some(value) => {
                let bytes = value.value();
                let result: ExplosionResult = rmp_serde::from_slice(bytes)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    /// Put explosion result into cache
    pub fn put_explosion(
        &self,
        component_id: &ComponentId,
        quantity: rust_decimal::Decimal,
        result: &ExplosionResult,
    ) -> Result<(), PersistentCacheError> {
        let key = Self::make_explosion_key(component_id, &quantity);
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(EXPLOSION_TABLE)?;
            let bytes = rmp_serde::to_vec(result)?;
            table.insert(key.as_str(), bytes.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    // General operations

    /// Clear all caches
    pub fn clear_all(&self) -> Result<(), PersistentCacheError> {
        let write_txn = self.db.begin_write()?;
        {
            let mut cost_table = write_txn.open_table(COST_TABLE)?;
            let mut explosion_table = write_txn.open_table(EXPLOSION_TABLE)?;

            // Clear all entries
            let cost_keys: Vec<String> = cost_table
                .iter()?
                .filter_map(|item| item.ok())
                .map(|(key, _)| key.value().to_string())
                .collect();

            for key in cost_keys {
                cost_table.remove(key.as_str())?;
            }

            let explosion_keys: Vec<String> = explosion_table
                .iter()?
                .filter_map(|item| item.ok())
                .map(|(key, _)| key.value().to_string())
                .collect();

            for key in explosion_keys {
                explosion_table.remove(key.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<PersistentCacheStats, PersistentCacheError> {
        let read_txn = self.db.begin_read()?;
        let cost_table = read_txn.open_table(COST_TABLE)?;
        let explosion_table = read_txn.open_table(EXPLOSION_TABLE)?;

        Ok(PersistentCacheStats {
            cost_entry_count: cost_table.len()?,
            explosion_entry_count: explosion_table.len()?,
        })
    }

    // Compact the database to reclaim space
    pub fn compact(&mut self) -> Result<(), PersistentCacheError> {
        self.db.compact()?;
        Ok(())
    }

    // Helper methods

    fn make_explosion_key(component_id: &ComponentId, quantity: &rust_decimal::Decimal) -> String {
        format!("{}:{}", component_id.as_str(), quantity)
    }
}

#[derive(Debug, Clone)]
pub struct PersistentCacheStats {
    pub cost_entry_count: u64,
    pub explosion_entry_count: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum PersistentCacheError {
    #[error("Database error: {0}")]
    Database(#[from] redb::DatabaseError),

    #[error("Table error: {0}")]
    Table(#[from] redb::TableError),

    #[error("Transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),

    #[error("Commit error: {0}")]
    Commit(#[from] redb::CommitError),

    #[error("Storage error: {0}")]
    Storage(#[from] redb::StorageError),

    #[error("Compaction error: {0}")]
    Compaction(#[from] redb::CompactionError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use bom_core::ComponentId;
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[test]
    fn test_persistent_cache_basic() {
        let cache = PersistentCache::in_memory().unwrap();

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
        assert!(cache.get_cost(&component_id).unwrap().is_none());

        // Put and get
        cache.put_cost(&component_id, &cost).unwrap();
        let cached = cache.get_cost(&component_id).unwrap().unwrap();
        assert_eq!(cached.total_cost, Decimal::from(100));

        // Remove
        cache.remove_cost(&component_id).unwrap();
        assert!(cache.get_cost(&component_id).unwrap().is_none());
    }

    #[test]
    fn test_explosion_cache() {
        let cache = PersistentCache::in_memory().unwrap();

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
        assert!(cache.get_explosion(&component_id, &quantity).unwrap().is_none());

        // Put and get
        cache.put_explosion(&component_id, quantity, &result).unwrap();
        let cached = cache.get_explosion(&component_id, &quantity).unwrap().unwrap();
        assert_eq!(cached.root_component, component_id);

        // Different quantity should miss
        assert!(cache
            .get_explosion(&component_id, &Decimal::from(20))
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_cache_stats() {
        let cache = PersistentCache::in_memory().unwrap();
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

        cache.put_cost(&component_id, &cost).unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.cost_entry_count, 1);
    }

    #[test]
    fn test_clear_all() {
        let cache = PersistentCache::in_memory().unwrap();
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

        cache.put_cost(&component_id, &cost).unwrap();
        assert!(cache.get_cost(&component_id).unwrap().is_some());

        cache.clear_all().unwrap();
        assert!(cache.get_cost(&component_id).unwrap().is_none());
    }
}
