input = open('H:/Projekte/Adventcode/2023/Tag_1/input.txt', 'r')
value = input.readline()
result = 0
res1 = ""
res2 = ""
firstValue = True


stringDigit = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9
}

def find_all(a_str, sub):
    start = 0
    while True:
        start = a_str.find(sub, start)
        if start == -1: return
        yield start
        start += len(sub)

def rreplace(s, old, new, occurrence):
    li = s.rsplit(old, occurrence)
    return new.join(li)

oldvalue = ""
while (value != ""):
    #print(value)
    otherValue = value
    seenStringDigits = list(map(lambda elem1: list(map(lambda elem2: (elem2, elem1), list(find_all(value, elem1)))), list(stringDigit.keys())))
    flattened_list = [item for sublist in seenStringDigits for item in sublist]
    sorted(flattened_list)
    if(flattened_list):
        value = value.replace(min(flattened_list)[1], str(stringDigit[min(flattened_list)[1]]), 1)
        otherValue = rreplace(otherValue, max(flattened_list)[1], str(stringDigit[max(flattened_list)[1]]), 1)
    #print(oldvalue + " " + value)
    for i in value:
        if (i.isdigit()):
            res1 = i
            break
    
    for i in otherValue[::-1]:
        if(i.isdigit()):
            res2 = i
            break



    result += int(res1+res2)

    firstValue = True
    value = input.readline()
    
print(result)

input.close()

"""
testStringArray = ["nineight", "eightwothree", "acbone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"]

for pos, i in enumerate(testString):
    if (firstValue and i.isdigit()):
        res1 = i
        res2 = i
        firstValue = False
    elif(i.isdigit()):
        res2 = i



#testString.find(stringDigit)

result = 0
for testString in testStringArray:
    oldvalue = testString
    seenStringDigits = list(map(lambda elem1: list(map(lambda elem2: (elem2, elem1), list(find_all(testString, elem1)))), list(stringDigit.keys())))
    flattened_list = [item for sublist in seenStringDigits for item in sublist]
    sorted(flattened_list)
    if(flattened_list):
        testString = testString.replace(min(flattened_list)[1], str(stringDigit[min(flattened_list)[1]]), 1)
        testString = rreplace(testString, max(flattened_list)[1], str(stringDigit[max(flattened_list)[1]]), 1)
    print(oldvalue + " " + testString)
    for i in testString:
        if (firstValue and i.isdigit()):
            res1 = i
            res2 = i
            firstValue = False
        elif(i.isdigit()):
            res2 = i
    
    result += int(res1+res2)

    firstValue = True
    
print(result)

for testString in testStringArray:
    for key in stringDigit:
        testString = testString.replace(key, str(stringDigit[key]))
    print(testString)

iteration = list(filter(lambda element: element[0] != -1, list(map(lambda element: (testStringArray[2].find(element), element), list(stringDigit.keys())))))

    for key in flattened_list: #converts stringDigits from actual line in digits
        value = value.replace(key[1], str(stringDigit[key[1]]))

while(iteration):
    for iter in iteration:
        print(iter[0])
        testStringArray[2] = testStringArray[2].replace(iter[0], str(stringDigit[iter[0]]),1)
    iteration = list(filter(lambda element: element[0] != -1, list(map(lambda element: (testStringArray[2].find(element), element), list(stringDigit.keys())))))
#for testString in iteration:
 #   testString = testString.replace(key, str(stringDigit[key]), 1)

  #  print(testString)
print(testStringArray[2])
print(iteration)
print(list(stringDigit.keys()))
#print(testString.find(list(stringDigit.keys())))

for iter in iteration:
    print(iter[0])
    #testStringArray[2] = testStringArray[2].replace(iter[1], str(stringDigit[iter[1]]),1)
    iteration = list(filter(lambda element: element[0] != -1, list(map(lambda element: (testStringArray[2].find(element), element), list(stringDigit.keys())))))

def find_all(a_str, sub):
    start = 0
    while True:
        start = a_str.find(sub, start)
        if start == -1: return
        yield start
        start += len(sub)

print(list(find_all(testStringArray[2], 'three')))
print(list(stringDigit.keys()))
#print(list(find_all(testStringArray[2], elem1)))

iteration2 = list(map(lambda elem1: 
                            list(map(
                                lambda elem2: 
                                    (elem2, elem1)
                                    , list(find_all(testStringArray[2], elem1))
                            ))
                        , list(stringDigit.keys())
                      ))
                
print(iteration2)
flattened_list = [item for sublist in iteration2 for item in sublist]
print(sorted(flattened_list))

test = [(3, "cool") , (1, "yeah")]
print(min(sorted(test)))"""