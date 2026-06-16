#include "postgres.h"
#include "pg_query.h"
#include "src/pg_query_internal.h"
#include "nodes/parsenodes.h"
#include "utils/palloc.h"
#include "utils/memutils.h"

static inline MemoryContext get_top_memory_context(void) {
  return TopMemoryContext;
}
