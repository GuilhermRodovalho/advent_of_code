// See https://aka.ms/new-console-template for more information

class RangeSections
{
    int start;
    int end;

    public RangeSections(string sections)
    {
        string[] numbers = sections.Split("-");
        this.start = int.Parse(numbers[0]);
        this.end = int.Parse(numbers[1]);
    }

    public bool FullyContains(RangeSections another)
    {
        if (this.start <= another.start && this.end >= another.end)
            return true;

        return false;
    }

    public bool PartiallyContains(RangeSections another)
    {
        if (another.end >= this.start && another.end <= this.end)
        {
            return true;
        }
        if (another.start >= this.start && another.end <= this.end)
        {
            return true;
        }

        return false;
    }

    public override string ToString()
    {
        return $"{this.start}-{this.end}";
    }
}


class CampCleanup
{
    public static void Main()
    {
        string[] lines = File.ReadAllLines("./input.txt");
        int resultado = 0;

        //part1
        // foreach (string line in lines)
        // {
        //     string[] pairs = line.Split(",");

        //     RangeSections[] rangePairs = new RangeSections[2];

        //     for (int i = 0; i < 2; i++)
        //     {
        //         rangePairs[i] = new RangeSections(pairs[i]);
        //     }

        //     if (
        //         rangePairs[0].FullyContains(rangePairs[1])
        //         || rangePairs[1].FullyContains(rangePairs[0])
        //         )
        //     {
        //         resultado++;

        //     }

        // }
        foreach (string line in lines)
        {
            string[] pairs = line.Split(",");

            RangeSections[] rangePairs = new RangeSections[2];

            for (int i = 0; i < 2; i++)
            {
                rangePairs[i] = new RangeSections(pairs[i]);
            }

            if (
                rangePairs[0].PartiallyContains(rangePairs[1])
                || rangePairs[1].PartiallyContains(rangePairs[0])
                )
            {
                resultado++;

            }

        }

        Console.WriteLine(resultado);
    }
}