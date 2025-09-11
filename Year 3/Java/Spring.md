Энтерпрайз фреймворк с IoC, системами сообщений, абстракциями для доступа к данным

Bean - сервис (или другой управляемый ресурс), удовлетворяющий интерфейс.

Внедрение зависимостей через DI

Фичи:
AOP
Spring Expression Language
Валидации
Абстракции над БД
Управление транзакциями
MVC
Сокеты

![[Assets/Pasted image 20250911141428.png]]
Core - ядро функционала
Bean - классы-сервисы
Context - получение beans через Context resolution + DI

```java
interface Music {}

class FooMusic implements Music {}
class BarMusic implements Music {}

class Player {
	private Music music;
	
	public void PlayMusic() {
		ClassPathXmplApplicationContext ctx = new ClassPathXmlApplicationContext("applicationContext.xml");
		
		music = context.getBean("fooMusic", FooMusic.class);
		music = context.getBean("barMusic", BarMusic.class);
	}
}
```

IOC
```java
interface Music {}

class FooMusic implements Music {}
class BarMusic implements Music {}

class Player {
	private Music music;
	
	public MusicPlayer(Music music) {
		this.music = music
	}
	public void PlayMusic() {}
}
```

IoC + Spring => DI

```java
interface Music {}

class FooMusic implements Music {}
class BarMusic implements Music {}

class Player {
	private Music music;
	
	public MusicPlayer(Music music) {
		this.music = music
	}
	public void PlayMusic() {}
}
```