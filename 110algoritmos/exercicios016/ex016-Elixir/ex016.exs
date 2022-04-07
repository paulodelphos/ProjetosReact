n = IO.gets("Digite um valor fracionado: ")
{num, _} = Float.parse(n)
truncado = trunc(num)

IO.puts("O valor #{num} truncado é igual a #{truncado}")
