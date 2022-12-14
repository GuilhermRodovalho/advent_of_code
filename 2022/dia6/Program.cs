
class TuningTrouble
{
    public static void Main()
    {
        string[] lines = File.ReadAllLines("./input.txt");

        foreach (string line in lines)
        {
            int result = GetFirstNonSubsequentCharacter(line);

            Console.WriteLine($"Result = {result}");

        }
    }

    private static int GetFirstNonSubsequentCharacter(string input)
    {
        Dictionary<char, int> amount = new Dictionary<char, int>();
        int i = 0;
        int j = 0;
        while (i <= input.Length)
        {
            // Change this condition for part1 and two
            if (i < 14)
            {
                if (!amount.TryAdd(input[i], 1))
                {
                    amount[input[i]]++;
                }
            }
            else
            {
                if (!amount.TryAdd(input[i], 1))
                {
                    amount[input[i]]++;
                }
                amount[input[j]]--;

                bool hasBiggerThanOne = false;
                foreach (KeyValuePair<char, int> pair in amount)
                {
                    if (pair.Value > 1)
                    {
                        hasBiggerThanOne = true;
                    }
                }


                if (!hasBiggerThanOne)
                    return i + 1;

                j++;
            }
            i++;
        }

        return -1;
    }
}
