from typing import List


def min_distance_one(numbers: List[int]) -> int:
    max_n = max(numbers)
    distances = [sum([abs(i-j) for j in numbers]) for i in range(max_n)]
    return min(distances)


def min_distance_two(numbers: List[int]) -> int:
    max_n = max(numbers)
    distances = [sum([sum(range(abs(i-j))) for j in numbers])
                 for i in range(max_n)]
    return min(distances)


def run():
    numbers = []
    with open('input.txt') as input:
        for line in input:
            numbers = [int(s) for s in line.strip().split(',')]

    min_d = min_distance_one(numbers)
    print(f"Min  distance {min_d}")
    assert(min_d == 355521)

    min_d = min_distance_two(numbers)
    print(f"Min  distance {min_d}")
    assert(min_d == 100148777)


run()
