import time

testInput = "supersizemeBucket111111.033dogsandcatsitembarked"
#testInput = "supersizemetobeornottobethatisthequestion"
dictionaryPath = "../words.txt"

def load_words(wordPath: str):
    wordFile = open(wordPath)
    words = {""}
    lines = wordFile.readlines()
    for line in lines:
        words.add(line.strip())

    return words

def is_float(element: str) -> bool:
    if element is None: 
        return False
    try:
        float(element)
        return True
    except ValueError:
        return False


def find_words(input: str, words):

    potentialWord = str(input)
    remainder = ""
    foundWords = []

    while len(potentialWord) > 0:
        #print("Testing: " + potentialWord)
        if potentialWord.lower() in words or is_float(potentialWord):
            #print("Found Word: " + potentialWord)
            foundWords.append(potentialWord)
            potentialWord = remainder
            remainder = ""
        else:
            #print("Not a word")
            remainder = potentialWord[-1:] + remainder
            potentialWord = potentialWord[0:-1]
            
            #print("Potential Word: " + potentialWord)
            #print("Remainder: " + remainder)
        #time.sleep(0.5)
    return foundWords

words = load_words(dictionaryPath)

result = find_words(testInput, words)

print(result)

    