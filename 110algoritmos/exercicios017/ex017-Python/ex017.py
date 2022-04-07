import math

ops = float(input("Comprimento do cateto oposto: "))
adj = float(input("Comprimento do cateto adjacente: "))
hipo = math.hypot(ops, adj)
print(f"A hipotenusa vai medir {hipo:.2f}")
