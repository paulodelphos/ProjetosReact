# Faça um algoritmo que leia o salário de um funcionário e mostre o seu novo
# salário, com 15% de aumento. entrada: 4319.43 saida: 4967.34
sal = float(input("Qual é o salário do Funcionário? R$"))
aumento = sal * 1.15
print(f"Um funcionário que ganhava R${sal}, com 15% de aumento, passa a receber R${aumento:.2f}")