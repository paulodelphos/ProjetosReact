sal = IO.gets("Qual eh o valor do salario? R$")
{salario, _} = Float.parse(sal)

aumento = salario * 1.15

IO.puts("Salario atual de R$#{salario}, com aumento de 15% fica R$#{Float.round(aumento)}")
