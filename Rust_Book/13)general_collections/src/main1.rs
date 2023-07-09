/* Хранение закодированного текста UTF-8 в строках: */

// Создание новых строк:

fn main() {
	let mut s = String::new();

	// 1)
	let data = "String1";
	let s1 = data.to_string();

	// 2)
	let s2 = "String2".to_string();

	//3)

	let s3 = String::from("String3");

	println!("{s}, {s1}, {s2}, {s3}");

	main1();
}

/* Обновление строковых данных: */

// Присоединение к строке с помощью push_str и push:

fn main1() {
	let mut s = String::from("foo");
	s.push_str("bar");

	println!("{s}");

	let mut s = String::from("foo1");
	let s2 = "bar1";
	s.push_str(s2);

	println!("{s}, {s2}");

	let mut s = String::from("lo");
	s.push('l');

	println!("{s}");

	main2();
}

// Объединение строк с помощью оператора '+' или макроса 'format!':

fn main2() {
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;

	println!("{s3}, {s2}");     // s1 нельзя больше использовать, т.к. мы передали его без ссылки, а полностью во владение s3.

	main3();
}

fn main3() {
	// 1) Первый способ (не очень удобый), если нужно объединить несколько строк:

	let s1 = String::from("Tic");
	let s2 = String::from("Tac");
	let s3 = String::from("Toe");

	let s = s1 + "-" + &s2 + "-" + &s3;

	println!("{s}");

	main4();
}

fn main4() {
	// 2) Второй способ (более удобый):

	let s1 = String::from("Tic");
	let s2 = String::from("Tac");
	let s3 = String::from("Toe");

	let s = format!("{s1}-{s2}-{s3}");	 // Ещё стоит отметить, что макрос format!() использует ссылки НА КАЖДОЕ ЗНАЧЕНИЕ!
	                                     // Значит теперь и s1 можно использовать дальше. УРА! :D

	println!("{s}, s1--{s1}");

	main5();
}

/* Индексирование в строках: */

// Срез строк:

fn main5() {
	let hello = "hЗдравствуйте";

	let s = &hello[0..5];

	println!("{s}");

	main6();
}

// Методы для перебора строк:

fn main6() {
	for i in "hЗд".chars() {
		println!("{i}");
	}

	for b in "hЗд".bytes() {
		println!("{b}");
	}

	main7();
}

// Изучение метода '.replace()':

fn main7() {
	let s = String::from("Bla Bla Hey Nigarus");

	println!("{}", s.replace("Bla", "Wow"));
}

#[cfg(test)]
mod tests {
	// Изучение метода '.contains()':
	#[test]
	fn test1() {
		let v = vec![4, 5, 18, 19];

		let truth = v.contains(&19);

		assert!(truth, "F");
	}

	#[test]
	fn test2() {
		let v = String::from("Hello, Bletman");

		let truth = v.contains("Bletman");

		assert!(truth, "No Beaches??? :(");
	}

	#[test]
	fn test3() {
		let s = String::from("Bla Bla Hey Nigarus");

		let s = s.replace("Bla", "Wow");

		assert_eq!("Wow Wow Hey Nigarus", s, "No Negarus???  ',8[  ");
	}
}