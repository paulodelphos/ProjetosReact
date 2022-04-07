using System;

class Program {
    static void Main() {
        int n1;

        Console.Write("Digite um valor: ");
        n1 = int.Parse(Console.ReadLine());

        Console.WriteLine("O sucessor de " + n1 + " é " + (n1 + 1) + " e o antecessor é " + (n1 - 1) + ".");
        

    }
}