using System;

class Program {

    static void Main() {
        float n1, n2, media;

        Console.WriteLine("Digite a primeira nota: ");
        n1 = int.Parse(Console.ReadLine());
        
        Console.Write("Digite a segunda nota: ");
        n2 = int.Parse(Console.ReadLine());

        media = (n1 + n2)/2;

        Console.WriteLine("A média entre " + n1 + " e " + n2 + " eh igual a " + media);

    }
}

