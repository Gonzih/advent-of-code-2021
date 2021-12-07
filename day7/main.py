from typing import List
import statistics


def min_distance_one(numbers: List[int]) -> int:
    avg = round(sum(numbers)/len(numbers))
    distances = [sum([abs(i-j) for j in numbers])
                 for i in range(avg)]
    return min(distances)


def min_distance_two(numbers: List[int]) -> int:
    avg = round(sum(numbers)/len(numbers))
    distances = [sum([sum(range(abs(i-j)+1)) for j in numbers])
                 for i in range(avg)]
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
