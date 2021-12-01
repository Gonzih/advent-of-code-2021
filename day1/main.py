def first_part():
    prev = None
    counter = 0

    with open('input.txt') as input:
        for line in input:
            v = int(line)
            if prev and prev < v:
                counter += 1
                print(v)
            prev = v

    return counter


def second_part():
    numbers = []
    windows = []
    sums = []
    counter = 0

    with open('input.txt') as input:
        for line in input:
            v = int(line)
            numbers.append(v)

    for i in range(len(numbers)-2):
        window = [numbers[i], numbers[i+1], numbers[i+2]]
        windows.append(window)
        sums.append(sum(window))

    for i in range(len(sums)):
        if i > 0 and sums[i-1] < sums[i]:
            counter += 1

    print(f"# of numbers {len(numbers)}")
    print(f"# of windows {len(windows)}")
    print(f"# of sums {len(sums)}")
    print(f"Answer is {counter}")


answer1 = first_part()
answer2 = second_part()
print(f"First answer is {answer1}")
print(f"Second answer is {answer2}")
