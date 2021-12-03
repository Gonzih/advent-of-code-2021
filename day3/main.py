from typing import List


def to_dec(binstr: str) -> int:
    return int(f"0b{binstr}", 2)


def filter_by_pos(lines: List[str], pos: int, char: str):
    return list(filter(lambda line: line[pos] == char, lines))


class Report:
    def __init__(self):
        self.gamma_rate = 0
        self.epsilon_rate = 0
        self.oxygen_rating = 0
        self.co2_rating = 0
        self.lines = []
        self.ones = []
        self.zeros = []
        self.most_commons = []
        self.least_commons = []
        self.allocated = False
        self.len = 0

    def allocate(self, n):
        for _ in range(n):
            self.ones.append(0)
            self.zeros.append(0)
        self.allocated = True
        self.len = n

    def process_line(self, line: str):
        self.lines.append(line)

        if not self.allocated:
            self.allocate(len(line))

        for i, c in enumerate(line):
            if c == '0':
                self.zeros[i] += 1
            elif c == '1':
                self.ones[i] += 1

    def calutate_power_consumption(self):
        for i in range(self.len):
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

        self.gamma_rate = to_dec("".join(self.most_commons))
        self.epsilon_rate = to_dec("".join(self.least_commons))

    def calculate_life_support_rating(self):
        oxygen_ratings = self.lines
        co2_ratings = self.lines

        for i in range(self.len):
            most = self.most_commons[i]
            least = self.least_commons[i]

            if len(oxygen_ratings) > 1:
                oxygen_ratings = filter_by_pos(oxygen_ratings, i, most)
            if len(co2_ratings) > 1:
                co2_ratings = filter_by_pos(co2_ratings, i, least)

            print(
                f"Reduced oxy to {len(oxygen_ratings)}, co2 {len(co2_ratings)}")

        self.oxygen_rating = to_dec("".join(oxygen_ratings[0]))
        self.co2_rating = to_dec("".join(co2_ratings[0]))

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
        for line in input:
            binstr = line.strip()
            print(binstr)
            report.process_line(binstr)

    report.calculate()
    print(f"First answer {report.first_answer()}")
    print(f"Second answer {report.second_answer()}")


run()
