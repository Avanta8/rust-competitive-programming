#!/usr/bin/python3

import os
import pathlib
import sys

cwd = pathlib.Path(os.getcwd())
setup_path = pathlib.Path(__file__).parent / "template" / "src" / "setup.rs"


def get_filepath():
    if len(sys.argv) != 3:
        raise Exception("Incorrect usage.")

    return cwd / sys.argv[1] / "src" / "bin" / sys.argv[2]


def main():
    setup = setup_path.read_text()
    filepath = get_filepath()

    print(f"Writing setup to: {filepath}")
    filepath.write_text(setup)


if __name__ == "__main__":
    main()
