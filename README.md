# Extism Python PDK

![GitHub License](https://img.shields.io/github/license/extism/extism)
![GitHub release (with filter)](https://img.shields.io/github/v/release/extism/python-pdk)

This project contains a tool that can be used to create
[Extism Plug-ins](https://extism.org/docs/concepts/plug-in) in Python.

## Overview

This PDK uses [PyO3](https://github.com/PyO3/pyo3) and
[wizer](https://github.com/bytecodealliance/wizer) to run Python code as an
Extism Plug-in.

## Install the compiler

We release the compiler as native binaries you can download and run. Check the
[releases](https://github.com/extism/python-pdk/releases) page for the latest.

## Install Script

### Linux, macOS

```bash
curl -Ls https://raw.githubusercontent.com/extism/python-pdk/main/install.sh | bash
```

This will install `extism-py` (and `wasm-merge`/`wasm-opt` if not already
installed) to `$HOME/.local/bin` and create `$HOME/.local/share/extism-py`

### Testing the Install

> _Note_: [Binaryen](https://github.com/WebAssembly/binaryen), specifically the
> `wasm-merge` and `wasm-opt` tools are required as a dependency. We will try to
> package this up eventually but for now it must be reachable on your machine.
> You can install on mac with `brew install binaryen` or see their
> [releases page](https://github.com/WebAssembly/binaryen/releases).

Then run command with no args to see the help:

```
extism-py
error: The following required arguments were not provided:
    <input-py>

USAGE:
    extism-py <input-py> -o <output>

For more information try --help
```

> **Note**: If you are using mac, you may need to tell your security system this
> unsigned binary is fine. If you think this is dangerous, or can't get it to
> work, see the "compile from source" section below.

## Getting Started

The goal of writing an
[Extism plug-in](https://extism.org/docs/concepts/plug-in) is to compile your
Python code to a Wasm module with exported functions that the host application
can invoke. The first thing you should understand is creating an export.

### Exports

Let's write a simple program that exports a `greet` function which will take a
name as a string and return a greeting string. Paste this into a file
`plugin.py`:

```python
import extism

@extism.plugin_fn
def greet():
  name = extism.input_str()
  extism.output_str(f"Hello, {name}")
```

Some things to note about this code:

1. We can export functions by name using the `extism.plugin_fn` decorator. This
   allows the host to invoke this function.
2. In this PDK we code directly to the ABI. We get input from the using using
   `extism.input*` functions and we return data back with the `extism.output*`
   functions.

Let's compile this to Wasm now using the `extism-py` tool:

```bash
extism-py plugin.py -o plugin.wasm
```

We can now test `plugin.wasm` using the
[Extism CLI](https://github.com/extism/cli)'s `run` command:

```bash
extism call plugin.wasm greet --input="Benjamin" --wasi
# => Hello, Benjamin!
```

> **Note**: Currently `wasi` must be provided for all Python plug-ins even if
> they don't need system access.

> **Note**: We also have a web-based, plug-in tester called the
> [Extism Playground](https://playground.extism.org/)

### More Exports: Error Handling

We catch any exceptions thrown and return them as errors to the host. Suppose we
want to re-write our greeting module to never greet Benjamins:

```python
import extism

@extism.plugin_fn
def greet():
  name = extism.input_str()
  if name == "Benjamin":
    raise Exception("Sorry, we don't greet Benjamins!")
  extism.output_str(f"Hello, {name}")
```

Now compile and run:

```bash
extism-py plugin.py -o plugin.wasm
extism call plugin.wasm greet --input="Benjamin" --wasi
# => Error: Sorry, we don't greet Benjamins!:
# =>  File "<source>", line 17, in __invoke
# =>  File "<source>", line 9, in greet
echo $? # print last status code
# => 1
extism call plugin.wasm greet --input="Zach" --wasi
# => Hello, Zach!
echo $?
# => 0
```

### JSON

```python
import extism

@extism.plugin_fn
def sum():
  params = extism.input_json()
  extism.output_json({"sum": params['a'] + params['b']})
```

```bash
extism call plugin.wasm sum --input='{"a": 20, "b": 21}' --wasi
# => {"sum":41}
```

You can also specify your input and output types as dataclasses using
`extism.Json`:

```python
from typing import Optional, List 
from dataclasses import dataclass

# ...

@dataclass
class User(extism.Json):
  admin: bool
  name: Optional[str]
  email: str
  addresses: List[Address]


@extism.plugin_fn
def reflect_user():
  input = extism.input(User)
  extism.output(input)
```

### Configs

Configs are key-value pairs that can be passed in by the host when creating a
plug-in. These can be useful to statically configure the plug-in with some data
that exists across every function call. Here is a trivial example using
`Config.get`:

```python
import extism

@extism.plugin_fn
def greet():
  user = extism.Config.get("user")
  extism.output_str(f"Hello, {user}!")
```

To test it, the [Extism CLI](https://github.com/extism/cli) has a `--config`
option that lets you pass in `key=value` pairs:

```bash
extism call plugin.wasm greet --config user=Benjamin --wasi
# => Hello, Benjamin!
```

### Logging

At the current time, calling `console.log` emits an `info` log. Please file an
issue or PR if you want to expose the raw logging interface:

```python
import extism

@extism.plugin_fn
def log_stuff():
  extism.log(Extism.LogLevel.Info, "Hello, world!")
```

Running it, you need to pass a log-level flag:

```
extism call plugin.wasm logStuff --wasi --log-level=info
# => 2023/10/17 14:25:00 Hello, World!
```

## Generating Bindings

It's often very useful to define a schema to describe the function signatures
and types you want to use between Extism SDK and PDK languages.

[XTP Bindgen](https://github.com/dylibso/xtp-bindgen) is an open source
framework to generate PDK bindings for Extism plug-ins. It's used by the
[XTP Platform](https://www.getxtp.com/), but can be used outside of the platform
to define any Extism compatible plug-in system.

### 1. Install the `xtp` CLI.

See installation instructions
[here](https://docs.xtp.dylibso.com/docs/cli#installation).

### 2. Create a schema using our OpenAPI-inspired IDL:

```yaml
version: v1-draft
exports: 
  CountVowels:
      input: 
          type: string
          contentType: text/plain; charset=utf-8
      output:
          $ref: "#/components/schemas/VowelReport"
          contentType: application/json
# components.schemas defined in example-schema.yaml...
```

> See an example in [example-schema.yaml](./example-schema.yaml), or a full
> "kitchen sink" example on
> [the docs page](https://docs.xtp.dylibso.com/docs/concepts/xtp-schema/).

### 3. Generate bindings to use from your plugins:

```
xtp plugin init --schema-file ./example-schema.yaml
    1. TypeScript                      
    2. Go                              
    3. Rust                            
  > 4. Python                          
    5. C#                              
    6. Zig                             
    7. C++                             
    8. GitHub Template                 
    9. Local Template
```

This will create an entire boilerplate plugin project for you to get started
with:

```python
# returns VowelReport (The result of counting vowels on the Vowels input.)
def count_vowels(input: str) -> VowelReport:
    raise Exception("Unimplemented: CountVowels")
```

Implement the empty function(s), and run `xtp plugin build` to compile your
plugin.

> For more information about XTP Bindgen, see the
> [dylibso/xtp-bindgen](https://github.com/dylibso/xtp-bindgen) repository and
> the official
> [XTP Schema documentation](https://docs.xtp.dylibso.com/docs/concepts/xtp-schema).

## Compiling the compiler from source

### Prerequisites

Before compiling the compiler, you need to install prerequisites.

1. Install Rust using [rustup](https://rustup.rs)
2. Install the WASI target platform via
   `rustup target add --toolchain stable wasm32-wasi`
3. Install [CMake](https://cmake.org/install/) (on macOS with homebrew,
   `brew install cmake`)
4. Install [Binaryen](https://github.com/WebAssembly/binaryen/) and add it's
   install location to your PATH (only wasm-opt is required for build process)
5. Install [7zip](https://www.7-zip.org/)(only for Windows)

### Compiling from source

Run make to compile the core crate (the engine) and the cli:

```
./build.py
```

To test the built compiler (ensure you have Extism installed):

```bash
./extism-py examples/count-vowels.py -o count-vowels.wasm
extism call out.wasm count_vowels --wasi --input='Hello World Test!'
# => "{\"count\":4}"
```

## How it works

This works a little differently than other PDKs. You cannot compile Python to
Wasm because it doesn't have an appropriate type system to do this. The
`extism-py` command we have provided here is a little compiler / wrapper that
does a series of things for you:

1. It loads an "engine" Wasm program containing the Python runtime
2. It initializes the Python runtime
3. It loads your Python source code into memory
4. It parses the Python source code for exports and generates 1-to-1 proxy
   export functions in Wasm
5. It freezes and emits the machine state as a new Wasm file at this
   post-initialized point in time

This new Wasm file can be used just like any other Extism plugin.
