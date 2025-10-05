use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Component basic information
/// Compatible with SAP MARA/MARC and Oracle MTL_SYSTEM_ITEMS_B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component ID (Material Number in SAP, Item ID in Oracle)
    pub id: ComponentId,

    /// Component description
    pub description: String,

    /// Component type (FERT/HALB/ROH in SAP)
    pub component_type: ComponentType,

    /// Unit of measure
    pub uom: String,

    /// Standard cost (移動平均價或標準價)
    pub standard_cost: Option<Decimal>,

    /// Lead time in days
    pub lead_time_days: Option<u32>,

    /// Procurement type (Make/Buy)
    pub procurement_type: ProcurementType,

    /// Organization/Plant (SAP WERKS, Oracle Organization_id)
    pub organization: String,

    /// Version for optimistic locking
    pub version: u64,

    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    /// 成品 (Finished Product - FERT in SAP)
    FinishedProduct,

    /// 半成品 (Semi-finished - HALB in SAP)
    SemiFinished,

    /// 原物料 (Raw Material - ROH in SAP)
    RawMaterial,

    /// 包裝材料 (Packaging Material)
    Packaging,

    /// 服務 (Service)
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcurementType {
    /// 自製 (Make/Produce)
    Make,

    /// 採購 (Buy/Purchase)
    Buy,

    /// 兩者皆可 (Both)
    Both,
}

/// BOM Item - represents a parent-child relationship
/// Compatible with SAP STPO and Oracle BOM_COMPONENTS_B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomItem {
    /// Unique ID for this BOM item
    pub id: Uuid,

    /// Parent component ID
    pub parent_id: ComponentId,

    /// Child component ID
    pub child_id: ComponentId,

    /// Quantity required per parent (基礎用量)
    pub quantity: Decimal,

    /// Scrap/waste factor (損耗率) - 0.05 means 5% waste
    pub scrap_factor: Decimal,

    /// Item sequence number (項次)
    pub sequence: u32,

    /// Operation sequence (工序號碼 - for routing integration)
    pub operation_sequence: Option<String>,

    /// Is this a phantom/pseudo component (虛設件)
    /// Phantom components are not procured/manufactured separately
    pub is_phantom: bool,

    /// Effectivity date range (生效日期)
    pub effective_from: Option<DateTime<Utc>>,
    pub effective_to: Option<DateTime<Utc>>,

    /// Alternative group (替代料組)
    /// Items with same alternative_group can substitute each other
    pub alternative_group: Option<String>,

    /// Priority within alternative group (1 is highest priority)
    pub alternative_priority: Option<u32>,

    /// Reference designator (for electronics - e.g., "R1, R2, R3")
    pub reference_designator: Option<String>,

    /// Component position (for assembly drawings)
    pub position: Option<String>,

    /// Notes/remarks
    pub notes: Option<String>,

    /// Version for optimistic locking
    pub version: u64,
}

impl BomItem {
    /// Calculate effective quantity including scrap
    pub fn effective_quantity(&self) -> Decimal {
        self.quantity * (Decimal::ONE + self.scrap_factor)
    }

    /// Check if this item is effective at given date
    pub fn is_effective_at(&self, date: &DateTime<Utc>) -> bool {
        let after_start = self.effective_from.as_ref().map_or(true, |from| date >= from);
        let before_end = self.effective_to.as_ref().map_or(true, |to| date <= to);
        after_start && before_end
    }
}

/// BOM Header - represents a complete BOM for a component
/// Compatible with SAP STKO/MAST and Oracle BOM_STRUCTURES_B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomHeader {
    /// BOM ID (Alternative BOM in SAP)
    pub id: String,

    /// Component this BOM is for
    pub component_id: ComponentId,

    /// BOM usage (1=Production, 2=Engineering, etc. - SAP STLAN)
    pub usage: BomUsage,

    /// BOM status
    pub status: BomStatus,

    /// Base quantity (基礎數量 - usually 1)
    pub base_quantity: Decimal,

    /// Alternative BOM indicator (for variant BOMs)
    pub alternative: Option<String>,

    /// Effectivity date range
    pub effective_from: Option<DateTime<Utc>>,
    pub effective_to: Option<DateTime<Utc>>,

    /// Organization/Plant
    pub organization: String,

    /// Version for optimistic locking
    pub version: u64,

    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BomUsage {
    /// 生產用 (Production)
    Production,

    /// 工程用 (Engineering)
    Engineering,

    /// 成本用 (Costing)
    Costing,

    /// 維修用 (Maintenance)
    Maintenance,

    /// 銷售用 (Sales)
    Sales,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BomStatus {
    /// 草稿 (Draft)
    Draft,

    /// 已發布 (Released/Active)
    Released,

    /// 凍結 (Frozen)
    Frozen,

    /// 已淘汰 (Obsolete)
    Obsolete,
}

/// Cost breakdown for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    /// Component ID
    pub component_id: ComponentId,

    /// Material cost (直接材料成本)
    pub material_cost: Decimal,

    /// Labor cost (直接人工成本)
    pub labor_cost: Decimal,

    /// Overhead cost (製造費用)
    pub overhead_cost: Decimal,

    /// Subcontracting cost (外包成本)
    pub subcontract_cost: Decimal,

    /// Total cost
    pub total_cost: Decimal,

    /// Calculation timestamp
    pub calculated_at: DateTime<Utc>,
}

impl CostBreakdown {
    pub fn sum(&self) -> Decimal {
        self.material_cost + self.labor_cost + self.overhead_cost + self.subcontract_cost
    }
}

/// Material explosion result (物料展開結果)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionResult {
    /// Root component that was exploded
    pub root_component: ComponentId,

    /// Flattened list of all required components with quantities
    pub items: Vec<ExplosionItem>,

    /// Total unique components
    pub unique_component_count: usize,

    /// Maximum BOM depth
    pub max_depth: usize,

    /// Calculation timestamp
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionItem {
    /// Component ID
    pub component_id: ComponentId,

    /// Total quantity required (sum of all paths)
    pub total_quantity: Decimal,

    /// BOM level/depth (0 = root, 1 = direct child, etc.)
    pub level: usize,

    /// All paths from root to this component
    pub paths: Vec<Vec<ComponentId>>,

    /// Is this a phantom component
    pub is_phantom: bool,
}

/// Where-used query result (反查結果)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereUsedResult {
    /// Component that was queried
    pub component: ComponentId,

    /// Parent assemblies that use this component
    pub used_in: Vec<WhereUsedItem>,

    /// Query timestamp
    pub queried_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereUsedItem {
    /// Parent component ID
    pub parent_id: ComponentId,

    /// Quantity per parent
    pub quantity: Decimal,

    /// BOM level (how many levels up from the queried component)
    pub level: usize,

    /// All paths from this parent to the queried component
    pub paths: Vec<Vec<ComponentId>>,
}
