#!/usr/bin/env python3

import os
import platform
import shutil
import subprocess
import logging
from pathlib import Path
import argparse

system = platform.system().lower()
home = Path.home()


def set_log_level(log_level):
    numeric_level = getattr(logging, log_level.upper(), None)
    if not isinstance(numeric_level, int):
        raise ValueError(f"Invalid log level: {log_level}")
    logging.basicConfig(
        level=numeric_level, format="%(asctime)s - %(levelname)s - %(message)s"
    )


def bin_dir() -> Path:
    if system == "linux":
        return home / ".local" / "bin"
    elif system == "darwin":  # macOS
        return home / ".local" / "bin"
    elif system == "windows":
        return Path(os.getenv("APPDATA")) / "Python" / "Scripts"
    else:
        raise Exception(f"Unsupported OS {system}")


def data_dir() -> Path:
    if system == "linux":
        return home / ".local" / "share" / "extism-py"
    elif system == "darwin":  # macOS
        return home / "Library" / "Application Support" / "extism-py"
    elif system == "windows":
        return Path(os.getenv("APPDATA")) / "extism-py"
    else:
        raise Exception(f"Unsupported OS {system}")


def run_subprocess(command, cwd=None, quiet=False):
    try:
        logging.info(f"Running command: {' '.join(command)} in {cwd}")
        if quiet:
            subprocess.run(command, cwd=cwd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        else:
            subprocess.run(command, cwd=cwd, check=True)
    except subprocess.CalledProcessError as e:
        logging.error(f"Command '{' '.join(command)}' failed with error: {e}")
        raise


def do_build(args):
    run_subprocess(["cargo", "build", "--release"], cwd="./lib", quiet=args.quiet)
    run_subprocess(["cargo", "build", "--release"], cwd="./bin", quiet=args.quiet)
    shutil.copyfile("./bin/target/release/extism-py", "./extism-py")
    shutil.copymode("./bin/target/release/extism-py", "./extism-py")


def do_install(args):
    do_build(args)
    bin_dir = args.bin_dir
    data_dir = args.data_dir
    os.makedirs(bin_dir, exist_ok=True)
    os.makedirs(data_dir, exist_ok=True)
    logging.info(f"Copying binary to {bin_dir / 'extism-py'}")
    shutil.copyfile("./bin/target/release/extism-py", bin_dir / "extism-py")
    shutil.copymode("./bin/target/release/extism-py", bin_dir / "extism-py")
    logging.info(f"Copying data files to {data_dir}")
    shutil.copytree(
        "./lib/target/wasm32-wasi/wasi-deps/usr", data_dir, dirs_exist_ok=True
    )
    if not args.quiet:
        print(f"extism-py installed to {bin_dir}")
        print(f"Data files installed to {data_dir}")


def main():
    parser = argparse.ArgumentParser(
        prog="build.py", description="Extism Python PDK builder"
    )
    parser.add_argument("command", choices=["build", "install"], help="Command to run")
    parser.add_argument(
        "--bin-dir",
        default=bin_dir(),
        dest="bin_dir",
        help="Directory to install binaries",
    )
    parser.add_argument(
        "--data-dir",
        default=data_dir(),
        dest="data_dir",
        help="Directory to install data files",
    )
    parser.add_argument(
        "--log-level",
        default="INFO",
        choices=["debug", "info", "warning", "error", "critical"],
        help="Set the logging level",
    )
    parser.add_argument("--quiet", "-q", action="store_true", help="Suppress output")

    args = parser.parse_args()

    if not args.quiet:
        set_log_level(args.log_level)

    if args.command == "build":
        do_build(args)
    elif args.command == "install":
        do_install(args)


if __name__ == "__main__":
    main()
