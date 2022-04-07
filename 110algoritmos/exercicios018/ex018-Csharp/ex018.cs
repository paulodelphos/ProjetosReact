using System;

class Program {
    static void Main() {
        double angulo, coseno, seno, tangente;

        Console.WriteLine("Digite o angulo:");
        angulo = double.Parse(Console.ReadLine());

        seno = Math.Sin((angulo * Math.PI) / 180.0);
        coseno = Math.Cos((angulo * Math.PI)/180.0);
        tangente = Math.Tan((angulo * Math.PI)/180.0);

        Console.WriteLine("O angulo " + angulo + " tem SENO: " + seno.ToString("F"));
        Console.WriteLine("O angulo " + angulo + " tem COSENO: " + coseno.ToString("F"));
        Console.WriteLine("O angulo " + angulo + " tem TANGENTE: " + tangente.ToString("F"));

    }
}