import math
a = float(input("Введите значение переменной A\n"))
b = float(input("Введите значение переменной B\n"))
c = float(input("Введите значение переменной C\n"))
d = float(input("Введите значение переменной D\n"))

# Variant 5
print("(a - b & int(c)) / (pow(abs(d), 1/3) % 5)")
expr = (int(a - b) & int(c)) / (pow(abs(d), 1/3) % 5) 
print(f'a - b = {a - b}')
print(f'a - b & int(c) = {int(a - b) & int(c)}')
print(f'pow(abs(d), 1/3) = {pow(abs(d), 1/3)}')
print(f'pow(abs(d), 1/3) % 5 = {pow(abs(d), 1/3) % 5}')
print(f'(a - b & int(c)) / (pow(abs(d), 1/3) % 5) = {(int(a - b) & int(c)) / (pow(abs(d), 1/3) % 5)}')
print(expr)

# Variant 10
print("\n2 * a ** b ** c / (3 * b - c) & НОД(d, c)")
expr_two = int(2 * a ** b ** c / (3 * b - c)) & int(math.gcd(int(d), int(c))) # powers ordered explicitly
print(f'b ** c = {b ** c}')
print(f'a ** b ** c = {a ** b ** c}')
print(f'2 * a ** b ** c = {2 * a ** b ** c}')
print(f'3 * b = {3 * b}')
print(f'3 * b - c = {3 * b - c}')
print(f'НОД(d, c) = {math.gcd(int(d), int(c))}')
print(f'2 * a ** b ** c / (3 * b - c) = {2 * a ** b ** c / (3 * b - c)}')
print(f'2 * a ** b ** c / (3 * b - c) & НОД(d, c) = {int(2 * a ** b ** c / (3 * b - c)) & int(math.gcd(int(d), int(c)))}')
print(expr_two)
