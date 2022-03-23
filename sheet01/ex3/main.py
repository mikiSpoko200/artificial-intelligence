# -*- encoding: utf-8 -*-

"""
Nikt nie będzie miał:
 - pokera królewskiego
Układy tylko u blotkarza:
 - poker
 - strit
Wspólne układy:
 - kareta
 - full
 - kolor
 - trójka
 - dwie pary
 - para
 - wysoka karta

Nie muszę implementować pełnego porównywania bo wiadomo, że figurant zawsze będzie miał lepszą wartość układu
w przypadku kolizji.
"""


from typing import NamedTuple
from enum import Enum, auto
import random


class UkladyFigurant(Enum):
    KARETA = 2
    FUL = 3
    KOLOR = 4
    TROJKA = 6
    DWIE_PARY = 7
    PARA = 8
    WYSOKA_KARTA = 9


class UkladyBlotkarz(Enum):
    POKER = 1
    KARETA = 2
    FUL = 3
    KOLOR = 4
    STRIT = 5
    TROJKA = 6
    DWIE_PARY = 7
    PARA = 8
    WYSOKA_KARTA = 9


class Player(Enum):
    BLOTKARZ = auto()
    FIGURANT = auto()


class Suit(Enum):
    SPADES   = auto()
    HEARTS   = auto()
    DIAMONDS = auto()
    CLUBS    = auto()


class Card(NamedTuple):
    suit: Suit
    value: int

KARTY_FIGURY = [
    Card(suit, value) for suit in Suit for value in range(11, 15)
] * 5

KARTY_BLOTKARZ = [
    Card(suit, value) for suit in Suit for value in range(2, 11)
] * 5


def znajdz_uklad()


def uklad_blotkarz(blotkarz: list[Card]) -> UkladyBlotkarz:
    


def compare_hand(blotkarz: list[Card], figurant: list[Card]) -> Player:
    uklad_blotkarz =


def main():
    blotkarz = []
    figurant = []
    wygrane_blotkarz = 0
    wygrane_figurant = 0
    for _ in range(10_000):
        random.shuffle(KARTY_BLOTKARZ)
        random.shuffle(KARTY_FIGURY)
        blotkarz = KARTY_BLOTKARZ[:5]
        figurant = KARTY_FIGURY[:5]
        match compare_hand(blotkarz, figurant):
            case Player.FIGURANT:
                figurant += 1
            case Player.BLOTKARZ:
                blotkarz += 1

