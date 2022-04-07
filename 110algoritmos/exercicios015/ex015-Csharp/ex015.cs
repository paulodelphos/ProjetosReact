using System;

class Program {
    static void Main(){
       double  dias, km, total;

       Console.WriteLine("Quantos dias alugados? ");
       dias = float.Parse(Console.ReadLine());

       Console.WriteLine("Quantos km rodados? ");
       km = float.Parse(Console.ReadLine());

       total = (dias * 60 ) + (km * 0.15);
       Console.WriteLine("O total a pagar é R$ " + total);
    }
}