n = IO.gets("Digite um numero: ")
{num, _} = Float.parse(n)
dobro = num * 2.0
triplo = num * 3.0
raiz = num ** 0.5

IO.puts("O dobro de #{num} eh #{dobro}. \nO triplo eh #{triplo} \nA raiz eh #{Float.round(raiz)}")
