#!/usr/bin/env python3

import argparse
import platform
import subprocess
import shutil
import os
from pathlib import Path

system = platform.system().lower()
home = Path.home()


def bin_dir() -> Path:
    if system == "linux":
        return home / ".local" / "bin"
    else:
        raise Exception(f"Unsupported OS {system}")


def data_dir() -> Path:
    if system == "linux":
        return home / ".local" / "share" / "extism-py"
    else:
        raise Exception(f"Unsupported OS {system}")


def do_build(args):
    subprocess.run(["cargo", "build", "--release"], cwd="./lib")
    subprocess.run(["cargo", "build", "--release"], cwd="./bin")
    shutil.copyfile("./bin/target/release/extism-py", "./extism-py")
    shutil.copymode("./bin/target/release/extism-py", "./extism-py")


def do_install(args):
    do_build(args)
    bin_dir = args.bin_dir
    data_dir = args.data_dir
    os.makedirs(bin_dir, exist_ok=True)
    os.makedirs(data_dir, exist_ok=True)
    shutil.copyfile("./bin/target/release/extism-py", bin_dir / "extism-py")
    shutil.copymode("./bin/target/release/extism-py", bin_dir / "extism-py")
    shutil.copytree(
        "./lib/target/wasm32-wasi/wasi-deps/usr", data_dir, dirs_exist_ok=True
    )


def main():
    parser = argparse.ArgumentParser(
        prog="build.py", description="Extism Python PDK builder"
    )
    parser.add_argument("command", default="build")
    parser.add_argument("--bin-dir", default=bin_dir(), dest="bin_dir")
    parser.add_argument("--data-dir", default=data_dir(), dest="data_dir")
    args = parser.parse_args()

    command = args.command
    if command == "build":
        do_build(args)
    elif command == "install":
        do_install(args)
    else:
        print(f'Unsupported command: {command}')
        print('  Available commands: build, install')
        sys.exit(1)


if __name__ == "__main__":
    main()
