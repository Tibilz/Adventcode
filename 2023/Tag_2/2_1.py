import os
import re

current_dir = os.path.dirname(os.path.abspath(__file__))
input_path = os.path.join(current_dir, '..', 'Tag_2', 'input.txt')
input = open(input_path, 'r')
value = input.readline()
print(value)
numbers = []
result = 0

maxWantedCubes = {
    "red": 12,
    "green": 13,
    "blue": 14
}


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

    splitted = value.split(":")
    subsets = splitted[1][1:].split("; ")
    subsets = [subset.split(", ") for subset in subsets]
    subsets = makeTuples(subsets)
    addTuples(subsets)

    if(foundCubes["red"] <= maxWantedCubes["red"] and foundCubes["green"] <= maxWantedCubes["green"] and foundCubes["blue"] <= maxWantedCubes["blue"]):
        id = splitted[0]
        numbers += re.findall(r'\d+', id)

    value = input.readline()
  
for i in range(len(numbers)):
    result += int(numbers[i])

print(result)

input.close()