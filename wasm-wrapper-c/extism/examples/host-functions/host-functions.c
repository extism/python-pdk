#include "../../extism-pdk.h"

#include <stdio.h>

extern uint64_t hello_world(uint64_t);

int32_t count_vowels() {
  uint64_t length = extism_input_length();

  if (length == 0) {
    return 0;
  }

  int64_t count = 0;
  char ch = 0;
  for (int64_t i = 0; i < length; i++) {
    ch = extism_input_load_u8(i);
    if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
        ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U') {
      count += 1;
    }
  }

  char out[128];
  int n = snprintf(out, 128, "{\"count\": %lld}", count);
  uint64_t offs_ = extism_alloc(n);
  extism_store(offs_, (const uint8_t *)out, n);
  offs_ = hello_world(offs_);
  extism_output_set(offs_, extism_length(offs_));
  return 0;
}
