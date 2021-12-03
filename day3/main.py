import typing


def to_dec(binstr: str) -> int:
    return int(f"0b{binstr}", 2)


class Report:
    def __init__(self):
        self.gamma_rate = 0
        self.epsilon_rate = 0
        self.ones = []
        self.zeros = []
        self.allocated = False
        self.len = 0

    def allocate(self, n):
        for _ in range(n):
            self.ones.append(0)
            self.zeros.append(0)
        self.allocated = True
        self.len = n

    def process_line(self, line: str):
        if not self.allocated:
            self.allocate(len(line))

        for i, c in enumerate(line):
            if c == '0':
                self.zeros[i] += 1
            elif c == '1':
                self.ones[i] += 1

    def calculate(self):
        gamma = []
        epsilon = []
        for i in range(self.len):
            most_common = ''
            least_common = ''

            if self.ones[i] > self.zeros[i]:
                most_common = '1'
                least_common = '0'
            else:
                most_common = '0'
                least_common = '1'

            gamma.append(most_common)
            epsilon.append(least_common)

        self.gamma_rate = to_dec("".join(gamma))
        self.epsilon_rate = to_dec("".join(epsilon))

        return True

    def answer(self) -> int:
        self.calculate()
        return self.gamma_rate * self.epsilon_rate


def run():
    report = Report()

    with open('input.txt') as input:
        for line in input:
            binstr = line.strip()
            print(binstr)
            report.process_line(binstr)

    print(f"First answer {report.answer()}")


run()
