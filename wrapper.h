#include "postgres.h"
#include "pg_query.h"
#include "src/pg_query_internal.h"
#include "nodes/parsenodes.h"
#include "nodes/nodeFuncs.h"
#include "nodes/makefuncs.h"
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

static inline StringInfo wrapped_raw_deparse(RawStmt *stmt, ErrorData **error) {
  StringInfo str;
  PG_TRY();
    str = makeStringInfo();
    deparseRawStmt(str, stmt);
  PG_CATCH();
    *error = CopyErrorData();
    FlushErrorState();
  PG_END_TRY();
  return str;
}

// FIXME(sage): libpg_query doesn't compile pnstrdup, which we want
static inline
char *
wrapped_pnstrdup(const char *in, Size len)
{
  char *out;

  len = strnlen(in, len);

  out = palloc(len + 1);
  memcpy(out, in, len);
  out[len] = '\0';

  return out;
}

static inline Node *wrapped_copy_object(Node *node, ErrorData **error) {
  Node *result = NULL;
  PG_TRY();
    result = copyObject(node);
  PG_CATCH();
    *error = CopyErrorData();
    FlushErrorState();
  PG_END_TRY();
  return result;
}
