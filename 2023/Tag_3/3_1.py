import os
import re

current_dir = os.path.dirname(os.path.abspath(__file__))
input_path = os.path.join(current_dir, '..', 'Tag_3', 'input.txt')
input = open(input_path, 'r')
result = 0

firstValue = input.readline()
secondValue = input.readline()
thirdValue = input.readline()

def findSymbol(substring):
    for i in range(len(substring)):
        if substring[i] != "." and not substring[i].isdigit():
            return True
    return False


numbers = re.findall(r'\d+', firstValue)
indices = [m.start() for m in re.finditer(r'\d+', firstValue)]

for i in range(len(numbers)):
    first = indices[i]
    last = indices[i] + len(numbers[i]) - 1

    if(indices[i] - 1 >= 0):
        first -= 1

    if(last + 1 < len(secondValue) - 1):
        last += 1

    if(findSymbol(firstValue[first])):
        result += int(numbers[i])
    elif(findSymbol(firstValue[last])):
        result += int(numbers[i])
    else:
        if(findSymbol(secondValue[first:last+1])):
            result += int(numbers[i])


while (thirdValue != ""):

    numbers = re.findall(r'\d+', secondValue)
    indices = [m.start() for m in re.finditer(r'\d+', secondValue)]

    for i in range(len(numbers)):
        first = indices[i]
        last = indices[i] + len(numbers[i]) - 1

        if(indices[i] - 1 >= 0):
            first -= 1

        if(last + 1 < len(secondValue) - 1):
            last += 1

        if(findSymbol(secondValue[first])):
            result += int(numbers[i])
        elif(findSymbol(secondValue[last])):
            result += int(numbers[i])
        elif(findSymbol(thirdValue[first:last+1]) or findSymbol(firstValue[first:last+1])):
            result += int(numbers[i])


    firstValue = secondValue
    secondValue = thirdValue
    thirdValue = input.readline()           

numbers = re.findall(r'\d+', secondValue)
indices = [m.start() for m in re.finditer(r'\d+', secondValue)]

for i in range(len(numbers)):
    first = indices[i]
    last = indices[i] + len(numbers[i]) - 1

    if(indices[i] - 1 >= 0):
        first -= 1

    if(last + 1 < len(secondValue) - 1):
        last += 1

    if(findSymbol(secondValue[first])):
        result += int(numbers[i])
    elif(findSymbol(secondValue[last])):
        result += int(numbers[i])
    else:
        if(findSymbol(firstValue[first:last+1])):
            result += int(numbers[i])


print(result)

input.close()