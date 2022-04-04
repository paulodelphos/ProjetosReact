using System;

class Program {

    static void main() {

        float metros = 0.0;
        float cent, mili;

        Console.Write("Digite um valor em metros: ");
        metros = float.Parse(Console.ReadLine());

        cent = metros * 100;
        mili = metros * 1000;

        Console.WriteLine(metros + " metros sao " + cent + " centimentos e " + mili + " milimetros.");

    }

}