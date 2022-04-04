n1 = IO.gets("Digite um numero inteiro: ")
{n2, _} = Integer.parse(n1)
n3 = n2+1
n4 = n2-1

IO.puts("O antecesor de #{n1} eh #{n4} e seu sucessor eh #{n3}")
