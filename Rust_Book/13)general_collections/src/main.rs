// Хранение списков значений в векторах:

// Создание нового вектора:

fn main() {

    let v: Vec<i32> = Vec::new();

    let v = vec![4, 5];

    println!("{v:?}");

    main1();
}

// Изменение вектора:

fn main1() {

let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{v:?}");

    main2();
}

// Чтение данных вектора:

fn main2() {
    let v = vec![6, 7, 8, 9, 10];

    let third: &u32 = &v[2];
    println!("Третий элемент вектора: {third}.");

    let third: Option<&u32> = v.get(2);
    match third {
        Some(third) => println!("Третий элемент: {third}."),
        None => println!("Третьего элемента не существует.")
    }

    main3();
}

fn main3() {
    let mut v = vec![4, 5, 18, 19];

    let first = v[0];

    println!("1. {first}, {v:?}");

    v.push(5);

    println!("2. {first}, {v:?}\n");

    main4();
}

// Перебор значений в векторе:

fn main4() {
    let mut v = vec![-1, 2, 3, -4, 5];

    for i in &v {
        println!("{i}");
    }

    println!("\n\r");

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    main5();
}

// Использование перечислений для хранения множества разных типов:

fn main5() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(5.5),
        SpreadsheetCell::Text(String::from("Blue"))
    ];

    println!("{:?}", row);

    main6();
}

// Удаление элементов из вектора:

fn main6() {
    {
        let v = vec![1, 2, 3, 4, 5];

        println!("{v:?}");
    }
    // Тут использовать println!("") не получится :/
}