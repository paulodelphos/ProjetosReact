using System;

class Program {
        static void Main() {
        String nome;

        Console.Write("\nDigite o seu nome: ");
        nome = Console.ReadLine();
        Console.WriteLine("Muito prazer tem te conhecer " + nome);   
        Console.WriteLine($"Muito prazer tem te conhecer {nome}");   
    }
}