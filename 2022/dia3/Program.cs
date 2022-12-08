// See https://aka.ms/new-console-template for more information
class Rucksack
{
    public static void Main()
    {
        string[] lines = File.ReadAllLines("./input.txt");

        int totalSum = 0;

        // part1
        foreach (string line in lines)
        {
            string part1 = line.Substring(0, line.Length / 2);
            string part2 = line.Substring(line.Length / 2);

            char common = Rucksack.GetCommonCharacter(part1, part2);

            totalSum += Rucksack.GetValueOfCharacter(common);
        }
        Console.WriteLine(totalSum);

        //part2
        totalSum = 0;
        for (int i = 0; i < lines.Length; i += 3)
        {
            char commonChar = GetCommonCharacters(lines[i], lines[i + 1], lines[i + 2]);
            totalSum += Rucksack.GetValueOfCharacter(commonChar);
        }

        Console.WriteLine(totalSum);


    }

    private static char GetCommonCharacters(string v1, string v2, string v3)
    {
        foreach (char item in v1)
        {
            if (v2.Contains(item) && v3.Contains(item))
            {
                return item;
            }
        }

        return (char)0;
    }

    private static char GetCommonCharacter(string part1, string part2)
    {
        foreach (char character in part1)
        {
            if (part2.Contains(character))
            {
                return character;
            }
        }

        return 'a';
    }

    private static int GetValueOfCharacter(char character)
    {
        if (char.IsLower(character))
        {
            return (int)character - (int)'a' + 1;
        }
        else
        {
            return (int)character - (int)'A' + 1 + 26;
        }
    }
}
