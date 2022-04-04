n1 = IO.gets("Digite a primeira nota: ")
{num1, _} = Float.parse(n1)

n2 = IO.gets("Digite a segunda nota: ")
{num2, _} = Float.parse(n2)

media = (num1 + num2) / 2

IO.puts("A MEDIA entre #{num1} e #{num2} eh igual a #{media}")
