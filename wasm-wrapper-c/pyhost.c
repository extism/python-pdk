#include "pyhost.h"
#include "extism/extism-pdk.h"

int pyhost_initialize(int argc) {
  Py_SetPythonHome(L"/usr");
  // Py_SetPath(L"/plugin");
  char *msg3 = "initalizing";
  extism_log(msg3, strlen(msg3), ExtismLogInfo);
  Py_Initialize();
  if (PyErr_Occurred()) {
    PyErr_Print();
  }

  return 0;
}

PyObject *pyhost_load_module(char *module_name) {
  PyObject *pName = PyUnicode_DecodeFSDefault(module_name);
  /* Error checking of pName left out */

  PyObject *pModule = PyImport_Import(pName);
  Py_DECREF(pName);
  return pModule;
}
