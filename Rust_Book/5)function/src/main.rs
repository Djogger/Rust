fn main() {
    println!("Hello from main!");

// Функции:

    another_function(-5, "hg");
}

fn another_function(x: i32, y: &str) {
    println!("Hello from another_function!");
    println!("Значение x и y: {x}{y}.");

    main1();
}

// Операторы и выражения:

fn main1() {
    let x = {
        let y = 4;
        y + 1
    };

    println!("Значение x: {x}.");

    let x = 5;
    let y = x - 1;

    println!("Значение y: {y}.");

    main2();
}

// Функции с возвращаемыми значениями:

fn five() -> i32 {
    -5
}

fn main2() {
    let x = five();

    println!("Значение x: {x}.");

    main3();
}

fn main3() {
    let x = plus_one(18);

    println!("Значение x: {x}.");
}

fn plus_one(x: u32) -> u32 {
    x + 1
}




