from typing import List


def print_fish(fish: List[int]):
    print("|".join([(str(v).rjust(3)) for v in fish]))


def count_fish(fish: List[int], days: int) -> int:
    for i in range(days):
        print_fish(fish)
        newborns = fish.pop(0)
        fish.append(newborns)
        fish[6] += newborns
    return sum(fish)


def run():
    fish = [0 for _ in range(9)]
    print_fish(fish)

    with open('input.txt') as input:
        for line in input:
            init = [int(s) for s in line.strip().split(',')]
            for age in init:
                fish[age] += 1

    d0 = sum(fish)
    assert(d0 == 300)

    d80 = count_fish(fish.copy(), 80)
    print(f"d80 {d80}")
    assert(d80 == 375482)

    d256 = count_fish(fish.copy(), 256)
    print(f"d256 {d256}")
    assert(d256 == 1689540415957)


run()
