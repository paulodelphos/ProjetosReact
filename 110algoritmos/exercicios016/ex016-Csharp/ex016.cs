using System;

class Program {
    static void Main(){
        double num, truncado;

        Console.Write("Digite um numero frazionado: ");
        num = double.Parse(Console.ReadLine());

        truncado = Math.Truncate(num);
        Console.Write("Valor: " + num + " truncado é: " + truncado);

        
    }
}