namespace Methods;

class Program
{
    static void Main(string[] args)
    {
        StaticMethod();

        // You can only call a non-static method by creating an instance of the class.
        Program programInstance = new Program();
        programInstance.Method();

        // This also works.
        new Program().Method();
    }

    void Method()
    {
        Console.WriteLine("This is a non-static method.");
    }

    static void StaticMethod()
    {
        Console.WriteLine("This is a static method.");
    }
}