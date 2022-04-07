using System;

class Program {
    static void Main() {
        int num;

        Console.Write("Digite um número para tabuada: ");
        num = int.Parse(Console.ReadLine());

        Console.WriteLine("=-=-=-=-=-=");
        for(int i = 1; i<=10; i++)
        {
            Console.WriteLine(num + " x " + i + " = " + (num * i));
        }
        Console.WriteLine("=-=-=-=-=-=");


    }
}