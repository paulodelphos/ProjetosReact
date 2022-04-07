using System;

class Program {
    static void Main() {
        double real, dollar;

        Console.Write("Digite quantos reais tem na carteira R$: ");
        real = int.Parse(Console.ReadLine());

        dollar = real/4.64;

        Console.WriteLine("Com R$ " + real + " você pode comprar $ " + dollar.ToString("F") + "dollars");

    }
}