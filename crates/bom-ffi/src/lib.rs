use bom_calc::costing::CostCalculator;
use bom_calc::explosion::ExplosionCalculator;
use bom_calc::where_used::WhereUsedAnalyzer;
use bom_core::{BomError, BomHeader, BomItem, BomRepository, Component, ComponentId};
use bom_graph::BomGraph;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// In-memory repository for FFI
struct InMemoryRepo {
    components: HashMap<ComponentId, Component>,
    bom_items: Vec<BomItem>,
}

impl InMemoryRepo {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
            bom_items: Vec::new(),
        }
    }
}

impl BomRepository for InMemoryRepo {
    fn get_component(&self, id: &ComponentId) -> Result<Component, BomError> {
        self.components
            .get(id)
            .cloned()
            .ok_or_else(|| BomError::ComponentNotFound(id.as_str().to_string()))
    }

    fn get_components(&self, ids: &[ComponentId]) -> Result<Vec<Component>, BomError> {
        ids.iter()
            .map(|id| self.get_component(id))
            .collect()
    }

    fn get_bom_header(
        &self,
        component_id: &ComponentId,
        alternative: Option<&str>,
        _effective_date: Option<DateTime<Utc>>,
    ) -> Result<BomHeader, BomError> {
        // Simple implementation - return basic header
        Ok(BomHeader {
            id: format!("BOM-{}", component_id.as_str()),
            component_id: component_id.clone(),
            usage: bom_core::BomUsage::Production,
            status: bom_core::BomStatus::Released,
            base_quantity: Decimal::ONE,
            alternative: alternative.map(String::from),
            effective_from: None,
            effective_to: None,
            organization: "DEFAULT".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
        })
    }

    fn get_bom_items(
        &self,
        parent_id: &ComponentId,
        _effective_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<BomItem>, BomError> {
        Ok(self
            .bom_items
            .iter()
            .filter(|item| &item.parent_id == parent_id)
            .cloned()
            .collect())
    }

    fn get_all_bom_items(&self) -> Result<Vec<BomItem>, BomError> {
        Ok(self.bom_items.clone())
    }

    fn find_parents(&self, component_id: &ComponentId) -> Result<Vec<BomItem>, BomError> {
        Ok(self
            .bom_items
            .iter()
            .filter(|item| &item.child_id == component_id)
            .cloned()
            .collect())
    }
}

/// Opaque handle to BOM engine
#[repr(C)]
pub struct BomEngine {
    repo: InMemoryRepo,
}

/// FFI result code
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BomResultCode {
    Success = 0,
    ErrorNullPointer = 1,
    ErrorInvalidUtf8 = 2,
    ErrorJsonParse = 3,
    ErrorJsonSerialize = 4,
    ErrorCalculation = 5,
    ErrorNotFound = 6,
}

/// Create a new BOM engine instance
/// Returns NULL on failure
#[no_mangle]
pub extern "C" fn bom_engine_new() -> *mut BomEngine {
    Box::into_raw(Box::new(BomEngine {
        repo: InMemoryRepo::new(),
    }))
}

/// Free a BOM engine instance
#[no_mangle]
pub extern "C" fn bom_engine_free(engine: *mut BomEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}

/// Add a component to the BOM graph
/// component_json: JSON string representing a Component
/// Returns BomResultCode
#[no_mangle]
pub extern "C" fn bom_add_component(
    engine: *mut BomEngine,
    component_json: *const c_char,
) -> BomResultCode {
    if engine.is_null() || component_json.is_null() {
        return BomResultCode::ErrorNullPointer;
    }

    let component_str = unsafe {
        match CStr::from_ptr(component_json).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let component: Component = match serde_json::from_str(component_str) {
        Ok(c) => c,
        Err(_) => return BomResultCode::ErrorJsonParse,
    };

    let engine = unsafe { &mut *engine };
    engine.repo.components.insert(component.id.clone(), component);

    BomResultCode::Success
}

/// Add a BOM item (parent-child relationship) to the graph
/// bom_item_json: JSON string representing a BomItem
/// Returns BomResultCode
#[no_mangle]
pub extern "C" fn bom_add_item(
    engine: *mut BomEngine,
    bom_item_json: *const c_char,
) -> BomResultCode {
    if engine.is_null() || bom_item_json.is_null() {
        return BomResultCode::ErrorNullPointer;
    }

    let item_str = unsafe {
        match CStr::from_ptr(bom_item_json).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let bom_item: BomItem = match serde_json::from_str(item_str) {
        Ok(i) => i,
        Err(_) => return BomResultCode::ErrorJsonParse,
    };

    let engine = unsafe { &mut *engine };
    engine.repo.bom_items.push(bom_item);

    BomResultCode::Success
}

/// Calculate material explosion for a component
/// component_id: Component ID string
/// quantity: Quantity as string (e.g., "10.5")
/// result_json: Output buffer for JSON result (caller must free with bom_free_string)
/// Returns BomResultCode
#[no_mangle]
pub extern "C" fn bom_calculate_explosion(
    engine: *mut BomEngine,
    component_id: *const c_char,
    quantity: *const c_char,
    result_json: *mut *mut c_char,
) -> BomResultCode {
    if engine.is_null() || component_id.is_null() || quantity.is_null() || result_json.is_null() {
        return BomResultCode::ErrorNullPointer;
    }

    let id_str = unsafe {
        match CStr::from_ptr(component_id).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let qty_str = unsafe {
        match CStr::from_ptr(quantity).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let qty: Decimal = match qty_str.parse() {
        Ok(q) => q,
        Err(_) => return BomResultCode::ErrorJsonParse,
    };

    let engine = unsafe { &*engine };
    let comp_id = ComponentId::new(id_str);

    // Build graph and calculate
    let graph = match BomGraph::from_component(&engine.repo, &comp_id, None) {
        Ok(g) => g,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let calculator = ExplosionCalculator::new(&graph);
    let explosion_result = match calculator.explode(&comp_id, qty) {
        Ok(r) => r,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let json_str = match serde_json::to_string(&explosion_result) {
        Ok(s) => s,
        Err(_) => return BomResultCode::ErrorJsonSerialize,
    };

    match CString::new(json_str) {
        Ok(c_str) => {
            unsafe {
                *result_json = c_str.into_raw();
            }
            BomResultCode::Success
        }
        Err(_) => BomResultCode::ErrorInvalidUtf8,
    }
}

/// Calculate cost breakdown for a component
/// component_id: Component ID string
/// result_json: Output buffer for JSON result (caller must free with bom_free_string)
/// Returns BomResultCode
#[no_mangle]
pub extern "C" fn bom_calculate_cost(
    engine: *mut BomEngine,
    component_id: *const c_char,
    result_json: *mut *mut c_char,
) -> BomResultCode {
    if engine.is_null() || component_id.is_null() || result_json.is_null() {
        return BomResultCode::ErrorNullPointer;
    }

    let id_str = unsafe {
        match CStr::from_ptr(component_id).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let engine = unsafe { &*engine };
    let comp_id = ComponentId::new(id_str);

    // Build graph and calculate
    let graph = match BomGraph::from_component(&engine.repo, &comp_id, None) {
        Ok(g) => g,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let calculator = CostCalculator::new(&graph, &engine.repo);
    let cost_breakdown = match calculator.calculate_cost(&comp_id) {
        Ok(c) => c,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let json_str = match serde_json::to_string(&cost_breakdown) {
        Ok(s) => s,
        Err(_) => return BomResultCode::ErrorJsonSerialize,
    };

    match CString::new(json_str) {
        Ok(c_str) => {
            unsafe {
                *result_json = c_str.into_raw();
            }
            BomResultCode::Success
        }
        Err(_) => BomResultCode::ErrorInvalidUtf8,
    }
}

/// Find where a component is used (reverse BOM lookup)
/// component_id: Component ID string
/// result_json: Output buffer for JSON array of parent component IDs (caller must free)
/// Returns BomResultCode
#[no_mangle]
pub extern "C" fn bom_where_used(
    engine: *mut BomEngine,
    component_id: *const c_char,
    result_json: *mut *mut c_char,
) -> BomResultCode {
    if engine.is_null() || component_id.is_null() || result_json.is_null() {
        return BomResultCode::ErrorNullPointer;
    }

    let id_str = unsafe {
        match CStr::from_ptr(component_id).to_str() {
            Ok(s) => s,
            Err(_) => return BomResultCode::ErrorInvalidUtf8,
        }
    };

    let engine = unsafe { &*engine };
    let comp_id = ComponentId::new(id_str);

    // Build graph for the component
    let graph = match BomGraph::from_component(&engine.repo, &comp_id, None) {
        Ok(g) => g,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let analyzer = WhereUsedAnalyzer::new(&graph);
    let where_used_result = match analyzer.analyze(&comp_id) {
        Ok(r) => r,
        Err(_) => return BomResultCode::ErrorCalculation,
    };

    let parent_ids: Vec<String> = where_used_result
        .used_in
        .iter()
        .map(|item| item.parent_id.as_str().to_string())
        .collect();

    let json_str = match serde_json::to_string(&parent_ids) {
        Ok(s) => s,
        Err(_) => return BomResultCode::ErrorJsonSerialize,
    };

    match CString::new(json_str) {
        Ok(c_str) => {
            unsafe {
                *result_json = c_str.into_raw();
            }
            BomResultCode::Success
        }
        Err(_) => BomResultCode::ErrorInvalidUtf8,
    }
}

/// Free a string returned by BOM functions
#[no_mangle]
pub extern "C" fn bom_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

/// Get error message for a result code
/// Returns a static string (do not free)
#[no_mangle]
pub extern "C" fn bom_error_message(code: BomResultCode) -> *const c_char {
    let msg = match code {
        BomResultCode::Success => "Success\0",
        BomResultCode::ErrorNullPointer => "Null pointer error\0",
        BomResultCode::ErrorInvalidUtf8 => "Invalid UTF-8 encoding\0",
        BomResultCode::ErrorJsonParse => "JSON parse error\0",
        BomResultCode::ErrorJsonSerialize => "JSON serialization error\0",
        BomResultCode::ErrorCalculation => "Calculation error\0",
        BomResultCode::ErrorNotFound => "Component not found\0",
    };
    msg.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_ffi_basic_workflow() {
        // Create engine
        let engine = bom_engine_new();
        assert!(!engine.is_null());

        // Add components
        let frame_json = CString::new(r#"{
            "id": "FRAME-001",
            "description": "Main frame",
            "component_type": "FinishedProduct",
            "uom": "EA",
            "standard_cost": "150.0",
            "lead_time_days": 7,
            "procurement_type": "Make",
            "organization": "ORG01",
            "version": 1,
            "created_at": "2025-10-05T10:00:00Z",
            "updated_at": "2025-10-05T10:00:00Z"
        }"#)
        .unwrap();

        let result = bom_add_component(engine, frame_json.as_ptr());
        assert_eq!(result, BomResultCode::Success);

        let wheel_json = CString::new(r#"{
            "id": "WHEEL-001",
            "description": "Standard wheel",
            "component_type": "RawMaterial",
            "uom": "EA",
            "standard_cost": "50.0",
            "lead_time_days": 3,
            "procurement_type": "Buy",
            "organization": "ORG01",
            "version": 1,
            "created_at": "2025-10-05T10:00:00Z",
            "updated_at": "2025-10-05T10:00:00Z"
        }"#)
        .unwrap();

        let result = bom_add_component(engine, wheel_json.as_ptr());
        assert_eq!(result, BomResultCode::Success);

        // Add BOM item
        let bom_item_json = CString::new(r#"{
            "id": "a7a7a7a7-a7a7-a7a7-a7a7-a7a7a7a7a7a7",
            "parent_id": "FRAME-001",
            "child_id": "WHEEL-001",
            "quantity": "2.0",
            "scrap_factor": "0.0",
            "sequence": 10,
            "is_phantom": false,
            "version": 1,
            "reference_designator": "WH1,WH2"
        }"#)
        .unwrap();

        let result = bom_add_item(engine, bom_item_json.as_ptr());
        assert_eq!(result, BomResultCode::Success);

        // Calculate explosion
        let comp_id = CString::new("FRAME-001").unwrap();
        let quantity = CString::new("1.0").unwrap();
        let mut result_json: *mut c_char = ptr::null_mut();

        let result = bom_calculate_explosion(
            engine,
            comp_id.as_ptr(),
            quantity.as_ptr(),
            &mut result_json,
        );
        assert_eq!(result, BomResultCode::Success);
        assert!(!result_json.is_null());

        // Free result
        bom_free_string(result_json);

        // Calculate cost
        let mut cost_json: *mut c_char = ptr::null_mut();
        let result = bom_calculate_cost(engine, comp_id.as_ptr(), &mut cost_json);
        assert_eq!(result, BomResultCode::Success);
        assert!(!cost_json.is_null());

        // Free result
        bom_free_string(cost_json);

        // Where used
        let wheel_id = CString::new("WHEEL-001").unwrap();
        let mut where_used_json: *mut c_char = ptr::null_mut();
        let result = bom_where_used(engine, wheel_id.as_ptr(), &mut where_used_json);
        assert_eq!(result, BomResultCode::Success);
        assert!(!where_used_json.is_null());

        // Free result
        bom_free_string(where_used_json);

        // Free engine
        bom_engine_free(engine);
    }

    #[test]
    fn test_ffi_null_handling() {
        let result = bom_add_component(ptr::null_mut(), ptr::null());
        assert_eq!(result, BomResultCode::ErrorNullPointer);

        let engine = bom_engine_new();
        let result = bom_add_component(engine, ptr::null());
        assert_eq!(result, BomResultCode::ErrorNullPointer);

        bom_engine_free(engine);
    }
}
