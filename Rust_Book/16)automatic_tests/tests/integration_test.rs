/* Интеграционные тесты */

/*
use automatic_tests::add_two;

#[test]
fn it_adds_two() {
	assert_eq!(4, add_two(2));
}
*/

// Подмодули в интеграционных тестах:

use automatic_tests::add_two;

mod common;

#[test]
fn it_adds_two() {
	common::setup();
	assert_eq!(4, add_two(2));
}