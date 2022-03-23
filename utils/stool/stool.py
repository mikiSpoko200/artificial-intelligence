# -*- encoding: utf-8 -*-


from pathlib import Path
import argparse
import os


def get_in_out_file_names() -> tuple[str, str]:
    ex_number = int(Path.cwd().name[2:])
    return f"zad{ex_number}_input.txt", f"zad{ex_number}_output.txt"


def create_in_out_files(ex_number: int = None) -> None:
    """Create input and output data files.
    It is assumed that the script is run in directory containing the solution.
    """
    if ex_number is None:
        cwd = Path.cwd()
        ex_number = int(cwd.name[2:])
    open(f"zad{ex_number}_input.txt", "w").close()
    open(f"zad{ex_number}_output.txt", "w").close()


def ex_setup(ex_number: int) -> None:
    """Setup for a new exercise with given number.
    It is assumed that the script is run in the sheet directory.

    Each ex will contain test.bat script that will run all the tests using validator.py present in sheet directory.
    As well as zad<ex_number>_input/output.txt files.
    """

    ex = Path.cwd() / f"ex{ex_number}"
    os.mkdir(ex)
    os.chdir(ex)
    print(Path.cwd())
    with open("test.bat", "w", encoding="utf-8") as test_script:
        test_script.writelines([
            "@echo off" + os.linesep,
            r"call ..\..\.venv\Scripts\activate" + os.linesep,
            f"py ../validator.py zad{ex_number} py main.py" + os.linesep,
            r"call deactivate" + os.linesep,
            "pause"
        ])
    create_in_out_files(ex_number)
    os.chdir("..")


def setup_sheet(sheet_number: int, ex_numbers: list[int]) -> None:
    """Setup for new executrices sheet.
    It is assumed that script runs in root artificial-intelligence folder.

    Script creates new directory labeled sheet<sheet_number> and populates it with subdirectories for each exercise.
    """
    sheet = Path.cwd() / f"sheet{sheet_number:02}"
    os.mkdir(sheet)
    os.chdir(sheet)
    print(Path.cwd())
    for ex_number in ex_numbers:
        ex_setup(ex_number)
    os.chdir("..")


def main():
    parser = argparse.ArgumentParser("AI course project setup utility.")
    subparsers = parser.add_subparsers(dest="mode")
    sheet_parser = subparsers.add_parser("sheet", help="Entire sheet setup.")
    ex_parser = subparsers.add_parser("ex", help="Singular exercise setup.")
    sheet_parser.add_argument(
        "number",
        type=int,
        choices=[*range(1, 8)],
        help="Sheet number."
    )
    sheet_parser.add_argument(
        "ex_numbers",
        nargs="+",
        type=int,
        help="Number of exercises in the sheet.",
    )
    ex_parser.add_argument(
        "number",
        type=int,
        help="Exercise number."
    )
    namespace = parser.parse_args()
    print(namespace)
    if namespace.mode == "sheet":
        setup_sheet(namespace.number, namespace.ex_numbers)
    else:
        ex_setup(namespace.number)


if __name__ == '__main__':
    main()
