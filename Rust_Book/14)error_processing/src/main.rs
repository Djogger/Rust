/* Обработка Ошибок */


/* 1. Неустранимые ошибки с макросом 'panic!' */

// Раскручивать стек или прерывать выполнение программы в ответ на панику?

fn main() {
    //panic!("crash and burn!");

    main1();
}

// Использование обратной трассировки panic!

fn main1() {
    let v = vec![4, 5, 18, 19];

    //v[99];

    main2();
}

/* Исправимые ошибки с Result */

use std::fs::File;   // Эта библиотека помогает найти файл в системе.
use std::io::{self, Read};   // Эта библиотека помогает прочитать файл в системе.

fn main2() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Невозможно открыть файл: {}", error),
    };

    let mut content = String::new();
    match greeting_file.read_to_string(&mut content) {
        Ok(value) => value,
        Err(error) => panic!("Не удалось прочитать файл: {}", error),
    };

    println!("{content}");

    main3();
}

// Обработка различных ошибок с помощью 'match':

use std::io::ErrorKind;

fn main3() {
    let greeting_file_result = File::open("hello1.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello1.txt") {
                Ok(result) => result,
                Err(error) => panic!("Файл не был создан: {}", error),
            },
            other_error => {
                panic!("Проблема с открытием файла: {}", other_error);
            }
        },    
    };

    main4();
}

// Альтернативы использованию match с Result<T, E>: (Это в главе 13 разберут, тут не вижу смысла пока писать код.
// Итак непонятно как он работает, так в чём смысл писать? Чтобы просто было???   ',:/   )

// Лаконичные способы обработки ошибок - unwrap и expext:

fn main4() {
    let greeting_file = File::open("hello1.txt").unwrap();

    let greeting_file = File::open("hello1.txt").expect("Файл hello1.txt не существует в директории.");

    main5();
    main6();
    main7();
}

// Проброс ошибок:

fn main5() -> Result<String, io::Error> {
    let username_file_result = File::open("hello1.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// Сокращение для проброса ошибок: оператор '?':     (? - Работает также, как match в прошлом примере чуть выше, но это короче.)

fn main6() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hello1.txt")?;

    let mut username = String::new();

    username_file_result.read_to_string(&mut username)?;

    Ok(username)
}

// Оператор '?' с Option<T>:

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main7() {
    assert_eq!(last_char_of_first_line("Hello, World\nWonderful World"), Some('d'));

    main8();
    main9();
}

use std::error::Error;

fn main8() -> Result<(), Box<dyn Error>> {   // О Box<dyn Error> будут говорить в главе 17, а пока можно считать,
                                             // что это "любой вид ошибки".
    let greeting_file = File::open("hello1.txt")?;

    Ok(())
}

// (Опыт):

fn main9() {
    let value = 5;

    if value <1 || value> 5 {
        panic!("Число вне области определеня!");
    }

    println!("Число в области определения :3");

    main10();
}

/* panic! или не panic! */

// Создание пользовательских типов для проверки:

use rand::Rng;
use std::cmp::Ordering;

fn main10() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}