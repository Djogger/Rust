/* HashMap */

use std::collections::HashMap;

fn main() {
	let mut map: HashMap<&str, u32> = HashMap::new();

	let USSR = "Союз Советских Социалистических Республик";    // ☭
	let Russian_Federation = "Российская Федерация";
	let Arstotzka = "Арстотцка";                               // ☭

	// Метод insert():

	map.insert(USSR, 1922);
	map.insert(Russian_Federation, 1991);
	map.insert(Arstotzka, 1945);

	println!("{map:#?}\n");

	// Метод get():

	let score = map.get(&USSR);
	match score {
		Some(value) => println!("Год основания СССР: {value}.\n"),
		None => println!("(o_o)"),
	}

	// Метод remove():

	println!("HashMap до метода remove: {map:#?}\n");

	map.remove(Arstotzka);

	println!("HashMap после метода remove: {map:#?}\n");

	// Вставка значения только в том случае, когда ключ не имеет значения:

	map.entry(Arstotzka).or_insert(1945);     // Тут значение запишется, т.к. его нет в HashMap.

	println!("Первая попытка: {map:#?}\n");

	map.entry(Arstotzka).or_insert(5);	 // А вот ТУТ нет, оно уже есть:)

	println!("Вторая попытка: {map:#?}\n");

	main1();
}

/* Хэш-функция */

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main1() {
	let input = "Служу Советскому Союзу!";
	let hash = hash_string(input);
	println!("Хешом этой строки: '{}' является: {}", input, hash);
}

fn hash_string(input: &str) -> u64 {
	let mut hasher = DefaultHasher::new();
	input.hash(&mut hasher);
	hasher.finish()
}