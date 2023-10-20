#define PY_SSIZE_T_CLEAN

#include "extism/extism-pdk.h"
#include "pyhost.h"
#include "utils.h"
#include "wasm_shim.h"

int main(int argc, char *argv[]) {
  char *msg2 = "calling _start";
  extism_log(msg2, strlen(msg2), ExtismLogInfo);
  _initialize();
  char *msg3 = "initalized called";
  extism_log(msg3, strlen(msg3), ExtismLogInfo);
  return 0;
}
