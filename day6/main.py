from typing import Dict, List


def total(fish: Dict[int, int]) -> int:
    result = 0
    for k in fish:
        result += fish[k]
    return result


def new_fish() -> Dict[int, int]:
    fish = {8: 0}
    for i in range(8):
        fish[i] = 0
    return fish


def print_fish(fish: Dict[int, int]):
    out: List[str] = []
    for k in range(9):
        v = str(fish[k])
        out.append(f"{v.rjust(3)}")
    print("|".join(out))

# def set_or_init(fish: Dict[int, int], newv: int, init: v):


def count_fish(fish: Dict[int, int], days: int) -> int:
    for i in range(days):
        print_fish(fish)
        new_generation = new_fish()
        newborns = fish[0]
        for k in range(9):
            if k == 0:
                new_generation[6] = fish[k]
            else:
                new_generation[k-1] += fish[k]
        new_generation[8] += newborns
        fish = new_generation
    return total(fish)


def run():
    fish = new_fish()
    print_fish(fish)

    with open('input.txt') as input:
        for line in input:
            initial_fish = [int(s) for s in line.strip().split(',')]
            for age in initial_fish:
                fish[age] += 1

    d0 = total(fish)
    print(f"Starting with {d0} jellyfishes")
    assert(d0 == 300)

    d80 = count_fish(fish.copy(), 80)
    print(f"d80 {d80}")
    assert(d80 == 375482)

    d256 = count_fish(fish.copy(), 256)
    print(f"d256 {d256}")
    assert(d256 >= 375482)


run()
