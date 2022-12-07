// See https://aka.ms/new-console-template for more information
string[] lines = File.ReadAllLines("./finalInput.txt");

// int topElf = 0;
// int thisElf = 0;

// // Part 1
// foreach (string line in lines)
// {
//     if (line != "")
//     {
//         thisElf += int.Parse(line);
//         continue;
//     }
//     if (thisElf > topElf)
//     {
//         topElf = thisElf;
//     }
//     thisElf = 0;
// }


// part 2
bool CheckIfElfIsTop(int thisElf, int[] topElves)
{
    foreach (int elf in topElves)
    {
        if (thisElf > elf)
        {
            return true;
        }
    }

    return false;
}

void PutElfInTopElves(int thisElf, int[] topElves)
{
    int poorestElf = 0;
    for (int i = 0; i < topElves.Length; i++)
    {
        if (topElves[i] < topElves[poorestElf])
        {
            poorestElf = i;
        }
    }

    topElves[poorestElf] = thisElf;
}

int thisElf = 0;
int[] topElves = { 0, 0, 0 };
foreach (string line in lines)
{
    if (line != "")
    {
        thisElf += int.Parse(line);
        continue;
    }
    if (CheckIfElfIsTop(thisElf, topElves))
    {
        PutElfInTopElves(thisElf, topElves);
    }
    thisElf = 0;
}

Console.WriteLine(topElves[0] + topElves[1] + topElves[2]);
