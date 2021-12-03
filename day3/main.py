from typing import List


def to_dec(binstr: str) -> int:
    return int(f"0b{binstr}", 2)


def join_bits(bits: List[str]) -> str:
    return "".join(bits)


class Dataset:
    def __init__(self, input: List[str]):
        self.input = input
        self.ones = []
        self.zeros = []
        self.most_commons = []
        self.least_commons = []
        self.allocate()
        self.process()
        self.calculate()

    def allocate(self):
        for _ in range(self.line_len()):
            self.ones.append(0)
            self.zeros.append(0)

    def process(self):
        for line in self.input:
            self.process_line(line)

    def line_len(self):
        if len(self.input) == 0:
            return 0
        else:
            return len(self.input[0])

    def len(self):
        return len(self.input)

    def process_line(self, line: str):
        for i, c in enumerate(line):
            if c == '0':
                self.zeros[i] += 1
            elif c == '1':
                self.ones[i] += 1

    def calculate(self):
        for i in range(self.line_len()):
            most_common = ''
            least_common = ''

            if self.ones[i] >= self.zeros[i]:
                most_common = '1'
                least_common = '0'
            else:
                most_common = '0'
                least_common = '1'

            self.most_commons.append(most_common)
            self.least_commons.append(least_common)

    def filter_by_pos(self, lines: List[str], pos: int, char: str):
        return Dataset(
            list(
                filter(
                    lambda line: line[pos] == char,
                    self.input)))


class Report:
    def __init__(self):
        self.gamma_rate = 0
        self.epsilon_rate = 0
        self.oxygen_rating = 0
        self.co2_rating = 0
        self.dataset = Dataset([])
        self.allocated = False

    def process_input(self, lines: List[str]):
        self.dataset = Dataset(lines)

    def calutate_power_consumption(self):
        self.gamma_rate = to_dec(join_bits(self.dataset.most_commons))
        self.epsilon_rate = to_dec(join_bits(self.dataset.least_commons))

    def calculate_life_support_rating(self):
        oxygen_ratings = self.dataset
        co2_ratings = self.dataset

        for i in range(self.dataset.line_len()):
            most = oxygen_ratings.most_commons[i]
            least = co2_ratings.least_commons[i]

            if oxygen_ratings.len() > 1:
                oxygen_ratings = oxygen_ratings.\
                    filter_by_pos(oxygen_ratings, i, most)
            if co2_ratings.len() > 1:
                co2_ratings = co2_ratings.\
                    filter_by_pos(co2_ratings, i, least)

        self.oxygen_rating = to_dec(join_bits(oxygen_ratings.input[0]))
        self.co2_rating = to_dec(join_bits(co2_ratings.input[0]))

    def calculate(self):
        self.calutate_power_consumption()
        self.calculate_life_support_rating()

    def first_answer(self) -> int:
        return self.gamma_rate * self.epsilon_rate

    def second_answer(self) -> int:
        return self.oxygen_rating * self.co2_rating


def run():
    report = Report()

    with open('input.txt') as input:
        lines = [line.strip() for line in input]
        report.process_input(lines)

    report.calculate()
    print(f"First answer {report.first_answer()}")
    print(f"Second answer {report.second_answer()}")


run()
