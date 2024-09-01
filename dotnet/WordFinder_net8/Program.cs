using System.Collections.Generic;
using System.IO;

const string testInput = "supersizemeBucket111111.033dogsandcatsitembarked";
const string dictionaryPath = @"..\..\words.txt";

var dictionary = LoadDictionary(dictionaryPath);

var foundWords = FindWords(testInput, dictionary);

Console.WriteLine("Found the following words and numbers.");

foreach (var word in foundWords)
{
    Console.WriteLine(word);
}

Console.ReadLine();

HashSet<string> LoadDictionary(string filePath)
{
    return new HashSet<string>(File.ReadAllLines(filePath));
}

ICollection<string> FindWords(string input, HashSet<string> dictionary)
{
    var potentialWord = input;
    var remainder = string.Empty;
    var foundWords = new List<string>();

    while (!string.IsNullOrWhiteSpace(potentialWord))
    {
        if (dictionary.Contains(potentialWord.ToLowerInvariant()) || decimal.TryParse(potentialWord, out var _potentialNumber))
        {
            foundWords.Add(potentialWord);
            potentialWord = remainder;
            remainder = string.Empty;
        }
        else 
        {
            remainder = potentialWord[^1] + remainder;
            potentialWord = potentialWord[0..^1];
        }
    }

    return foundWords;
}
