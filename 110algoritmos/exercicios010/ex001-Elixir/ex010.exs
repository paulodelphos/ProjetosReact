r = IO.gets("Quantos reais você tem na carteira? R$ ")
{reais, _} = Float.parse(r)
dollar = reais/4.61

IO.puts("\nCom R$ #{reais} reais voce pode comprar $ #{Float.round(dollar)} dollares.\n=-=-=-=-=-=")
