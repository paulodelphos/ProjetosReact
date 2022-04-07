c = IO.gets("Comprimento do cateto oposto: ")
{co, _} = Float.parse(c)

adj = IO.gets("Comprimento do cateto adjcente: ")
{ca, _} = Float.parse(adj)


hypo = ((co * 2.0) + (ca * 2.0)) * 0.5

IO.puts("A hypotenusa eh: #{hypo}")
