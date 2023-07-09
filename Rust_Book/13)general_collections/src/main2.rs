/* Хранение ключей со связанными значениями в Hashmap: */

// Создание новой хеш-карты:

use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 18);
	scores.insert(String::from("Yellow"), 19);

	// Доступ к данным в HashMap:


	// 1) Мой способ:
	let team_name = String::from("Yellow");
	let score = scores.get(&team_name);
	match score {                              // Обработка ошибки, если в get поступает пустое значение.
		Some(value) => println!("{value}"),    // ( Забавно то, что даже без match всё работает :], но как говорится: "Предупреждён, значит вооружён))" )
		None => println!(">:("),               // Ложная тревога, в книге объяснили поч работает, а стерать мне лень, все же это мои духовные переживания и бла-бла-бла ;)
	};

	//println!("{:?}", scores);

	// 2) Способ из книги:
	let team_name = String::from("Yellow");
	let score = scores.get(&team_name).copied().unwrap_or(0);

	println!("{score}\n");

	// Цикл for для HashMap:

	for (key, value) in &scores {
		println!("{key}: {value}");
	}

	main2();
}

// Хеш-карты и владение:

/* Для типов, которые реализуют типаж Copy, например i32, значения копируются в HashMap. 
   Для значений со владением, таких как String, значения будут перемещены в хеш-карту и 
   она станет владельцем этих значений. */

fn main2() {
	let field_name = String::from("Любимый цвет");
	let field_value = String::from("Красный");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	// Дальше использовать переменные field_name и field_value нельзя, т.к. они переданы во владение 'HashMap' map.
	// Проверка:
	//println!("{field_name}, {field_value}");    
	/* Всё правильно. Как сказано, так и работает. Чтобы исправить это, можно передать ссылки на переменные field_... или
	   копирнуть их. :3 */                           

	println!("{map:?}");

	main3();
}

// Обновление данных в Hashmap:

// 1. Перезапись старых значений:

fn main3() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 18);
	scores.insert(String::from("Blue"), 19);

	println!("{scores:?}");

	main4();
}

// 2. Вставка значения только в том случае, когда ключ не имеет значения:

fn main4() {
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 5);

	scores.entry(String::from("Yellow")).or_insert(55);
	scores.entry(String::from("Blue")).or_insert(19);

	println!("{scores:?}");

	main5();
}

// 3. Создание нового значения на основе старого значения:

fn main5() {
	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{map:?}");
}