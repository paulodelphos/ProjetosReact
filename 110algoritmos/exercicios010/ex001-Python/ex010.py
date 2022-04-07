#               CONVERSOR DE MOEDAS
#Crie um programa que leia quanto dinheiro a pessoa tem na carteira e mostre
# quantos Dólares ela pode comprar.
# Considera $1,00 = 4.61
real = float(input("\nQuantos reais você tem na carteira? R$ "))
dollar = real / 4.61
print(f"\nCom R$ {real:.2f} reis você pode comprar $ {dollar:.2f} doláres.\n")
