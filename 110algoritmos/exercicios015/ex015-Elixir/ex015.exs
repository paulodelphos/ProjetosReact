d = IO.gets("Quantos dias alugados? ")
{dias, _} = Float.parse(d)

k = IO.gets("Quantos km rodados? ")
{km, _} = Float.parse(k)

total = (dias* 60.0) + (km * 0.15)

IO.puts("O total a pagar eh R$ #{total}")
