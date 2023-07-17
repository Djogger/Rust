/* Типажи: определение общего поведения */

// Определение типажа:

//-----------------------------------------------------------------------------------------------------------------------------------//
/*
#[derive(Debug)]
pub struct Test {
	pub x: i32,
}
ЭТО БЫЛА ПРОВЕРКА, что-то я с того урока про пути файловой системы ни разу не занимался этим ੧| ‾︶ ‾́  |੭
Но всегда приходит время чем-то заняться, т.ч. это тот случай))
*/
//-----------------------------------------------------------------------------------------------------------------------------------//
/*
pub trait Summary {
	fn summarize(&self) -> String;
}

// Реализация типажа у типа:

pub struct Country {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country {
	fn summarize(&self) -> String {
		format!("{} is the Leader of the {}, that was founded in: {}.", &self.leader, &self.name, &self.year)
	}
}
*/

// Реализация поведения по умолчанию:
/* 1)
pub trait Summary {
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
}

pub struct Country {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country {}

pub struct Country2 {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country2 {
	fn summarize(&self) -> String {
		format!("{} is the Leader of the {}, that was founded in: {}.", &self.leader, &self.name, &self.year)
	}
}
*/
// 2)
/*
pub trait Summary {
	fn summarize(&self) -> String;

	fn summarize_itog(&self) -> String {
		format!("(Read more from {}...)", &self.summarize())
	}
}

pub struct Country {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country {
	fn summarize(&self) -> String {
		format!("@{}", &self.name)
	}
}

pub struct Country2 {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country2 {
	fn summarize(&self) -> String {
		format!("{} is the Leader of the {}, that was founded in: {}.", &self.leader, &self.name, &self.year)
	}
}
*/
// Типажи как параметры:
/*
pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct Country {
	pub name: String,
	pub year: String,
	pub leader: String,
	pub age: String,
	pub truth: bool,
}

impl Summary for Country {
	fn summarize(&self) -> String {
		format!("{} is the Leader of the {}, that was founded in: {}.", &self.leader, &self.name, &self.year)
	}
}

pub fn notify(item: &impl Summary) {
	println!("Breaking News! {}. HOW IS IT POSSIBLE?!?!?! ლ(o◡oლ)", item.summarize());
}
*/

// Синтаксис ограничения типажа:

// Тут расписывать код не буду, просто сам себе скажу, что то, что находится ниже, лишь сокращение fn notify чуть выше.
// Это можно использовать в более сложных функциях.
// Сначала о том, как реализовывается синтаксис ошраничения типажа, а затем пример более сложной функции:

/*
pub fn notify<T: Summary>(item: &T) {
	println!("Breaking News! {}. HOW IS IT POSSIBLE?!?!?! ლ(o◡oლ)", item.summarize());
}

// Код выше - это та же функция notify, но уже более коротко. Далее будет понятно почему БОЛЕЕ КОРОТКО)

1. Без синтаксиса ограничения типажа:

pub fn notify(item1: &impl Summary, item2: &impl Summary) {

2. С синтаксисом ограничения типажа:

pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Вот представьте, если больше 5 надо будет вписать, там до края мира майнкравта недалеко...
// Но при всём при том 1. способ написания может сгодиться, если нам нужны разные типы.
// Ну тут всё ясно, Товарищи. Идём дальше? ',:>)
*/

// Задание нескольких границ типажей с помощью синтаксиса '+':

// 1.

// pub fn notify(item: &(impl Summary + Display)) {

// 2.

// pub fn notify<T: Summary + Display>(item: &T) {

/* Вот пример примерно того, как выглядит код с этим:

use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Notification: {}", item.summarize());
    println!("Display: {}", item);
}

fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
    };

    notify(&book);
}
*/

//  Более ясные границы типажа с помощью 'where':

/*
// 1. Вместо этого:

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...

// 2. Можно писать это:

fn some_function<T, U>(t: &T, u: &U) -> i32
where
	T: Display + Clone,
	U: Clone + Debug,
{...
*/

// Возврат значений типа реализующего определённый типаж:

/*
pub trait Summary {
	fn summarize(&self) -> String;
}

pub struct Country {
	name: String,
	age: String,
}

impl Summary for Country {
	fn summarize(&self) -> String {
		format!("{} was founded in: {}.	ヽ(՞ ᗜ ՞ )ง", &self.name, &self.age)
	}
}

pub fn returns_summary() -> impl Summary {
	Country {
		name: "Russian Federation".to_string(),
		age: "1991".to_string(),
	}
}
*/

// Использование ограничений типажа для условной реализации методов:

use std::fmt::Display;

#[derive(Debug)]
pub struct Pair<T> {
	pub x: T,
	pub y: T,
}

impl<T> Pair<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x: x,
			y: y,
		}
	}
}

impl<T: Display + PartialOrd> Pair<T> {
	pub fn cmp_display(&self) {
		if self.x > self.y {
			println!("Наибольшее значение - это x: {}", self.x);
		} else {
			println!("Наибольшее значение - это y: {}", self.y);
		}
	}
}
/*
// Общая реализация (Реализация типажа для любого типа, который удовлетворяет ограничениям типажа):

impl<T: Display> ToString for T {
    // --snip--
}

// Эта строчка кода определяет, что для любого типа T, который реализует трейт Display, 
// также будет реализован трейт ToString. Трейт ToString позволяет преобразовывать значение в строку.
*/