#include "copy_pg_error.h"

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
