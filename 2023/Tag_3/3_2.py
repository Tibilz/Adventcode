import os

current_dir = os.path.dirname(os.path.abspath(__file__))
input_path = os.path.join(current_dir, '..', 'Tag_3', 'input.txt')
input = open(input_path, 'r')
value = input.readline()
tempVal = 1
result = 0


def makeTuples(subsets):
    tuples = []
    for subset in subsets:
        for elem in subset:
            parts = elem.split()
            tuples.append((int(parts[0]), parts[1]))
    return tuples

def addTuples(subsets):
    for subset in subsets:
        if subset[0] > foundCubes[subset[1]]:
            foundCubes[subset[1]] = subset[0]


while (value != ""):
    foundCubes = {
        "red": 0,
        "green": 0,
        "blue": 0
    }
    tempVal = 1

    splitted = value.split(":")
    subsets = splitted[1][1:].split("; ")
    subsets = [subset.split(", ") for subset in subsets]
    subsets = makeTuples(subsets)
    addTuples(subsets)

    for color, count in foundCubes.items():
        tempVal *= count

    result += tempVal
    value = input.readline()

print(result)

input.close()