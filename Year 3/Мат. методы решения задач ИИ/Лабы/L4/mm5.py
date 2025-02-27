import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.neighbors import KNeighborsClassifier
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix

# Загрузка данных
data = pd.read_excel('mm5.xlsx')

# Разделение данных на независимые и зависимые переменные
X = data[['Возраст', 'Средняя зп']]  # Независимые переменные
y = data['Готовность']  # Зависимая переменная

# Нормализация данных
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)

# Разделение на тренировочную и тестовую выборки
X_train, X_test, y_train, y_test = train_test_split(X_scaled, y, test_size=0.3, random_state=42)

# Создание и обучение модели KNN
knn = KNeighborsClassifier(n_neighbors=5)
knn.fit(X_train, y_train)

# Прогнозирование на тестовых данных
y_pred = knn.predict(X_test)

# Оценка точности модели
accuracy = accuracy_score(y_test, y_pred)
print(f"Точность модели: {accuracy:.4f}")

# Отчет по классификации
print("Матрица ошибок:")
print(confusion_matrix(y_test, y_pred))

print("Отчет по классификации:")
print(classification_report(y_test, y_pred))
