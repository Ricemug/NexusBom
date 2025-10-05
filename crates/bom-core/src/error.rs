use thiserror::Error;

#[derive(Error, Debug)]
pub enum BomError {
    #[error("Circular dependency detected in BOM: {0}")]
    CircularDependency(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("BOM structure not found: {0}")]
    BomNotFound(String),

    #[error("Invalid quantity: {0}")]
    InvalidQuantity(String),

    #[error("Invalid effectivity date range: {from} to {to}")]
    InvalidEffectivityRange {
        from: String,
        to: String,
    },

    #[error("Phantom component cannot have cost: {0}")]
    PhantomWithCost(String),

    #[error("Alternative group not found: {0}")]
    AlternativeGroupNotFound(String),

    #[error("Version conflict: expected {expected}, found {found}")]
    VersionConflict {
        expected: u64,
        found: u64,
    },

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

pub type Result<T> = std::result::Result<T, BomError>;
