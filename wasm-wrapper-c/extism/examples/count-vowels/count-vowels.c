#include "../../extism-pdk.h"

#include <stdio.h>

int32_t count_vowels() {
  uint64_t count = 0;
  uint8_t ch = 0;
  uint64_t length = extism_input_length();

  for (uint64_t i = 0; i < length; i++) {
    ch = extism_input_load_u8(i);
    count += (ch == 'A') + (ch == 'a') + (ch == 'E') + (ch == 'e') +
             (ch == 'I') + (ch == 'i') + (ch == 'O') + (ch == 'o') +
             (ch == 'U') + (ch == 'u');
  }

  char out[128];
  int n = snprintf(out, 128, "{\"count\": %llu}", count);

  uint64_t offs_ = extism_alloc(n);
  extism_store(offs_, (const uint8_t *)out, n);
  extism_output_set(offs_, n);

  return 0;
}
