c = IO.gets("Informe a temperatura em Celsius")
{celcius, _} = Float.parse(c)

f = (9 * celcius)/5 + 32

IO.puts("A temperatura em #{celcius} C corresponde em #{f} F")
