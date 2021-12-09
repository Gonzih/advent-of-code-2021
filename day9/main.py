from __future__ import annotations
# from typing import list, set
from functools import reduce

GREEN = '\033[0;32m'
NC = '\033[0m'


def green_color(s: str) -> str:
    return f"{GREEN}{s}{NC}"


class Vec2:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"({self.x}, {self.y})"

    def __eq__(self, other: object):
        if isinstance(other, Vec2):
            return self.x == other.x and self.y == other.y
        return False

    def __add__(self, other: Vec2):
        return Vec2(self.x + other.x, self.y + other.y)

    def __sub__(self, other: Vec2):
        return Vec2(self.x - other.x, self.y - other.y)

    def __copy__(self) -> Vec2:
        return Vec2(self.x, self.y)

    def __hash__(self):
        return hash(str(self))

    def invert(self) -> Vec2:
        return Vec2(0, 0) - self

    def within_box(self, start: Vec2, end: Vec2) -> bool:
        return self.x >= start.x and\
            self.y >= start.y and\
            self.x < end.x and\
            self.y < end.y


class Field:
    def __init__(self):
        self.field = []
        self.width = 0
        self.height = 0

    def add_line(self, line: str):
        parsed_line = [int(s) for s in line.strip()]
        self.field.extend(parsed_line)
        self.width = len(parsed_line)
        self.height += 1

    def neighbours(self, pos: Vec2) -> list[Vec2]:
        start = Vec2(0, 0)
        end = Vec2(self.width, self.height)
        offsets = [Vec2(0, 1), Vec2(1, 0), Vec2(0, -1), Vec2(-1, 0)]

        return [nb for nb in [pos + off for off in offsets]
                if nb.within_box(start, end)]

    def basin_size(self, pos: Vec2) -> int:
        run_history: set[Vec2] = set()
        to_process: list[Vec2] = [pos]
        count = 0

        while len(to_process) > 0:
            new_batch: list[Vec2] = []
            for point in to_process:
                if point not in self.basin_history:
                    count += 1
                self.basin_history.add(point)
                run_history.add(point)
                new_batch.extend(
                    [nb for nb in self.neighbours(point)
                        if self.get(nb) != 9 and nb not in self.basin_history])
            to_process = new_batch

        # self.print(run_history)
        # print(f"{pos} -> {count}")

        return count

    def get(self, pos: Vec2) -> int:
        return self.field[pos.y * self.width + pos.x]

    def lowest(self, pos: Vec2) -> bool:
        # print(f"{pos} -> {self.neighbours(pos)}")
        return all(self.get(pos) < self.get(neighbour)
                   for neighbour in self.neighbours(pos))

    def count_lowests(self) -> int:
        sum = 0
        for x in range(self.width):
            for y in range(self.height):
                p = Vec2(x, y)
                if self.lowest(p):
                    sum += self.get(p) + 1
        return sum

    def count_basins(self) -> int:
        self.basin_history: set[Vec2] = set()
        basins = []

        for y in range(self.height):
            for x in range(self.width):
                p = Vec2(x, y)
                if self.lowest(p) and p not in self.basin_history:
                    basins.append(self.basin_size(p))

        basins.sort(reverse=True)
        print(basins)
        print(basins[:3])
        return reduce(lambda a, b: a*b, basins[:3])

    def print(self, highlight: set[Vec2]):
        out = ''
        for y in range(self.height):
            line = ''
            for x in range(self.width):
                p = Vec2(x, y)
                c = str(self.get(p))
                if p in highlight:
                    c = green_color(c)
                line += (c)
            out += (f"{line}\n")

        print(out)


def run():
    field = Field()
    with open('input.txt') as input:
        for line in input:
            field.add_line(line)

    risk = field.count_lowests()
    print(risk)
    # assert(risk == 15)
    assert(risk == 436)

    basins = field.count_basins()
    print(basins)
    # assert(basins == 1134)
    assert(basins == 1317792)


run()
