from __future__ import annotations
from typing import Optional

RED = '\033[0;31m'
NC = '\033[0m'


def red_color(s: str) -> str:
    return f"{RED}{s}{NC}"


OPENING = list("([{<")
CLOSING = list(")]}>")


def chunkers() -> dict[str, str]:
    chunks: dict[str, str] = {}
    for i in range(len(OPENING)):
        chunks[OPENING[i]] = CLOSING[i]
        chunks[CLOSING[i]] = OPENING[i]
    return chunks


class Chunk:
    def __init__(self, line: str):
        self.line = line.strip()
        self.state: list[str] = []
        self.error_char: Optional[str] = None
        self.error_idx: Optional[int] = None

    def parse(self) -> Optional[str]:
        chunk_mapping = chunkers()
        for i, c in enumerate(list(self.line)):
            if c in OPENING:
                self.state.append(c)
            elif c in CLOSING:
                if self.state[-1] == chunk_mapping[c]:
                    self.state.pop()
                else:
                    # print(f"Error {c} at {i}, state: {self.state}")
                    self.error_char = c
                    self.error_idx = i
                    return c
        return None

    def __repr__(self) -> str:
        out = ''
        for i, c in enumerate(list(self.line)):
            cc = c
            if i == self.error_idx:
                cc = red_color(c)
            out += cc
        return f"{out} -> {self.error_idx}:{self.error_char}"


ERROR_SCORE_TABLE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

COMPLETE_SCORE_TABLE = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}


def run(fname) -> tuple[int, int]:
    incomplete_lines = []
    errors = []
    with open(fname) as input:
        for line in input:
            chunk = Chunk(line)
            res = chunk.parse()
            print(chunk)
            if res is not None:
                errors.append(res)
            else:
                incomplete_lines.append(chunk)
    escore = 0
    for e in errors:
        escore += ERROR_SCORE_TABLE[e]

    cscore = 0
    cscores: list[int] = []
    chunk_mapping = chunkers()
    for chunk in incomplete_lines:
        _score = 0
        while len(chunk.state) > 0:
            _score *= 5
            opener = chunk.state.pop()
            closer = chunk_mapping[opener]
            _score += COMPLETE_SCORE_TABLE[closer]
        cscores.append(_score)
    cscores.sort(reverse=True)
    cscore = cscores[len(cscores)//2]

    print("")
    print(errors)
    print(escore)

    print(cscores)
    print(cscore)
    return (escore, cscore)


tescore, tcscore = run('test-input.txt')
assert(tescore == 26397)
assert(tcscore == 288957)

print("")

escore, cscore = run('input.txt')
assert(escore == 240123)
assert(cscore == 3260812321)
