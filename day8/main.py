from __future__ import annotations
import typing

LEN_MAPPINGS = {
    2: 1,
    4: 4,
    3: 7,
    7: 8,
}


class Digit:
    def __init__(self, segment: str):
        self.segment = segment
        self.number = -1

    def set(self, n: int):
        self.number = n

    def len_specific(self) -> bool:
        return len(self.segment) in LEN_MAPPINGS

    def len(self) -> int:
        return len(self.segment)

    def to_dec(self) -> int:
        return self.number

    def __repr__(self) -> str:
        return f"{self.segment}({self.to_dec()})"

    def contains(self, other: Digit) -> bool:
        return all(s in self.segment for s in other.segment)

    def equal(self, other: Digit) -> bool:
        return self.contains(other) and self.len() == other.len()


class Line:
    def __init__(self, line: str):
        sections = line.strip().split(' | ')
        assert(len(sections) == 2)
        self.input = [Digit(segment)
                      for segment in sections[0].split(' ')]
        self.output = [Digit(segment) for segment in sections[1].split(' ')]
        self.determine_things()
        print(
            f"{' '.join(str(v) for v in self.input)} | {' '.join(str(v) for v in self.output)}")

    def determine_things(self):
        self.mappings = {}
        for digit in self.input:
            if digit.len_specific():
                v = LEN_MAPPINGS[digit.len()]
                self.mappings[v] = digit

        for digit in self.input:
            if digit.len() == 6:
                if not digit.contains(self.mappings[1]):
                    self.mappings[6] = digit
                elif digit.contains(self.mappings[4]):
                    self.mappings[9] = digit
                else:
                    self.mappings[0] = digit

        for digit in self.input:
            if digit.len() == 5:
                if digit.contains(self.mappings[1]):
                    self.mappings[3] = digit
                elif self.mappings[6].contains(digit):
                    self.mappings[5] = digit
                else:
                    self.mappings[2] = digit

        for k in self.mappings:
            self.mappings[k].set(k)
            for dig in self.output:
                if dig.equal(self.mappings[k]):
                    dig.set(k)

    def output_val(self) -> int:
        return int(''.join(str(dig.to_dec()) for dig in self.output))

    def number_of_simple_in_output(self):
        return sum(1 for v in self.output if v.len_specific())


def run():
    sumsum = 0
    count = 0
    with open('input.txt') as input:
        for s in input:
            line = Line(s)
            count += line.number_of_simple_in_output()
            sumsum += line.output_val()

    print(count)
    assert(count == 512)
    print(sumsum)


run()
