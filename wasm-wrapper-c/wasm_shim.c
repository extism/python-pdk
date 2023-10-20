#include "wasm_shim.h"
#include "extism/extism-pdk.h"
#include "pyhost.h"
#include "utils.h"

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define __FILENAME__                                                           \
  (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)

static PyObject *_plugin_module = NULL;

void _initialize() {
  // Uncomment and look if python script cannot be found
  Py_SetPythonHome("/plugin");
  Py_SetPath(L"/usr");
  print_current_dir();
  list_current_dir();

  if (_plugin_module != NULL) {
    char *msg3 = "plugin module already loaded";
    extism_log(msg3, strlen(msg3), ExtismLogInfo);
    exit(1);
  }

  char *msg3 = "initialize python";
  extism_log(msg3, strlen(msg3), ExtismLogInfo);

  pyhost_initialize(0);
  char *msg4 = "initialized python";
  extism_log(msg4, strlen(msg4), ExtismLogInfo);

  _plugin_module = pyhost_load_module("plugin");
}

u8 *allocate(i32 size) {
  LOG_MSG(__FILENAME__, "Called allocate(%d)", size);
  u8 *result = malloc(size);
  LOG_MSG(__FILENAME__, "allocate(%d) returning %p", size, result);
  return result;
}

void deallocate(u8 *pointer, i32 size) {
  LOG_MSG(__FILENAME__, "Called deallocate(%p, %d)", pointer, size);
  return free(pointer);
}

int32_t run_it() {
  _initialize();
  uint64_t length = extism_input_length();
  uint8_t input[length];
  extism_load_input(input, length);

  // LOG_MSG(__FILENAME__, "run_it: loading python func");
  char *msg = "Loaded input";
  extism_log(msg, strlen(msg), ExtismLogInfo);

  PyObject *pFunc = PyObject_GetAttrString(_plugin_module, "run_it");
  char *msg2 = "pfunc created";
  extism_log(msg2, strlen(msg2), ExtismLogInfo);

  if (!pFunc || !PyCallable_Check(pFunc)) {
    // if (PyErr_Occurred())
    //   PyErr_Print();
    Py_XDECREF(pFunc);
    return 1;
  }
  char *msg3 = "done";
  extism_log(msg3, strlen(msg3), ExtismLogInfo);

  return 0;

  // LOG_MSG(__FILENAME__, "run_it: loading input arg");

  // PyObject *pArgs = Py_BuildValue("s", input);
  // if (!pArgs) {
  //   if (PyErr_Occurred())
  //     PyErr_Print();
  //   LOG_MSG(__FILENAME__, "run_it: failed to convert");
  //   Py_XDECREF(pFunc);
  //   return 1;
  // }

  // PyObject *pValue = PyObject_CallObject(pFunc, pArgs);
  // if (pValue == NULL) {
  //   PyErr_Print();
  //   LOG_MSG(__FILENAME__, "run_it: call to python function failed!");
  //   Py_XDECREF(pArgs);
  //   Py_XDECREF(pFunc);
  //   return 1;
  // }

  // Py_XDECREF(pValue);
  // Py_XDECREF(pArgs);
  // Py_XDECREF(pFunc);

  //   LOG_MSG(__FILENAME__, "id=%d | run_e: completed", ident);

  LOG_MSG(__FILENAME__, "Called run_it %s", input);
  return 0;
}

// void run_e(u8 *pointer, i32 size, i32 ident) {
//   _initialize();
//   LOG_MSG(__FILENAME__, "id=%d | Called run_e(%p, %d, %d)", ident, pointer,
//           size, ident);
//   if (_plugin_module == NULL) {
//     LOG_MSG(__FILENAME__, "id=%d | run_e: plugin module was not loaded!",
//             ident);
//     exit(1);
//   }

//   PyObject *pFunc = PyObject_GetAttrString(_plugin_module, "run_e");
//   if (!pFunc || !PyCallable_Check(pFunc)) {
//     if (PyErr_Occurred())
//       PyErr_Print();
//     LOG_MSG(__FILENAME__, "id=%d | run_e: cannot find function \"%s\"!",
//     ident,
//             "run_e");
//     Py_XDECREF(pFunc);
//     return;
//   }

//   PyObject *pArgs = Py_BuildValue("s#i", pointer, size, ident);
//   if (!pArgs) {
//     if (PyErr_Occurred())
//       PyErr_Print();
//     LOG_MSG(__FILENAME__,
//             "id=%d | run_e: failed to convert arguments pointer=%p, size=%d,
//             " "ident=%d!", ident, pointer, size, ident);
//     Py_XDECREF(pFunc);
//     return;
//   }

//   LOG_MSG(__FILENAME__, "id=%d | Calling CPython plugin.run_e(\"%.*s\", %d)",
//           ident, size, (char *)pointer, ident);
//   PyObject *pValue = PyObject_CallObject(pFunc, pArgs);
//   if (pValue == NULL) {
//     PyErr_Print();
//     LOG_MSG(__FILENAME__, "id=%d | run_e: call to python function failed!",
//             ident);
//     Py_XDECREF(pArgs);
//     Py_XDECREF(pFunc);
//     return;
//   }

//   Py_XDECREF(pValue);
//   Py_XDECREF(pArgs);
//   Py_XDECREF(pFunc);

//   LOG_MSG(__FILENAME__, "id=%d | run_e: completed", ident);
// }
