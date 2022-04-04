using System;

class Program {
    static void Main() {
        int num, dobro, triplo;
        double raiz;

        Console.Write("Digite um número: ");
        num = int.Parse(Console.ReadLine());

        dobro = num * 2;
        triplo = num * 3;
        raiz = Math.Pow(num, 0.5);

        Console.WriteLine("Do dobro de " + num + " é " + dobro);
        Console.WriteLine("Do triplo é " + triplo);
        Console.WriteLine("A raiz é " + raiz.ToString("f"));

    }
}