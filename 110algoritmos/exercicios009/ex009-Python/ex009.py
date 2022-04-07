#               TABUADA
n = int(input("Digite um número para ver a sua tabuada: "))

print("=-="*4)
x = 1
while x <= 10:
    r = x * n
    print(f"{n} x {x} = {r}")
    x += 1
print("=-="*4)


