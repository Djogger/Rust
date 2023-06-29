// Правила владения:
// 1) У каждого значения в Rust есть владелец,
// 2) У значения может быть только один владелец в один момент времени,
// 3) Когда владелец покидает область видимости, значение удаляется.

fn main() {           // значение x пока не действует, так как еще не объявлен.
    let x = "Word.";  // значение x уже действует с этого момента, так как мы его объявили.

    println!("{x}");

    main1();
}                     // здесь закончилось тело фун-ии, поэтому дальше значение x не действует.

// Это мой эксперемент по теме))
fn main1() {
    //let x = String::from("hh");
    let x = {
        let x = String::from("a");
        x + "h"
    };

    println!("{x}");

    main2();
}
//                                                                                 _ _
// Тип данных String (почему литералы не могут изменяться, а строковой тип может? (o_O ) 
// Дальше объясню сам себе)

fn main2() {
    let mut s = String::from("Counter");
    //let a = "Counter";   // .push_str() -- не может использоваться с s, если этот s -- литерал!!!!!
    s.push_str(" Strike"); // .push_str() -- добавляет литерал к строке.
    println!("{s}");

    main3();
}
// Итак, почему String может меняться? Дело тут в том, что строковой литерал жестко прописан,
// поэтому не может изменяться, а вот строку можно
// использовать, когда мы не знаем точное значение, которое может потом понадобиться,
// поэтому, программа может спокойно соединить одно значение строки с другим значением строки, 
// но не значением типа 'char'. 
// { Уже проверил в эксперементе выше:) }.

// 1. Взаимодействие переменных и данных с помощью перемещения:

fn main3() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{s2}, world!");    // Тут можно использовать только значение s2, т.к.
                                 // Rust самостоятельно признает s1 недействительным,
                                 // чтобы экономить память, но это не значит, что мы не
                                 // можем попробовать глубоко скопировать данные, чтобы их дублировать!

    main4();
}

// 2. Взаимодействие переменных и данных с помощью клонирования:

fn main4() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}.");    // В этом случае уже можно использовать s1, т.к. мы его
                                // не передали s2, а клонировали в него кучу.
    main5();
}

// Владение и функции:

fn main5() {
    let s = String::from("hello");  // появилось значение s.

    takes_ownership(s);             // значение s передается в фун-ию...
                                    // ... далее значение недействительно.

    let x = 5;                      // появилось значение x.

    makes_copy(x);                  // x перейдет в фун-ию,
                                    // но это тип i32, поэтому он, уже хранясь полностью в стеке,
  //println!("{s}{x}");             // останется действительным. Дальше можно использовать.

    main6();
} // Здесь уже и значение x недействительно, закончилось тело фун-ии.

fn takes_ownership(some_string: String) { // some_string принимает переведенное значение.
    println!("{some_string}");
} // Здесь конец тела фун-ии, запрашивается drop. Память
  // освобождена.

fn makes_copy(some_integer: i32) { // some_string принимает переведенное значение.
    println!("{some_integer}");
} // Здесь конец тела фун-ии.

// Возвращение значений и область видимости (тут переводить комментарии не буду, уже все разобрано выше. А английский 5-ого класса я знаю))

fn main6() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("{s1} {s3}");   // s2 уже удален Rust-ом, можем использовать только s1 и s3.
 
    main7();   
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

//

fn main7() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    main8();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() возвращает длину строки.

    (s, length)
}

//  -----------------------------------------------------------------------------------------

// Ссылки и заимствование:

fn main8() {
    let x = String::from("Minecraft");
    let _len = find_len(&x);

    println!("Длина слова '{x}':  {_len}.");

    main9();
}

fn find_len(a: &String) -> usize {
    a.len()
}

// Изменяемая ссылка:
//                                                                               ( ?_?)    (._. )
// Чтож, сначала стоит начать с правил использования ссылки, то есть, когда её можно изменить?
// 1) Если уже используется изменяемая ссылка, то не получится использовать её одновременно для
// другого значения. Ко второму аргументу можно будет привязать, например только после макроса типа 'println!("")'.
// 2) Если используется неизменяемая ссылка, то тоже нельзя использовать одновременно с изменяемой.
// 3) А вот две неизменяемые можно использовать))
// В общем, мы должны понимать ограничения области, чтобы понять, где можно использовать, а где нет!

fn main9() {
    let mut s = String::from("War Thunder.");

    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}");

    let r3 = { let _ = &mut s;
        s + ":D"
    };

    println!("{r3}");

    main11();  
}

// Висячие ссылки:

// Ниже представлен некомпилируемый код. Некомпилируемый он, потому что имеет 
// в себе висячую ссылку, которая ссылает на ничто. Rust укажет нам на ошибку,
// поэтому не нужно детально рассматривать каждую строку кода, всё предусмотрено.

//fn main10() {
//    let reference_to_nothing = dangle();

//}

//fn dangle() -> &String {
//    let s = String::from("Shrek");

//    &s
//}

// Ниже представлено мой эксеримент и исправление кода вверху:

// Эксперемент:

fn main11() {
    let mut s1 = String::from("AAAAA");
    let s3 = "D";
    badamss(&mut s1, s3);

    no_dangle();

    main12();
}

fn badamss(a: &mut String, b: &str) {
    a.push_str("B");
    println!("a -- {a}, b -- {b}");
}

// Исправление кода:

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// --------------------------------------------------------------------------------------------

// Тип срезы:

// 1) Строковые срезы:

fn main12() {
    let x = String::from("Hello World!");
    let slice1 = &x[..];
    let slice2 = &x[6..];
    let slice3 = &x[..5];

    let y = "Oooops";
    let slice_y = &y[2..5];

    println!("{slice1}, {slice2}, {slice3}, {slice_y}.");

    main13();
}

// Фун-ия, находящая первое и второе слово в строке (Автор -- Я:}):

fn main13() {
    let s = String::from("War Thunder !");
    
    let x = first_word(&s[..]);     // Тут я показал, что функция first_word забирает с собой
                                    // срез, это удобно, когда нужно часть строки анализировать,
                                    // а не её всю. (Можно не писать &s[..], если мы хотим взять всё, а можно просто оставить &s, 
                                    // как ссылку, но это тоже может быть срезом, просто несрезающем ничего со строки.)

    let y = second_word(&s);

    println!("{x} и {y}");

    main14();
}

fn first_word(a: &str) -> &str {
    for i in 0..= a.len() {
        if a.chars().nth(i) == Some(' ') {     // .chars() -- конвертирует строку в итератор,
                                               // что позволяет перебирать символы по одному.
                                               // .nth(i) -- получает определенный символ при
                                               // помощи 'i'.
            return &a[..i];
        }
    }
    &a[..]
}

fn second_word(a: &str) -> &str {
    let mut index1 = 0;
    let mut index2 = 0;
    let mut number = 0;

    for i in 0..= a.len() {
        if number == 1 && a.chars().nth(i) == Some(' ') {
            index2 += i;
            println!("Второй пробел есть");
            return &a[index1..index2];
        }
        if number == 0 && a.chars().nth(i) == Some(' ') {
            index1 += i + 1;
            number += 1;
            println!("Первый пробел есть");
        }
    }
    &a[..]
}

// Фун-ия, находящая первое слово в строке (из методички):

fn main14() {
    let s = String::from("War Thunder");
    
    let x = first_word2(&s);

    println!("{x}");
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
