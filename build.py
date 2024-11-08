#!/usr/bin/env python3

import os
import platform
import shutil
import subprocess
import logging
from pathlib import Path
import argparse
import sys

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
    if system in ["linux", "darwin"]:
        return home / ".local" / "bin"
    elif system == "windows":
        return Path(os.getenv("USERHOME"))
    else:
        raise OSError(f"Unsupported OS {system}")


def data_dir() -> Path:
    if system == "linux":
        return home / ".local" / "share" / "extism-py"
    elif system == "darwin":
        return home / "Library" / "Application Support" / "extism-py"
    elif system == "windows":
        return Path(os.getenv("APPDATA")) / "extism-py"
    else:
        raise OSError(f"Unsupported OS {system}")


def exe_file() -> str:
    if system == "windows":
        return "extism-py.exe"
    else:
        return "extism-py"


def run_subprocess(command, cwd=None, quiet=False, env=None):
    try:
        logging.info(f"Running command: {' '.join(command)} in {cwd or '.'}")
        stdout = subprocess.DEVNULL if quiet else None
        stderr = subprocess.DEVNULL if quiet else None
        subprocess.run(command, cwd=cwd, check=True, stdout=stdout, stderr=stderr, env=env)
    except subprocess.CalledProcessError as e:
        logging.error(f"Command '{' '.join(command)}' failed with error: {e}")
        raise


def check_rust_installed():
    try:
        subprocess.run(
            ["rustc", "--version"],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        subprocess.run(
            ["cargo", "--version"],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError:
        logging.error(
            "Rust and Cargo are required but not found. Please install Rust: https://www.rust-lang.org/tools/install"
        )
        sys.exit(1)


def do_build(args):
    env = os.environ.copy()
    env['RUSTFLAGS'] = '-Ctarget-cpu=mvp'
    check_rust_installed()
    run_subprocess(["cargo", "+nightly", "build", "-Zbuild-std=panic_abort,std", "--release"], cwd="./lib", quiet=args.quiet, env=env)
    run_subprocess(["cargo", "build", "--release"], cwd="./bin", quiet=args.quiet)
    shutil.copy2(Path("./bin/target/release") / exe_file(), Path(".") / exe_file())
    logging.info("Build completed successfully.")


def do_install(args):
    do_build(args)
    bin_dir = Path(args.bin_dir)
    data_dir = Path(args.data_dir)
    bin_dir.mkdir(parents=True, exist_ok=True)
    data_dir.mkdir(parents=True, exist_ok=True)

    dest_path = bin_dir / exe_file()
    logging.info(f"Copying binary to {dest_path}")
    shutil.copy2(Path("./bin/target/release") / exe_file(), dest_path)

    logging.info(f"Copying data files to {data_dir}")
    shutil.copytree(
        Path("./lib/target/wasm32-wasi/wasi-deps"), data_dir, dirs_exist_ok=True
    )

    logging.info(f"{exe_file()} installed to {bin_dir}")
    logging.info(f"Data files installed to {data_dir}")
    logging.info("Installation completed successfully.")


def do_clean(args):
    logging.info("Cleaning build artifacts...")
    shutil.rmtree("./lib/target", ignore_errors=True)
    shutil.rmtree("./bin/target", ignore_errors=True)
    if (Path(".") / exe_file()).exists():
        (Path(".") / exe_file()).unlink()
    logging.info("Clean completed successfully.")


def get_version(path):
    try:
        result = subprocess.run(
            [path / exe_file(), "--version"], capture_output=True, text=True, check=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError:
        return "Unknown"


def main():
    parser = argparse.ArgumentParser(
        prog="build.py", description="Extism Python PDK builder"
    )
    parser.add_argument(
        "command",
        choices=["build", "install", "clean"],
        default="build",
        nargs="?",
        help="Command to run",
    )
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

    try:
        if args.command == "build":
            do_build(args)
        elif args.command == "install":
            do_install(args)
        elif args.command == "clean":
            do_clean(args)

        if args.command in ["install"]:
            version = get_version(Path(args.bin_dir))
            logging.info(f"extism-py version: {version}")
    except Exception as e:
        logging.error(f"An error occurred: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
