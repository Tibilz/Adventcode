input = open('H:/Projekte/Adventcode/2023/Tag_1/input.txt', 'r')
value = input.readline()
result = 0
res1 = ""
res2 = ""
firstValue = True

while (value != ""):

    for i in value:
        if (firstValue and i.isdigit()):
            res1 = i
            res2 = i
            firstValue = False
        elif(i.isdigit()):
            res2 = i
    result += int(res1+res2)

    firstValue = True
    value = input.readline()

print(result)


input.close()
