from typing import List, Set

BOARD_SIZE = 5
GREEN = '\033[0;32m'
NC = '\033[0m'


def green_color(s: str) -> str:
    return f"{GREEN}{s}{NC}"


class Board:
    def __init__(self):
        self.numbers: Set[int] = set()
        self.print_rows: List[List[int]] = []
        self.rows: List[Set[int]] = []
        self.cols: List[Set[int]] = []

        for _ in range(BOARD_SIZE):
            self.print_rows.append([])
            self.rows.append(set())
            self.cols.append(set())

    def add_row(self, i: int, row_list: List[int]):
        self.print_rows[i] = row_list

        row = set(row_list)
        self.rows[i] = row

        for j, n in enumerate(row_list):
            self.cols[j].add(n)

    def __display_n__(self, n: int) -> str:
        out = str(n)
        if n in self.numbers:
            return green_color(out)
        return out

    def __repr__(self) -> str:
        return "\n".join(
            [" ".join([self.__display_n__(n) for n in row])
             for row in self.print_rows])

    def set_numbers(self, numbers: Set[int]):
        self.numbers = numbers.copy()

    def is_winner(self):
        for i in range(BOARD_SIZE):
            row_match = self.rows[i].issubset(self.numbers)
            col_match = self.cols[i].issubset(self.numbers)
            if row_match or col_match:
                return True
        return False

    def unmarked_sum(self) -> int:
        result = 0

        for row in self.rows:
            for n in row:
                if not n in self.numbers:
                    result += n

        return result


class Bingo:
    def __init__(self, numbers: List[int]):
        self.boards: List[Board] = []
        self.numbers: List[int] = numbers

    def add_board(self, board: Board):
        self.boards.append(board)

    def print_board(self, board: Board, n: int):
        print(board)
        print(f"\nfor set {board.numbers}\nwinning number: {n}")
        score = board.unmarked_sum() * n
        print(f"\nFinal score: {score}\n")

    def play(self):
        numbers_set = set()
        first_board = None
        first_number = None
        last_board = None
        last_number = None

        for n in self.numbers:
            numbers_set.add(n)
            for board in self.boards:
                if not board.is_winner():
                    board.set_numbers(numbers_set)

                if not last_board and\
                        all(board.is_winner() for board in self.boards):
                    last_board = board
                    last_number = n

            if not first_board:
                winners = list(
                    filter(lambda board: board.is_winner(), self.boards))
                if len(winners) == 1:
                    first_board = winners[0]
                    first_number = n

        print("First winning board:\n")
        self.print_board(first_board, first_number)

        print("Last winning board:\n")
        self.print_board(last_board, last_number)


def run():
    lines = []
    with open('input.txt') as input:
        lines = [line.strip() for line in input]

    numbers = [int(number) for number in lines[0].split(",")]

    bingo = Bingo(numbers)
    board = Board()
    line_number = 0
    for line in lines[2:]:
        if len(line) == 0:
            bingo.add_board(board)
            board = Board()
            line_number = 0
            continue

        row = [int(number) for number in line.split(" ") if len(number) > 0]
        board.add_row(line_number, row)
        line_number += 1

    bingo.play()


run()
