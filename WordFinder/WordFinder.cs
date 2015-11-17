using System;
using System.Collections.Generic;
using System.IO;

namespace WordFinder
{
    class WordFinder
    {
        static void Main(string[] args)
        {
            var dictionary = LoadWords("..\\..\\words.txt");
            var input = "supersizemeBucket111111.033dogsandcatsitembarked";

            var foundWords = FindWords(input, dictionary);

            foreach (var foundWord in foundWords)
            {
                Console.WriteLine(foundWord);
            }

            Console.ReadLine();
        }

        public static HashSet<string> LoadWords(string wordFile)
        {
            var wordSet = new HashSet<string>();

            using (var reader = new StreamReader(wordFile))
            {
                while (!reader.EndOfStream)
                {
                    wordSet.Add(reader.ReadLine());
                }
            }

            return wordSet;
        }

        public static ICollection<string> FindWords(string input, HashSet<string> dictionary)
        {
            var potentialWord = input;
            var remainder = string.Empty;
            var foundWords = new List<string>();

            decimal potentialNumber = 0.0m;

            while (!string.IsNullOrWhiteSpace(potentialWord))
            {
                if (dictionary.Contains(potentialWord.ToLowerInvariant()) || Decimal.TryParse(potentialWord, out potentialNumber))
                {
                    foundWords.Add(potentialWord);
                    potentialWord = remainder;
                    remainder = string.Empty;
                }
                else
                {
                    remainder = potentialWord.Substring(potentialWord.Length - 1, 1) + remainder;
                    potentialWord = potentialWord.Substring(0, potentialWord.Length - 1);
                }
            }
            return foundWords;
        }
    }
}
