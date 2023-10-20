#include <stdint.h>

int32_t infinite_loop() {
  unsigned int i = 0;
  while (1) {
    i += 1;
    continue;
  }
  return 0;
}
