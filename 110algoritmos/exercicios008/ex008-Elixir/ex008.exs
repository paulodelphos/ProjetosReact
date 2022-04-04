m = IO.gets("Digite quantos metros: ");
{metros, _} = Integer.parse(m)

centi = metros * 100
mili = metros * 1000

IO.puts("#{metros} metros sao #{centi} centimetros e #{mili} milimetros.")
