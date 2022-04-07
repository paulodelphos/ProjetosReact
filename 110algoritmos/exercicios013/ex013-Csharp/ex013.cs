using System;

class Program {
    static void Main() {
        double salario, aumento;

        Console.Write("Qual é o salário do funcionário? R$");
        salario = double.Parse(Console.ReadLine());

        aumento = salario * 1.15;

        Console.WriteLine("Um funcionario que ganharava R$" + salario + " com 15% de aumento, passa a receber R$" + aumento.ToString(2) + ".\n");

    }
}