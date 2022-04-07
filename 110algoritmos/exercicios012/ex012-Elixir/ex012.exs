v = IO.gets("Qual eh o valor do produto? R$ ")
{valor, _} = Float.parse(v)
descontado = valor * 0.95

IO.puts("\nO produto que custava R$#{valor}, na promoção com desconto de 5% vai custar #{descontado}")
