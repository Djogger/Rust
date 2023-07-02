fn main() {
    // Логические опрации:
    println!("Логические операции: \n\r");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let quotient1 = -5.0 / 3.0;

    // remainder
    let remainder = 43 % 5;

    println!("Значения: {sum}, {difference}, {product}, {quotient}, {truncated}, {quotient1}, {remainder}.");

    // Составные типы данных:
    // 1) Кортежи:
    // 1. Первый способ получения значения из кортежа:
    
    println!("\n\r");
    println!("Кортежи: \n\r 1)");

    let _cortege1: (i32, f64, u8) = (-500, 6.4, 255);
    let (x, y, z) = _cortege1;
    
    println!("Значения x, y, z: {x}, {y}, {z}.");

    // 2. Второй способ получения значения из кортежа:
    
    println!("\n\r");
    println!("Кортежи: \n\r 2)");

    // Получения значений из _cortege1:
    
    let first_value = _cortege1.0;
    let second_value = _cortege1.1;
    let third_value = _cortege1.2;

    println!("Значения x, y, z: {first_value}, {second_value}, {third_value}.");

    // 2) Массивы:
    
    println!("\n\r");
    println!("Массивы: \n\r");

    // Обычный массив:
    let a1 = [1, 2, 3, 4, 5];

    // Массив с определенным типом записанных в нем значений:
    let a2: [i32; 5] = [1, 2, 3, 4, 5];

    // Массив, содержащий одно и то же значение:
    let a3 = [4; 5];

    let first = a1[2];
    let second = a2[2];
    let third = a3[2];

    println!("Первый массив: {first}.");
    println!("Второй массив: {second}.");
    println!("Третий массив: {third}.");
}
