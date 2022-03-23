# -*- encoding: utf-8 -*-

from __future__ import annotations

import random
from enum import Enum, auto
from random import randint
from typing import Optional

from stool.stool import get_in_out_file_names


IN_FILE_NAME, OUT_FILE_NAME = get_in_out_file_names()


def opt_dist(binary_array: list[int] | bytearray, D: int) -> int:
    num_of_switches = set()
    possible_divisions = [
        (binary_array[i:D + i], binary_array[:i] + binary_array[D + i:]) for i in range(0, len(binary_array) - D + 1)
    ]

    for window, rest in possible_divisions:
        window_on_bits = sum(window)
        window_off_bits = D - window_on_bits
        rest_on_bits = sum(rest)
        num_of_switches.add(window_off_bits + rest_on_bits)

    return min(num_of_switches)


class Nonogram:
    
    ITER_LIMIT = 25_000

    class Status(Enum):
        SUCCESS = auto()
        FAILURE = auto()

    def __init__(self, row_counts: list[int], col_counts: list[int]) -> None:
        self.row_counts = row_counts
        self.col_counts = col_counts
        self.row_buffer: bytearray = bytearray(len(row_counts) * len(col_counts))
        self.col_buffer: bytearray = bytearray(len(row_counts) * len(col_counts))
        self.row_flips = list()
        self.col_flips = list()

    @classmethod
    def from_str(cls, data: str) -> Nonogram:
        lines = data.strip().split('\n')
        height, *_ = map(int, lines[0].split(" "))
        rows_counts = list(map(int, lines[1: 1 + height]))
        cols_counts = list(map(int, lines[1 + height:]))
        return cls(rows_counts, cols_counts)

    @property
    def width(self) -> int:
        """Width of the nonogram."""
        return len(self.col_counts)

    @property
    def height(self) -> int:
        """Height of the nonogram."""
        return len(self.row_counts)
        
    def rows(self) -> list[bytearray]:
        """Iterator over rows."""
        return list(self.row_buffer[self.width * i: self.width * (i + 1)] for i in range(self.height))

    def cols(self) -> list[bytearray]:
        """Iterator over columns."""
        return list(self.col_buffer[self.height * i: self.height * (i + 1)] for i in range(self.width))

    def randomize(self) -> None:
        """Randomize pixel values."""
        self.row_buffer = [randint(0, 1) for _ in range(self.width * self.height)]
        self.col_buffer = [self.row_buffer[i + j * self.height] for i in range(self.width) for j in range(self.height)]
        self.row_flips = [self.count_row_flips(row_number) for row_number in range(self.height)]
        self.col_flips = [self.count_col_flips(col_number) for col_number in range(self.width)]

    def count_col_flips(self, col_number: int) -> int:
        return opt_dist(self.col_buffer[col_number * self.height: (col_number + 1) * self.height], self.col_counts[col_number])

    def count_row_flips(self, row_number: int) -> int:
        return opt_dist(self.row_buffer[row_number * self.width: (row_number + 1) * self.width], self.row_counts[row_number])

    def pick_row(self) -> Optional[int]:
        incorrect_row_indices = [i for i, _ in enumerate(self.rows()) if self.row_flips[i] > 0]
        if incorrect_row_indices:
            return random.choice(incorrect_row_indices)
        return None

    def pick_col(self) -> Optional[int]:
        incorrect_col_indices = [i for i, _ in enumerate(self.cols()) if self.col_flips[i] > 0]
        if incorrect_col_indices:
            return random.choice(incorrect_col_indices)
        return None

    @staticmethod
    def is_row_col_correct(data: bytearray | list[int], expected_count: int) -> bool:
        return opt_dist(data, expected_count) == 0

    def is_correct(self) -> bool:
        """Determine if current pixel configuration is correct."""
        return all([flip_count == 0 for flip_count in self.row_flips + self.col_flips])

    def flip(self, row_number: int, col_number: int) -> None:
        self.row_buffer[row_number * self.width + col_number] ^= 1
        self.col_buffer[col_number * self.height + row_number] ^= 1

    def update(self) -> Nonogram.Status:
        row_number = self.pick_row()
        col_number = self.pick_col()
        if row_number is None and col_number is None:  # nonogram is correct
            return Nonogram.Status.SUCCESS
        if row_number is not None:  # row picked
            best_col_number = 0
            best_row_flip_count = self.width + self.height + 1
            best_col_flip_count = self.width + self.height + 1
            for col_number, _ in enumerate(self.cols()):
                self.flip(row_number, col_number)
                new_row_flip_count = self.count_row_flips(row_number)
                new_col_flip_count = self.count_col_flips(col_number)
                if new_col_flip_count + new_row_flip_count < best_col_flip_count + best_row_flip_count:
                    best_col_number = col_number
                    best_row_flip_count = new_row_flip_count
                    best_col_flip_count = new_col_flip_count
                self.flip(row_number, col_number)
            self.flip(row_number, best_col_number)
            self.row_flips[row_number] = best_row_flip_count
            self.col_flips[best_col_number] = best_col_flip_count
        else:  # col picked
            best_row_number = 0
            best_row_flip_count = self.width + self.height + 1
            best_col_flip_count = self.width + self.height + 1
            for row_number, _ in enumerate(self.rows()):
                self.flip(row_number, col_number)
                new_row_flip_count = self.count_row_flips(row_number)
                new_col_flip_count = self.count_col_flips(col_number)
                if new_row_flip_count + new_col_flip_count < best_row_flip_count + best_col_flip_count:
                    best_row_number = row_number
                    best_row_flip_count = new_row_flip_count
                    best_col_flip_count = new_col_flip_count
                self.flip(row_number, col_number)
            self.flip(best_row_number, col_number)
            self.row_flips[best_row_number] = best_row_flip_count
            self.col_flips[col_number] = best_col_flip_count
        return Nonogram.Status.FAILURE

    def __str__(self) -> str:
        str_builder = [''.join(map(lambda elem: "#" if elem else ".", row)) for row in self.rows()]
        return "\n".join(str_builder)


DEBUG = False


def solve(input_file: str, output_file: str) -> None:
    with open(input_file, 'r') as in_file, open(output_file, 'w') as out_file:
        counter = 0
        n = Nonogram.from_str(in_file.read())
        n.randomize()
        if DEBUG:
            n.update()
        else:
            while not n.is_correct():
                counter += 1
                if counter > Nonogram.ITER_LIMIT:
                    counter = 0
                    n.randomize()
                match n.update():
                    case Nonogram.Status.SUCCESS:
                        break
                    case Nonogram.Status.FAILURE:
                        pass
            out_file.write(str(n))


def main():
    solve(IN_FILE_NAME, OUT_FILE_NAME)


if __name__ == '__main__':
    main()
