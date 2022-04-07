using System;

class Program {
    static void Main(){
        double valor, descontado;

        Console.Write("Qual é o preço do produto? R$");
        valor = double.Parse(Console.ReadLine());

        descontado = valor * 0.95;

        Console.WriteLine("O produto que custava " + valor + ", na promoção com desconto vai custar R$" + descontado );
    }
}