using System;

class Program {
    static void Main() {
        double largura, altura, area, tinta;

        Console.WriteLine("Digite a largura da parede: ");
        largura = double.Parse(Console.ReadLine());
        
        Console.WriteLine("Digite a altura da parede: ");
        altura = double.Parse(Console.ReadLine());

        area = altura * largura;
        tinta = area/2;

        Console.WriteLine("A area eh igual a: " + area +
                         ".\nSao necessarios " + tinta + " litros de tinta para pintar."
                            
        );       
    }
}