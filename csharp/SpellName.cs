namespace SpellName;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("What's your name?");
        string? name = Console.ReadLine();
        name?.Trim();

        if (!string.IsNullOrEmpty(name))
        {
            foreach (char letter in name)
            {
                Console.WriteLine(letter);
            }

            Console.WriteLine($"There you go, {name}!");
        }
        else
        {
            Console.WriteLine("You didn't enter a name!");
        }
    }
}
