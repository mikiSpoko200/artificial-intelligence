# -*- encoding: utf-8 -*-

"""Program



"""


from typing import Optional
from stool.stool import get_in_out_file_names


DICTIONARY_PATH = r"words_for_ai1.txt"
IN_FILE_NAME, OUT_FILE_NAME = get_in_out_file_names()


with open(DICTIONARY_PATH, 'r', encoding='UTF-8') as file:
    DICTIONARY = {phrase for phrase in file.read().split()}


def solve(input_file: str, output_file: str) -> None:
    def division_length(path: list[str]) -> int:
        return sum([len(phrase)**2 for phrase in path])

    def find_optimal_divisions(phrase: str, memo) -> Optional[list[str]]:
        if phrase in memo:
            return memo[phrase]
        if phrase in DICTIONARY:
            return [phrase]
        best_path = None
        # Find possible division points that can create meaningful words.
        division_indexes = [i for i in range(1, len(phrase) + 1) if phrase[:i] in DICTIONARY][::-1]
        if division_indexes:
            words, remainders = [phrase[:i] for i in division_indexes], [phrase[i:] for i in division_indexes]
            for word, remainder in zip(words, remainders):
                remainder_division = find_optimal_divisions(remainder, memo)
                if remainder_division is not None:
                    result = [word] + remainder_division
                    if best_path is None or division_length(result) > division_length(best_path):
                        best_path = result
                        memo[phrase] = result
        else:
            memo[phrase] = None

        return best_path

    with open(input_file, 'r', encoding='utf-8') as in_file, open(output_file, 'w', encoding='utf-8') as out_file:
        for line in in_file.read().split():
            out_file.write(' '.join(find_optimal_divisions(line, {})) + '\n')


if __name__ == '__main__':
    solve(IN_FILE_NAME, OUT_FILE_NAME)
