# -*- encoding: utf-8 -*-

"""
Moje rozwiązanie działa przechodzi oknem szerokości D po całym ciągu a następnie oblicza dwie wartości:
  1. ilość zgaszonych bitów wewnątrz okna
  2. ilość zapalonych bitów poza oknem
Wówczas suma tych wartości da nam ilość operacji zapalania / gaszenia bitu jakie musielibyśmy
wykonać aby otrzymać dokładnie jeden ciąg zapalonych bitów długości D otoczony przez zera.
"""

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


def solve(input_file: str, output_file: str) -> None:
    with open(input_file, 'r') as in_file, open(output_file, 'w') as out_file:
        for line in in_file.read().split('\n'):
            if line:
                sequence, d = line.split(' ')
                out_file.write(str(opt_dist([int(num) for num in sequence], int(d))) + '\n')


def main():
    solve(IN_FILE_NAME, OUT_FILE_NAME)


if __name__ == '__main__':
    main()
