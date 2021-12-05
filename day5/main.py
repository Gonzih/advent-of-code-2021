from __future__ import annotations
from typing import Dict, List


class Vec2:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return f"({self.x}, {self.y})"

    def __eq__(self, other: Vec2):
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
        return Vec2(-self.x, -self.y)

    def to_direction(self) -> Vec2:
        x = 0
        y = 0

        if self.x > 0:
            x = 1
        elif self.x < 0:
            x = -1

        if self.y > 0:
            y = 1
        elif self.y < 0:
            y = -1

        return Vec2(x, y)


class Line:
    def __init__(self, line: str):
        coords = [[int(s) for s in coord.split(",")]
                  for coord in line.strip().split(" -> ")]
        self.start = Vec2(coords[0][0], coords[0][1])
        self.end = Vec2(coords[1][0], coords[1][1])

    def plot(self) -> List[Vec2]:
        direction = (self.start - self.end).to_direction().invert()

        points = []
        point = self.start
        while True:
            # print(f"loop {point} + {direction}, {self}")
            points.append(point)
            if point == self.end:
                break
            point += direction

        return points

    def __repr__(self) -> str:
        return f"{self.start} -> {self.end}"


class Field:
    def __init__(self):
        self.field: Dict[Vec2, int] = {}

    def plot(self, line: Line):
        points = line.plot()
        for point in points:
            if point in self.field:
                self.field[point] += 1
            else:
                self.field[point] = 1

    def overlaps(self) -> int:
        return len([k for k in self.field if self.field[k] > 1])


def run():
    field = Field()
    with open('input.txt') as input:
        for instr in input:
            line = Line(instr)
            field.plot(line)

    print(f"Overlaps: {field.overlaps()}")


run()
