/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace bom {
#endif // __cplusplus

/**
 * FFI result code
 */
typedef enum BomResultCode {
  Success = 0,
  ErrorNullPointer = 1,
  ErrorInvalidUtf8 = 2,
  ErrorJsonParse = 3,
  ErrorJsonSerialize = 4,
  ErrorCalculation = 5,
  ErrorNotFound = 6,
} BomResultCode;

/**
 * In-memory repository for FFI
 */
typedef struct InMemoryRepo InMemoryRepo;

/**
 * Opaque handle to BOM engine
 */
typedef struct BomEngine {
  struct InMemoryRepo repo;
} BomEngine;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new BOM engine instance
 * Returns NULL on failure
 */
 struct BomEngine *bom_engine_new(void) ;

/**
 * Free a BOM engine instance
 */
 void bom_engine_free(struct BomEngine *engine) ;

/**
 * Add a component to the BOM graph
 * component_json: JSON string representing a Component
 * Returns BomResultCode
 */
 enum BomResultCode bom_add_component(struct BomEngine *engine, const char *component_json) ;

/**
 * Add a BOM item (parent-child relationship) to the graph
 * bom_item_json: JSON string representing a BomItem
 * Returns BomResultCode
 */
 enum BomResultCode bom_add_item(struct BomEngine *engine, const char *bom_item_json) ;

/**
 * Calculate material explosion for a component
 * component_id: Component ID string
 * quantity: Quantity as string (e.g., "10.5")
 * result_json: Output buffer for JSON result (caller must free with bom_free_string)
 * Returns BomResultCode
 */
 enum BomResultCode bom_calculate_explosion(struct BomEngine *engine, const char *component_id, const char *quantity, char **result_json) ;

/**
 * Calculate cost breakdown for a component
 * component_id: Component ID string
 * result_json: Output buffer for JSON result (caller must free with bom_free_string)
 * Returns BomResultCode
 */
 enum BomResultCode bom_calculate_cost(struct BomEngine *engine, const char *component_id, char **result_json) ;

/**
 * Find where a component is used (reverse BOM lookup)
 * component_id: Component ID string
 * result_json: Output buffer for JSON array of parent component IDs (caller must free)
 * Returns BomResultCode
 */
 enum BomResultCode bom_where_used(struct BomEngine *engine, const char *component_id, char **result_json) ;

/**
 * Free a string returned by BOM functions
 */
 void bom_free_string(char *s) ;

/**
 * Get error message for a result code
 * Returns a static string (do not free)
 */
 const char *bom_error_message(enum BomResultCode code) ;

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#ifdef __cplusplus
} // namespace bom
#endif // __cplusplus
