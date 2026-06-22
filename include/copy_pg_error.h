#ifndef COPY_PG_ERROR_H
#define COPY_PG_ERROR_H

#include "postgres.h"
#include "utils/memutils.h"
#include "utils/elog.h"

typedef struct Error {
  MemoryContext mem;
  ErrorData *error_data;
} Error;

/*
 * Creates a new memory context and copies the current PG error into it.
 * Returns both after flushing the error state.
 */
Error copy_pg_error();

#endif
