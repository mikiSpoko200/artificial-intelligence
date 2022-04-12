# -*- coding: utf-8 -*-

from __future__ import annotations

import random
from enum import Enum, auto
from typing import NamedTuple, Optional, Iterable, Protocol
from collections import deque


# INPUT_FILE, OUTPUT_FILE = get_in_out_file_names()


FULL_DEBUG      = False
DEBUG_ON        = True

DEBUG_LABYRINTH = True and DEBUG_ON or FULL_DEBUG
DEBUG_STATE     = False and DEBUG_ON or FULL_DEBUG


class Debug(Protocol):
    def debug(self) -> None: ...


class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()

    def __str__(self) -> str:
        return self.name[0]

    @staticmethod
    def pick_random() -> Direction:
        return random.choice(Direction.variants)

    def reverse(self) -> Direction:
        match self:
            case Direction.UP:
                return Direction.DOWN
            case Direction.DOWN:
                return Direction.UP
            case Direction.LEFT:
                return Direction.RIGHT
            case Direction.RIGHT:
                return Direction.LEFT

    @classmethod
    def from_string(cls, _: str) -> Direction:
        match _:
            case 'U':
                return Direction.UP
            case 'D':
                return Direction.DOWN
            case 'L':
                return Direction.LEFT
            case 'R':
                return Direction.RIGHT


Direction.variants = list(Direction)


class Position(NamedTuple):
    """Position in a buffer."""
    x: int
    y: int

    def move(self, _: Direction) -> Position:
        match _:
            case Direction.UP:
                return Position(self.x, self.y - 1)
            case Direction.DOWN:
                return Position(self.x, self.y + 1)
            case Direction.LEFT:
                return Position(self.x - 1, self.y)
            case Direction.RIGHT:
                return Position(self.x + 1, self.y)


class Labyrinth:
    __DEBUG = DEBUG_LABYRINTH
    MAX_STARTING_POSITION_COUNT = 2
    MAX_STARTING_MOVE_COUNT     = 108
    MAX_REDUCTION_ATTEMPTS: Optional[int] = None

    class TooManyReductionAttempts(Exception):
        """Exception raised to signify that program did not manage
        to reduce number of starting positions below MAX_STARTING_POSITION_COUNT
        in less than MAX_STARTING_MOVE_COUNT moves
        despite attempting MAX_REDUCTION_ATTEMPTS times.

        Probably some parameters should be changed.
        """

        def __init__(self, msg: str, *args) -> None:
            super().__init__(*args)
            self.msg = msg

        def __str__(self) -> str:
            return self.msg

    def __init__(self, buffer: list[list[str]]) -> None:
        self.buffer = buffer
        self.height = len(buffer)
        assert self.height > 0, "Labyrinth height must be greater than zero."
        self.width = len(buffer[0])
        self.starting_positions = set()
        self.ending_positions = set()

        for y, row in enumerate(buffer):
            for x, cell in enumerate(row):
                match cell:
                    case 'G':
                        self.ending_positions.add(Position(x, y))
                    case 'S':
                        self.starting_positions.add(Position(x, y))
                    case 'B':
                        self.starting_positions.add(Position(x, y))
                        self.ending_positions.add(Position(x, y))
                    case _:
                        pass

    @classmethod
    def from_string(cls, _: str) -> Labyrinth:
        lines = _.split('\n')
        buffer = [[cell for cell in row] for row in lines]
        return cls(buffer)

    def is_position_valid(self, _: Position) -> bool:
        # We can skip the boundary check because we know that Labyrinth will have a '#' border
        return self.buffer[_.y][_.x] != '#'

    def is_position_final(self, _: Position) -> bool:
        return _ in self.ending_positions

    def __str__(self) -> str:
        return '\n'.join(''.join(cell for cell in row) for row in self.buffer)

    def move_all_positions(self, positions: Iterable[Position], direction: Direction) -> Iterable[Position]:
        return (new_position if self.is_position_valid(new_position := position.move(direction))
                else position
                for position in positions)

    def reduce_starting_positions(self, flag=[True]) -> list[Direction]:
        """Generates random moves that reduce the number of starting points to acceptable level."""
        if len(flag) > 0 and flag.pop() and Labyrinth.__DEBUG: print(
            "Reduction parameters:\n"
            f"Initial starting position count : {len(self.starting_positions)}\n"
            f"MAX_STARTING_POSITION_COUNT     : {Labyrinth.MAX_STARTING_POSITION_COUNT}\n"
            f"MAX_STARTING_MOVE_COUNT         : {Labyrinth.MAX_STARTING_MOVE_COUNT}"
        )
        while True:
            prev_direction: Optional[Direction] = None
            starting_positions = self.starting_positions.copy()
            moves = list()
            for move_count in range(Labyrinth.MAX_STARTING_MOVE_COUNT):
                if len(starting_positions) > Labyrinth.MAX_STARTING_POSITION_COUNT:
                    direction = Direction.pick_random()
                    if prev_direction is not None:
                        while direction is prev_direction.reverse():
                            direction = Direction.pick_random()
                    starting_positions = set(self.move_all_positions(starting_positions, direction))
                    moves.append(direction)
                    prev_direction = direction
                else:
                    return moves

    def debug_update_starting_positions(self, starting_positions):
        for position in self.starting_positions:
            self.buffer[position.y][position.x] = ' '
        for position in starting_positions:
            self.buffer[position.y][position.x] = 'S'
        self.starting_positions = starting_positions

    def debug(self) -> None:
        if self.__DEBUG: print(self)


class State:
    __DEBUG = DEBUG_STATE
    labyrinth: Optional[Labyrinth] = None

    @staticmethod
    def set_labyrinth_ref(_: Labyrinth) -> None:
        State.labyrinth = _

    @staticmethod
    def remove_labyrinth_ref() -> None:
        State.labyrinth = None

    def __init__(self,
                 positions: tuple[Position],
                 parent_ref: Optional[State],
                 action: Optional[Direction],
                 depth: int
                 ) -> None:
        self.positions = positions
        self.parent_ref = parent_ref
        self.action = action
        self.depth = depth

    @classmethod
    def from_labyrinth(cls, labyrinth: Labyrinth) -> State:
        State.set_labyrinth_ref(labyrinth)
        start_point_positions = labyrinth.starting_positions.copy()
        return cls(tuple(start_point_positions), None, None, 0)

    def is_final(self) -> bool:
        return all(State.labyrinth.is_position_final(position) for position in self.positions)

    def move(self, direction: Direction) -> State:
        new_positions = tuple(State.labyrinth.move_all_positions(self.positions, direction))
        return State(new_positions, self, direction, self.depth + 1)

    def substates(self) -> list[State]:
        return [self.move(_) for _ in Direction]

    def __hash__(self) -> int:
        return hash(self.positions)

    def __str__(self) -> str:
        return f"Positions: {self.positions}, Action: {self.action}"

    def debug(self) -> None:
        if self.__DEBUG: print(self)


def solve(labyrinth_repr: str) -> str:
    MAX_PATH_LENGTH = 150
    labyrinth = Labyrinth.from_string(labyrinth_repr)
    debug(labyrinth)
    while True:
        initial_moves = labyrinth.reduce_starting_positions()
        init_state = State.from_labyrinth(labyrinth)
        for direction in initial_moves:
            init_state = init_state.move(direction)
        state_queue = deque([init_state])
        visited_positions = set()
        inner_running = True
        depth = 0
        while inner_running and len(state_queue) > 0:
            state = state_queue.popleft()
            if state.depth > depth:
                depth = state.depth
            debug(state)
            if state.positions in visited_positions:
                continue
            elif state.depth > MAX_PATH_LENGTH:
                break
            elif not state.is_final():
                state_queue.extend(state.substates())
                visited_positions.add(state.positions)
            else:
                path = []
                while (parent_state := state.parent_ref) is not None:
                    path.append(str(state.action))
                    state = parent_state
                if len(path) <= MAX_PATH_LENGTH:
                    return ''.join(path[::-1])
                else:
                    if DEBUG_ON: print(f"Path len: {len(path)}")


def read_from_file(file_name: str) -> str:
    with open(file_name, 'r', encoding="ascii") as input_file:
        return input_file.read()


def debug(_: Debug) -> None:
    _.debug()


def main():
    temp = "test1.txt"
    with open(temp or "zad_input.txt", 'r', encoding="utf-8") as infile:
        with open("zad_output.txt", 'w', encoding="utf-8") as outfile:
            moves = solve(infile.read())
            outfile.write(moves)
            if DEBUG_ON: print(f"Solution: {moves}\nLength: {len(moves)}")


if __name__ == "__main__":
    main()
