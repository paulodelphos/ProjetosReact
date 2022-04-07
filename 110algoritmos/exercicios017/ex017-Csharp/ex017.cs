using System;

class Program {
    static void Main(){
        double ca, co, hypo;

        Console.WriteLine("Digite o cateto adjacente: ");
        ca = double.Parse(Console.ReadLine());

        Console.WriteLine("Digite o cateto oposto: ");
        co = double.Parse(Console.ReadLine());

        hypo = Math.Sqrt(Math.Pow(ca, 2.0) + Math.Pow(co, 2.0));

        Console.WriteLine("A hypotenusa é: " + hypo);
    }
}