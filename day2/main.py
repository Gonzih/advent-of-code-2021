prev = None
counter = 0

with open('input.txt') as input:
    for line in input:
        v = int(line)
        if prev and prev < v:
            counter += 1
            print(v)
        prev = v

print(f"Answer is {counter}")
