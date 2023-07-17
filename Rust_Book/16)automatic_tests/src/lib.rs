/* Написание автоматических тестов */

// Как писать тесты:

// Структура тестирующей функции:
/*
#[cfg(test)]
mod tests {
    #[test]
//  #[ignore]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn error() {
        panic!("Специально-вызванная ошибка.");
    }
}
*/

// Проверка результатов с помощью макроса assert!:

/*
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::{Rectangle};

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 19,
            height: 18,
        };
        let r2 = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(r1.can_hold(&r2), "r1 не может хранить в себе r2");
    }

    #[test]
    fn smaller_can_not_hold_smaller() {
        let r1 = Rectangle {
            width: 19,
            height: 18,
        };
        let r2 = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(!r2.can_hold(&r1), "r2 не может хранить в себе r1"); // '!' здесь говорит о том, что если будет false, то все ок. 
    }
}
*/

// Проверка на равенство с помощью макросов assert_eq! и assert_ne!:

// С этим я уже знаком, писать не буду :/
// Если хоть кто-то читает мой код, чтобы подчеркнуть для себя что-нибудь, то увы,
// тут придётся открыть книгу и почитать самому  (; ^ ;)

// Главное помнить, что любой студент сумеет отыскать все, что угодно, если захочет))


// Создание сообщений об ошибках:

// Тут тоже всё просто.

// Проверка с помощью макроса 'should_panic':

/*
pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {                   // '||' - это 'or'; '&&' - это 'and'.
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
*/

// Более точные тесты с 'should_panic' и 'expected':

/*
pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(x: i32) -> Self {
        if x < 1 {
            panic!("Value is smaller than number 1! Value: {}", x);
        } else if x > 100 {
            panic!("Value is bigger than number 100! Value: {}", x);
        }

        Guess {value: x}
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    #[should_panic(expected = "bigger than number 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
*/

// Использование Result<T, E> в тестах:

/*
#[cfg(test)]
mod tests {
    #[test]
//  #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            println!("Сумма верна :3");
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }
}
*/

//----------------------------------------------------------------------------------------------------------------------------------

/* Организация тестов */

// 1) Модульные тесты:

// Тестирование приватных функций (private):

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// 2) Интеграционные тесты:

// С момента написания этой строчки была создана папка tests для тестовых крейтов, надо будет поглядывать туда.