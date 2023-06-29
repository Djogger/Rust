// Определение и инициализация структур:

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("KriStiA"),
        email: String::from("laboodaboodabdab@mail.ru"),
        sign_in_count: 1
    };
    println!("{}", user1.active);       // Здесь я хочу вывести в терминал значение из структуры.
    println!("{}", user1.username);     // Здесь то же самое, но для того, чтобы в будущем показать, как изменится значение.
    user1.username = String::from("Mr.Mocki-Docki");    // Тут я меняю значение в структуре. (Важно отметить, чтобы изменить
                                                        // значение в структуре нужно сделать саму структуру изменяемой!)
    println!("\n\r{}", user1.username);

    let user2 = build_user(user1.username, user1.email);  // Возвращает значение структуры в фун-ии build_user, с переданными значениями из другой структуры.

    println!("Username: {}", user2.username);
    println!("email: {}", user2.email);

    main1();
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        //username: username,     // Можно обойтись без повторения тех же имён. Под комментариями username и email показано, как это сделать)
        //email: email,
        username,
        email,
        sign_in_count: 1
    }
}

// Создание экземпляра структуры из экземпляра другой структуры с помощью синтаксиса обновления структуры:

fn main1() {
    let user1 = User {
        active: true,
        username: String::from("Dan"),
        email: String::from("while@mail.ru"),
        sign_in_count: 1
    };

    println!("\n\r{}, {}", user1.username, user1.email);

// Это длинный и подробный вариант написания:
//    let user2 = User {
//        active: user1.active,
//        username: user1.username,
//        email: String::from("Ching@mail.ru"),
//        sign_in_count: user1.sign_in_count
//   };

// Ниже представлен более простой и короткий вариант написания, если нам надо изменить несколько значений, но не все:
    let user2 = User {
        email: String::from("Ching@mail.ru"),
        ..user1     // Этот оператор передаёт все оставшиеся значения из структуры 'user1'.
    };

    // Нужно отметить, что теперь, после создания 'user2', мы не можем больше использовать 'user1', т.к. из user1 был взят String
    // значения username, если бы мы его не брали, то тогда user1 был бы еще действителен.

    println!("\n\r{}, {}", user2.username, user2.email);

    main2();
}

// Кортежные структуры: структуры без именованных полей для создания разных типов:

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Color(4, 0, 0);
    let origin = Point(0, 5, 0);

    println!("{}, {}", black.0, origin.1);

    main3();
}

// Единично-подобные структуры: структуры без полей:

    // Тут пока можно подробно не писать пример, то есть вообще его не писать, хех:)
    // В 10-ой главе методички их подробно разберут, а главное покажут для чего их можно использовать,
    // а так, единичная структура объявляется также, как и выше представленные, только без скобок, просто имя структуры.

// Владение данными структуры:

    // Тут также всё объяснят в главе 10. Из того, что я прочитал, я понял, что пока структура может существовать с владеющим
    // типом String, но конечно, когда я научусь указывать время жизни ссылки, то и ссылки можно будет хранить в структурах.


// Пример использования структур:

fn main3() {
    let width = 30;
    let height = 50;

    println!(
        "\n\rПлощадь прямоугольника: {}",
        area(width, height)
    );

    main4();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Рефакторинг при помощи кортежей:

fn main4() {
    let cortege: (u32, u32) = (4, 5);

    println!(
        "\n\rПлощадь прямоугольника: {}",
        area1(cortege)
    );

    main5();
}

fn area1(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}

// Рефакторинг при помощи структур:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main5() {
    let demensions = Rectangle {
        width: 5,
        height: dbg!(9 + 1)
    };

    dbg!(&demensions);

    println!(
        "\n\rПлощадь прямоугольника: {}",
        area2(&demensions)
    );
    
    println!("demensions is: {:#?}", demensions);

    main6();
}

fn area2(values: &Rectangle) -> u32 {
    values.width * values. height
}

// Синтаксис метода:

// 1) Определение методов:
// 2) Методы с несколькими параметрами:

impl Rectangle {
    fn area3(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main6() {
    let demensions = Rectangle {
        width: 5,
        height: 9
    };
    let demensions2 = Rectangle {
        width: 6,
        height: 9
    };
    let demensions3 = Rectangle {
        width: 4,
        height: 5
    };

    println!(
        "\n\rПлощадь прямоугольника: {}",
        demensions.area3()
    );

    println!("\n\rМожет ли в demensions поместиться demensions2: {}", demensions.can_hold(&demensions2));
    println!("\n\rМожет ли в demensions поместиться demensions3: {}", demensions.can_hold(&demensions3));

    main7();
}

// 3) Ассоциированные функции:

impl Rectangle {
    fn square(value: u32) -> Self {
        Self {
            width: value,
            height: value
        }
    }
}

fn main7() {
    let x = Rectangle::square(5);
    println!("Площадь квадрата: {}, {:#?}", x.area3(), x);
}