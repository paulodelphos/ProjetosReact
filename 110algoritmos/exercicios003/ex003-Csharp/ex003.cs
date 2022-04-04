using System;

class Program {
    static void Main(){
        float num1, num2, total;

        Console.Write("Digite o primeiro valor: ");
        num1 = int.Parse(Console.ReadLine());
        Console.Write("Digite o segundo valor");
        num2 = int.Parse(Console.ReadLine());
        total = num1 + num2;
     //   Console.WriteLine("A soma de "+ num1 + " + " + num2 + " = " + total);
        Console.WriteLine($"A soma de {num1} + {num2} = {total}");
    }
}