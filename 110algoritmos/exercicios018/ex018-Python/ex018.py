import math

angulo = (float(input("Digite o ângulo que você deseja: ")))
seno = math.sin(math.radians(angulo))
cos = math.cos(math.radians(angulo))
tangente = math.tan(math.radians(angulo))
print(f"\nO ângulo de {angulo} tem o SENO de {seno:.2}")
print(f"\nO ângulo de {angulo} tem o COSENO de {cos:.2}")
print(f"\nO ângulo de {angulo} tem o TANGENTE de {tangente:.2}")