# Crie um algoritmo que leia um número e mostre o seu dobro, triplo e raiz quadrada.
num = int(input("Digite um número: "))
dobro = num * 2
triplo = num * 3
raiz = num ** (1/2)
print(f"A o dobro de {num} é {dobro} \nO tripo é {triplo}\nA raiz quadrada é {raiz:.2f}")