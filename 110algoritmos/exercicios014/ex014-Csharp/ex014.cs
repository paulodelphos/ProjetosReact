using System;

class Program{

    static void Main() {

        double c, f;

        Console.Write("Informe a temperatura em Celsius: ");
        c = double.Parse(Console.ReadLine());

        f = (9 * c)/5 + 32;

        Console.WriteLine("A temperatura em " + c + " C corresponde em " +f+ " F");

    }
}