import numpy as np

def numerical_gradient(func, x, h=1e-5):
    """
    Функция для численного вычисления градиента методом конечных разностей.

    func: функция для которой нужно вычислить градиент
    x: точка, в которой вычисляется градиент
    h: малое значение для численного дифференцирования
    """
    grad = np.zeros_like(x)  # Инициализация градиента
    for i in range(len(x)):
        x_h1 = x.copy()
        x_h2 = x.copy()
        x_h1[i] += h  # Увеличиваем значение по i-й переменной
        x_h2[i] -= h  # Уменьшаем значение по i-й переменной
        grad[i] = (func(x_h1) - func(x_h2)) / (2 * h)  # Вычисляем частную производную
    return grad


def gradient_descent(func, x0, eps1, eps2, m, t):
    """
    Реализация метода градиентного спуска.

    func: функция для минимизации
    x0: начальная точка (numpy массив)
    eps1: точность по градиенту
    eps2: точность по изменениям функции и точки
    m: максимальное количество итераций
    t: начальный шаг
    """
    x = np.array(x0, dtype=float)
    k = 0
    while k < m:
        grad = numerical_gradient(func, x)  # Вызов функции для вычисления градиента
        grad_norm = np.linalg.norm(grad)

        # Проверка критерия остановки по градиенту
        if grad_norm < eps1:
            break

        # Обновляем точку
        x_new = x - t * grad

        # Проверка критериев остановки по изменениям
        if np.linalg.norm(x_new - x) < eps2 and abs(func(x_new) - func(x)) < eps2:
            x = x_new
            break

        x = x_new
        k += 1

    return x, k


# Функция для проверки
def f(x):
    return x[0] ** 3 - x[0] * x[1] + x[1] ** 2 - 2 * x[0] + 3 * x[1] - 4  # Задание для проверки № 1
    # return (x[1]-x[0]**2)**2 + (1-x[0])**2 # Задание для проверки № 2
    # return (x[1]**2+x[0]**2-1)**2+(x[0]+x[1]-1)**2 # Задание для исследования № 1
    # return (x[0]**2+x[1]-11)**2+(x[0]+x[1]**2-7)**2 # Задание для исследования № 2
    # return 4*(x[0]-5)**2 + (x[1]-6)**2 # Задания для исследования № 3-5


x0 = [0, 0]
eps1 = 1e-4
eps2 = 1e-4
m = 1000
t = 0.01
result, iterations = gradient_descent(f, x0, eps1, eps2, m, t)
print("Результат:", result)

