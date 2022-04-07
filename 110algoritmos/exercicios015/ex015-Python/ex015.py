# Escreve um programa que pergunte a quantidade de Km percorridos por um 
# carro alugado e a quantidade de dias pelos quis ele foi alugado.
# Calcule o preço a pagar, sabendo que o carro custa R$60 opr dia e
# R$0.15 por km rodado
dias = int(input("Quantos dias alugados? "))
km = float(input("Quantos Km rodados? "))
total = (dias * 60) + (km * 0.15)
print(f"O total a pagar é R${total:.2f}")