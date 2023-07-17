/* Типажи: определение общего поведения */

// Определение типажа:

/*
use lib::Test;

pub mod lib;

     !THIS IS NOT THE EXAMPLE OF CURRENT SUBJECT!
fn main() {
	let a = Test { x: 5};

	println!("{:?}\n{}", a, a.x);
}
*/

//use generic_types_types_and_lifetime::{Summary, Country};

/*
fn main() {
	let country = Country {
		name: "USSR".to_string(),
		year: "1922".to_string(),
		leader: "Stalin".to_string(),
		age: "44".to_string(),
		truth: true,
	};

	println!("{}", country.summarize());
}
*/

// Реализация поведения по умолчанию:
/*
use generic_types_types_and_lifetime::{Summary, Country, Country2};

fn main() {
	let country = Country2 {
		name: "USSR".to_string(),
		year: "1922".to_string(),
		leader: "Stalin".to_string(),
		age: "44".to_string(),
		truth: true,
	};

	println!("{}", country.summarize());

	let country = Country {
		name: "USSR".to_string(),
		year: "1922".to_string(),
		leader: "Stalin".to_string(),
		age: "44".to_string(),
		truth: true,
	};

	println!("{}", country.summarize_itog());
}
*/
// Типажи как параметры:
/*
use generic_types_types_and_lifetime::{Summary, Country, notify};

fn main() {
	let country = Country {
		name: "USSR".to_string(),
		year: "1922".to_string(),
		leader: "Stalin".to_string(),
		age: "44".to_string(),
		truth: true,
	};

	let x = notify(&country);
}
*/

// Возврат значений типа реализующего определённый типаж:

/*
use generic_types_types_and_lifetime::{Summary, returns_summary};

fn main() {
	let country = returns_summary();

	println!("{}", country.summarize());
}
*/

// Использование ограничений типажа для условной реализации методов:

use generic_types_types_and_lifetime::{Pair};

fn main() {
	let pair = Pair::new(4.5, 5.6);
	let pair1 = Pair::cmp_display(&pair);
	println!("Pair, сделанный с помощью метода new: {pair:#?}\n");
	
	let pair = Pair::new(4, 5);
	let pair1 = Pair::cmp_display(&pair);
	println!("Pair, сделанный с помощью метода new: {pair:#?}\n");

	main1();
}

fn main1() {
	let s = 19.to_string();

	println!("{s}");
}
//                                                                                               ___
//                                                                _________________________      | |
//                                                               |это я пытался кота в шля-|     | |
//                                                               |пе сделать, а не китайца |   __|_|__
//                                                                -------------------------   (' ^._.^)
// ФУУУУУУУУУУУУУУУУУУХ, это было нелегко, но я смог понять теперь примерно то, как этим можно пользоваться :,)
// Если у вас вопрос, почему я про это говорю, то загляните в lib.rs, он связан с main1))