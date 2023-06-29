fn main() {
    // Зависимость:
    println!("Зависимость: \n\r");

    let mut x = 5;
    println!("Значение x: {x}");
    x = 6;
    println!("Значение x: {x}");

    // Константа:
    println!("\n\r");
    println!("Константа: \n\r");
    
    const YEAR_OF_CRAETING_THE_SOVIET_UNION: u64 = 1900 + 11 * 2;
    println!("СССР был создан в: {YEAR_OF_CRAETING_THE_SOVIET_UNION} году. \n\r ☭");

    // Затенение (переменных):
    println!("\n\r");
    println!("Затенение (переменных): \n\r");

    let x = 4;
    let x = x + 1;
    {
        let x = x * 3 + 4;
        println!("Значение x в скобках: {x}.")
    }

    println!("Значение x: {x}.");

    // Затенение переменных разных типов:
    println!("\n\r");
    println!("Затенение переменных разных типов: \n\r");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Количество пробелов: {spaces}.") 
}
