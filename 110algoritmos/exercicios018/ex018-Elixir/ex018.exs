require Math

ang = IO.gets("Digite o Angulo: ")
{angulo, _} = Float.parse(ang)

sen = (angulo * 3.1415)/180.0
seno = Math.sin(sen)

IO.puts("O angulo #{angulo} tem SENO de: #{seno}")
