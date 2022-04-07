l = IO.gets("Digite a largura da parede: ")
{largura, _} = Float.parse(l)

a = IO.gets("Digite a altura da parede: ")
{altura, _} = Float.parse(a)

area = altura * largura
tinta = area / 2

IO.puts("\nA parede tem dimensoes #{largura} x #{altura}")
IO.puts("A parede tem area de #{area} metros quadrados")
IO.puts("Sao necessario #{tinta} litros de tinta para pinta-la.\n")
