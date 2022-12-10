// See https://aka.ms/new-console-template for more information
class SupplyStacks
{
    public static void Main()
    {
        string allTheInput = File.ReadAllText("./input.txt");

        string[] parts = allTheInput.Split("\n\n");
        string boxes = parts[0];

        // I'm calling these stacks, but they're actually lists, so I'll be using
        // prepend to add to the "top" of the stack and remove to remove the element
        // at the top
        List<Char>[] stacks = SplitIntoStacks(boxes, 9);

        List<int[]> instructions = GetInstructions(parts[1]);

        // part1 
        // foreach (int[] instruction in instructions)
        // {
        //     DoInstuctionOneAtATime(instruction, stacks);
        // }

        // part2
        foreach (int[] instruction in instructions)
        {
            DoInstructionAllAtOnce(instruction, stacks);
        }

        //print the result after applying instructions
        foreach (List<Char> stack in stacks)
        {
            Console.Write(stack.First());
        }
    }

    /*
    this function basically executes each instruction on its respective stacks
    */
    private static void DoInstuctionOneAtATime(int[] instruction, List<char>[] stacks)
    {
        var rightStack = stacks.ElementAt(instruction[1] - 1);
        var destinyStack = stacks.ElementAt(instruction[2] - 1);

        for (int i = 0; i < instruction[0]; i++)
        {
            var element = rightStack.ElementAt(0);
            rightStack.RemoveAt(0);

            destinyStack.Insert(0, element);
        }
    }

    private static void DoInstructionAllAtOnce(int[] instruction, List<char>[] stacks)
    {
        var rightStack = stacks.ElementAt(instruction[1] - 1);
        var destinyStack = stacks.ElementAt(instruction[2] - 1);

        var elements = rightStack.Take(instruction[0]);

        destinyStack.InsertRange(0, elements);

        rightStack.RemoveRange(0, instruction[0]);
    }

    /*
    this function returns a list of array of integers, each position of the list
    contains an instruction, that is, 3 numbers saying how many to move, from 
    where and to where
    */
    private static List<int[]> GetInstructions(string allLines)
    {
        List<int[]> instructions = new List<int[]>();

        string[] lines = allLines.Split('\n');

        foreach (string line in lines)
        {
            int[] thisCorrectInstructions = new int[3];

            string[] splitedLine = line.Split(" ");

            thisCorrectInstructions[0] = int.Parse(splitedLine[1]);
            thisCorrectInstructions[1] = int.Parse(splitedLine[3]);
            thisCorrectInstructions[2] = int.Parse(splitedLine[5]);

            instructions.Add(thisCorrectInstructions);
        }

        return instructions;
    }

    /*
    This function creat an array of lists (I know, it's confusing, but makes sense)
    with each of one containing an stack
    */
    public static List<Char>[] SplitIntoStacks(string input, int stacksQuantity)
    {
        int currentStack = 0;
        List<Char>[] allStacks = new List<Char>[stacksQuantity];

        for (int i = 0; i < stacksQuantity; i++)
        {
            allStacks[i] = new List<char>();
        }

        // this will put every character on the correct list (still a list)
        for (int i = 1; i <= input.Length; i += 4)
        {
            if (input[i] != ' ' && Char.IsAsciiLetter(input[i]))
            {
                allStacks[currentStack].Add(input[i]);
            }

            currentStack += 1;
            currentStack = currentStack % stacksQuantity;
        }

        return allStacks;
    }
}
