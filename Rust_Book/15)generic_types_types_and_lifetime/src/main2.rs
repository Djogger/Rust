/* Валидация ссылок при помощи времён жизни */

// ОООО, эта тема пойдёт быстро, т.к. в тундре есть такая же, как мне кажется,
// аналогия. Когда ракета птур летит до танка, она не знает, где находится это танк, чтобы взвестись над
// его крышей, поэтому есть такая вещь, как радио взрыватель, то есть местный 'a))
// С помощью этого взрывателя ракета может взвестись и поразить цель.

// На самом деле я просто хотел рассказать про ракету, поэтому забудьте про этот пример (≖ ‿ ≖)

// Времена жизни предотвращают появление "повисших" ссылок:

fn main() {
	let r;

	{
		let x = 5;
		r = &x;

		println!("{r}");
	}

	main1();
}

// Обобщённые времена жизни в функциях:

fn main1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}\n", result);

	main2();
}

// (В определении функции longest указано, что все ссылки должны иметь одинаковое время жизни, обозначенное как 'a):
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	let truth: bool = x.len() > y.len();
	match truth {
		true => x,
		false => y,
	}
}

// Аннотации времени жизни в сигнатурах функции:

fn main2() {
    let string1 = String::from("abcd");
    {	
		let string2 = "xyz";
		let result = longest1(string1.as_str(), string2);
		println!("The longest string is {}\n", result);
	}

	main3();
}

fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
	let truth: bool = x.len() > y.len();
	match truth {
		true => x,
		false => y,
	}
}

// Определение времён жизни при объявлении структур:

struct ImportantExcerpt<'a> {
	past: &'a str,
}

fn main3() {
	let novel = "The missle knows there it is at all the time. So the missle knows and gets the target because it knows there it is at all the time...";
	let first_sentence = match novel.split('.').next() {
		Some(value) => value,
		None => {
			println!("So the missle doesn't know and get the target because it doeasn't know there it is at no time");
			panic!(">:(")
		}
	};
	let p = ImportantExcerpt {
		past: first_sentence,
	};

	println!("{}", p.past);

    main4();
}

// Правила неявного выведения времени жизни:

// Тут можно обойтись без 'a:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main4() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{word}");

    main5();
}

// И тут можно обойтись без 'a:
fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main5() {
    let my_string = String::from("hello world");

    // first_word1 works on slices of `String`s
    let word = first_word1(&my_string[..]);

    {
        let my_string_literal = "hello world";

        // first_word1 works on slices of string literals
        let word = first_word1(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word1 = first_word1(my_string_literal);
        println!("{word1}\n");
    }

	main6();
}

// Правила времени жизни ссылки:

/* 
1) Если в фун-ии имеется один входной параметр ссылки, то надо писать так:

fn example<'a>(s: &'a str) -> &'a str {...

2) Если в фун-ии несколько входных параметров, то у них могут быть разные времена жизни:

fn example<'a, 'b>(s1: &'a str, s2: &'b str) -> &str {...     тут нужно обратить внимание на возвращаемый тип -(O_O ) у него теперь нет 'a или 'b

3) "Третье правило связано с методами, поэтому дальше начнём с времён жизни в опр. методов" ----->
*/

// Аннотация времён жизни в определении методов:

struct ImportantExcerpt1<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt1<'a> {
	fn level(&self) -> i32 {
		5
	}
}

impl<'a> ImportantExcerpt1<'a> {
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		println!("War Thunder realistic thing is: {}", announcement);
		self.part
	}
}

fn main6() {
	let novel = "The missle knows there it is at all the time. So the missle knows and gets the target because it knows there it is at all the time...";
	let first_sentence = match novel.split('.').next() {
		Some(value) => value,
		None => {
			println!("So the missle doesn't know and get the target because it doeasn't know there it is at no time");
			panic!(">:(")
		}
	};
	let p = ImportantExcerpt1 {
		part: first_sentence,
	};

	println!("{}", p.part);
	println!("{}", p.level());
	println!("{}", p.announce_and_return_part("Donate for free playing:D"));

	main7();
}

// Статическое время жизни:

// (Ссылка, которая может жить всю продолжительность работы программы)

// let s: &'static str = "Я имею статичесое время жизни (- v -)";

/* Обобщённые типы параметров, ограничения типажей и времена жизни вместе */

// Код, совмещающий все темы подглав главы в себе:

fn main7() {
	let s1 = String::from("Slovooooooooo");
	let s2 = "Slovo";
	let s3 = "Slovosochetanie";

	let answer = longest_with_the_annoucement(s1.as_str(), s2, s3);

	println!("Final answer: {} :]", answer);
}

use std::fmt::Display;

fn longest_with_the_annoucement<'a, T> (
	x: &'a str, 
	y: &'a str,
	value: T,
) -> &'a str 
where
	T: Display,
{
	println!("Announcement from Arstotzka delegation: {}.", value);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

// С этой главой покончено, а теперь к следующей :)