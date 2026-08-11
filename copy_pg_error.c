#include "copy_pg_error.h"
#include "nodes/nodeFuncs.h"

Error copy_pg_error() {
  MemoryContext mem = AllocSetContextCreate(
      TopMemoryContext,
      "pg_raw_parse_error",
      ALLOCSET_DEFAULT_SIZES
  );

  MemoryContext prev = MemoryContextSwitchTo(mem);
  ErrorData *error_data = CopyErrorData();
  FlushErrorState();
  MemoryContextSwitchTo(prev);

  Error error = {
    .mem = mem,
    .error_data = error_data
  };
  return error;
}

bool wrapped_raw_expression_tree_walker_impl(Node *n, tree_walker_callback w, void *c, Error *error) {
  bool result;
  PG_TRY();
    result = raw_expression_tree_walker_impl(n, w, c);
  PG_CATCH();
    *error = copy_pg_error();
    result = true;
  PG_END_TRY();
  return result;
}
