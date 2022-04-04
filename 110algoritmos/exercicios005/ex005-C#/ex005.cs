using System;

class Program {
    static void Main() {
        int n1;

        Console.Write("Digite um valor: ");
        n1 = int.Parse(Console.ReadLine());

        Console.WriteLine("Você digitou " + n1);
        Console.WriteLine($"Você digitou: {n1}");

    }
}