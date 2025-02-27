import pandas as pd
from sklearn.preprocessing import StandardScaler
from sklearn.cluster import KMeans
from sklearn.metrics import silhouette_score 

# Загрузка данных
data = pd.read_excel('mm4.xlsx')

# Убираем пробелы из имен столбцов, если они есть
data.columns = data.columns.str.strip()

# Числовые переменные
numerical_columns = ['Возраст', 'Средняя зп']

# Создаем объединенный набор данных с числовыми признаками
X = data[numerical_columns]

# Нормализация данных
scaler = StandardScaler()
X_normalized = scaler.fit_transform(X)

# Применение K-Means с 3 кластерами
kmeans = KMeans(n_clusters=3, random_state=42)
data['Cluster'] = kmeans.fit_predict(X_normalized)

# Анализ кластеров
print(f"Количество точек в каждом кластере:\n{data['Cluster'].value_counts()}")

# Отключаем ограничение на количество отображаемых столбцов
pd.set_option('display.max_columns', None)

# Средние значения по кластерам
print(f"\nСредние значения по признакам для каждого кластера:\n{data.groupby('Cluster').mean()}")

# Анализ готовности остаться
print(f"\nРаспределение 'Готовность остаться' по кластерам:\n{data.groupby('Cluster')['да'].sum()}")

# Расчёт силуэтного коэффициента
sil_score = silhouette_score(X_normalized, data['Cluster'])
print(f"\nСилуэтный коэффициент: {sil_score}")