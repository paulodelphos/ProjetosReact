n = IO.gets("Digite o numero da tabuada: ")
{num, _} = Integer.parse(n)

#Enum.each(1..10, IO.puts("") )


for x <- 1..10, do:  IO.puts("#{num} x #{x} = #{x*num}")
