// Определение перечислений:

enum IPAddrKind {
    V4,
    V6
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(IPAddrKind::V4);

    main1();
}

fn route(ip_kind: IPAddrKind) {}

// Пример кода с помощью перечисления:

enum IPAddr {
    V4(u32, i64, u8, i8),
    V6(String)
}

fn main1() {
    let home = IPAddr::V4(4, -5, 18, -19);
    let loopback = IPAddr::V6(String::from("::1"));

    main2();
}

// ВАЖНО !!!
// enum (перечисления) отличается от struct (структуры) тем, что внутри структуры находятся значения, а в перечислении 
// находятся структуры, в КОТОРЫХ находятся значения. Фуу-ух.
// Ато я вкурить прикол не мог с тем, почему же в методичке тах хвалят перечисления и булят структуры))

// Примеры того, что я выше сказал:

// 1. Реализация кода с перечислением:

#[derive(Debug)]
enum Message {
    Quit,
    move1(i32),
    Move { x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// 2. Реализация того же, но со структурами:
// Короче, то же самое, только если мы хотим сделать это с помощью самих структур.

//struct Quit1;
//struct Move1 {
//    x: i32, 
//    y: i32
//}
//struct Write(String);
//struct ChangeColor1(i32, i32, i32);

// Вот сам писал, и прочувствовал на себе это прекрасное чувство рутины) Хвала тем, кто создал enum :D

// Дальше идет часть кода для примера 1.

// Метод (impl) для enum:

impl Message {
    fn call(&self) { 
        println!("{:#?}", &self);
    }
}

fn main2() {
    let a = Message::move1(5);
    let m = Message::Write(String::from("Hello!"));
    let x = Message::Move{x: 5, y: 5};

    a.call();
    m.call();
    x.call();

    main3();
}

// !!!!! (Надо будет разобраться, как в структуры через перечисление впихивать числа и несколько значений.) !!!!!

// Перечисление Option и его преимущества перед Null-значениями:

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T)
}

fn main3() {
    let some_number = Some(5);
    let some_char = Some('e');

    // Насчёт того, что находится ниже:
    // В методичке почему-то то, что я раньше писал, было неправильно, но сейчас представлен правильный вариант.
    // Для указания того, что значение, тип которого мы знаем, не существует, или же наоборот указать это значение,
    // нам нужно перед 'None' или 'Значением' приписать 'Option::' !!!!!
    // Даже человек, пишущий методичный материал, может иногда ошибаться, 
    // ну хоть можно быть уверенным в том, что не чат gpt писал))

    let absent_number: Option<i32> = Option::None;

    let number: Option<i8> = Option::Some(-5);

    let x = value_of_coins(Coins::Gaijincoin);

    println!("Значение x: {}", x);

    main4();
}

// Настрадался я с этими enum-ами, конечно примеры с ними не заканчиваются, это только начало моих душевных терзаний с перечислением,
// потому что пока туго идет, но я же программист)) Короче, дальше будет веселее, ПОЕХАЛИ!

// Управляющая конструкция match:

// Пример match с монетами:

enum Coins {
    Gaijincoin,
    Penny,
    Ruble,
    Nickel,
    Dime,
}

fn value_of_coins(сoin: Coins) -> u32 {
    match сoin {
        Coins::Gaijincoin => {
            println!("Gaijin, что за дела? >:(");
            1000
        }
        Coins::Penny => 1,
        Coins::Ruble => 120,
        Coins::Nickel => 4,
        Coins::Dime => 5, 
    }
}

// Шаблоны, которые привязывают значения:

#[derive(Debug)]
enum Russia {
    Steam,
    Gaijin
}

enum Coin {
    Gaijincoin(Russia),
    Penny,
    Ruble,
    Nickel,
    Dime,
}

fn main4() {
    let value = value_of_coins1(Coin::Gaijincoin(Russia::Steam));
    println!("{}", value);

    main5();
}

fn value_of_coins1(сoin: Coin) -> u32 {
    match сoin {
        Coin::Gaijincoin(programm) => {
            println!("Gaijin, что за дела? >:(  {:#?}", programm);
            1000
        }
        Coin::Penny => 1,
        Coin::Ruble => 120,
        Coin::Nickel => 4,
        Coin::Dime => 5, 
    }
}

// Сопоставление шаблона для Option<T>:

fn main5() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Option::None => {
                println!("Значения нет: {:#?}", x);
                Option::None
            }
            Option::Some(a) => {
                println!("Значение есть: {:#?}", x);
                Option::Some(a + 1)
            }
        }
    }

    let five: Option<i32> = Option::Some(5);
    let six = plus_one(five);
    let none1: Option<i32> = Option::None; 
    let none = plus_one(none1);

    println!("Проверка: {:#?}, {:#?}", six, none);

    main6();
}

// Наиздевался надо мной код main5, надеюсь, что в будущем в методичке все будет совпадать с версией моего Rust.
// Теперь я практически полностью уяснил, как работать с Option, но еще осталось несколько маленьких вопросов.
// Думаю, эти вопросы я быстро решу. Теперь же я могу сконцентрироваться на конструкции match :)

// Универсальные шаблоны и заполнитель '_':

// 1) С передающимся значением:

fn main6() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    main7();
}

// 2) Без передающегося значения:

fn main7() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player() {}

    main8();
}

// 3) Ничего не выполняет:

fn main8() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    main9();
}

// Компактное управление потоком выполнения с 'if let':

// 1) Длинное написание с match:

fn main9() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    main10();
}

// 2) То же самое, но компактно и без match:

fn main10() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
