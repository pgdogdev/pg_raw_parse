#include "postgres.h"
#include "pg_query.h"
#include "src/pg_query_internal.h"
#include "nodes/parsenodes.h"
#include "nodes/nodeFuncs.h"
#include "utils/palloc.h"
#include "utils/memutils.h"
#include "copy_pg_error.h"

static inline MemoryContext get_top_memory_context(void) {
  return TopMemoryContext;
}

static inline bool wrapped_raw_expression_tree_walker_impl(Node *n, tree_walker_callback w, void *c, Error *error) {
  bool result;
  PG_TRY();
    result = raw_expression_tree_walker_impl(n, w, c);
  PG_CATCH();
    *error = copy_pg_error();
    result = true;
  PG_END_TRY();
  return result;
}
