/* Обобщённые типы, типажи и время жизни */

// Удаление дублирования кода с помощью выделения общей функциональности:

// 1) Код с дублированием:

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    println!("{number_list:?}    :)\n");

    let number_list = vec![349, 504, 252, 10000, 655];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    println!("{number_list:?}    :)\n");

    main1();
}

// 2) Тот же код без дублирования:

fn largest(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}   :]\n", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}   :]\n", result);

    main2();
}

// ЭТО БЫЛ РАЗГОН, А ТЕПЕЕЕЕРЬ)))) ПОЕХАЛИ!!!!!

/* Обобщённые типы данных */

// 1. В объявлении функции:

// 1) Код (БЕЗ ОБОБЩЕНИЯ):

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}\n", result);
    assert_eq!(*result, 'y');

    main3();
}

// 2) Тот же код (С ОБОБЩЕНИЕМ):

fn largest1<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main3() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest1(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest1(&char_list);
    println!("The largest char is {}\n", result);
    assert_eq!(*result, 'y');

    main4();
}

// 2. В определении структур:

// 1) С одинаковым типом:

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main4() {
    let integer = Point { x: 4, y: 5};
    let float = Point { x: 4.5, y: 5.5};

    println!("{:?}\n{:?}\n", integer, float);

    main5();
}

// 2) С разными типами:

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

fn main5() {
    let integer_f = Point1 { x: 4, y: 5.5};
    let float_i = Point1 { x: 4.5, y: 5};
    let both_f = Point1 { x: 4.1, y: 19.5};
    let both_i = Point1 { x: 18, y: 19};

    println!("{:?}\n{:?}\n{:?}\n{:?}\n", integer_f, float_i, both_f, both_i);

    main6();
}

// 3. В определениях перечислений:

// Это мне уже знакомо, но зато теперь можно более трезво взглянуть на эти перечисления -(*￣ ︶ ￣* ;)

// 1) Option<T> :

enum Option<T> {
    Some(T),
    None,
}

// 2) Result<T, E> :

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 4. В определении методов:

// Первый пример:

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main6() {
    let p = Point2 { x: 5, y: 5.};

    println!("{:?}", p);

    println!("p.x() = {}", p.x());
    println!("p.y() = {}\n", p.y());

    main7();
}

// Второй пример:

#[derive(Debug)]
struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<T2, U2>(self, other: Point3<T2, U2>) -> Point3<T, U2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main7() {
    let p1 = Point3 {
        x: 4.5,
        y: 1922,
    };
    let p2 = Point3 {
        x: "USSR",
        y: 'a',
    };

    println!("{:?}\n", p1);
    println!("{:?}\n", p2);

    let p3 = p2.mixup(p1);

    println!("{:?}\n{} was founded in: {}.\n", p3, p3.x, p3.y);

    main8();
}

// Производительность кода, использующего обобщённые типы:

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

fn main8() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    println!("{:?}", integer);
    println!("{:?}", float);
}