import typing


class Instruction:
    def __init__(self, command: str):
        spl = command.split(" ", 1)
        self.direction = spl[0]
        self.value = int(spl[1])

    def __str__(self):
        return f"Instruction: {self.direction} -> {self.value}"


class Sub:
    def __init__(self):
        self.aim = 0
        self.depth = 0
        self.horizontal = 0

    def apply_first(self, inst: Instruction):
        match inst.direction:
            case "forward":
                self.horizontal += inst.value
            case "up":
                self.depth -= inst.value
            case "down":
                self.depth += inst.value

    def apply_second(self, inst: Instruction):
        match inst.direction:
            case "forward":
                self.horizontal += inst.value
                self.depth += self.aim * inst.value
            case "up":
                self.aim -= inst.value
            case "down":
                self.aim += inst.value

    def answer(self):
        return self.depth * self.horizontal


def run():
    print("Day 2")

    sub1 = Sub()
    sub2 = Sub()

    with open("input.txt") as input:
        for line in input:
            inst = Instruction(line.strip())
            sub1.apply_first(inst)
            sub2.apply_second(inst)

    print(f"First answer: {sub1.answer()}")
    print(f"Second answer: {sub2.answer()}")


run()
