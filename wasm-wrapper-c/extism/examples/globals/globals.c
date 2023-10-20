#include "../../extism-pdk.h"

#include <stdio.h>

uint64_t count = 0;

int32_t globals() {
  char out[128];
  int n = snprintf(out, 128, "{\"count\": %llu}", count);

  uint64_t offs_ = extism_alloc(n);
  extism_store(offs_, (const uint8_t *)out, n);
  extism_output_set(offs_, n);

  count += 1;

  return 0;
}
