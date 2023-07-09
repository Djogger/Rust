/* Вектор */

fn main() {
    // Методы: new(), push().
    let mut vec: Vec<i32> = Vec::new();

    println!("Ёмкость: {}", vec.capacity());

    vec.push(4);
    vec.push(18);
    vec.push(19);
    vec.push(5);

    println!("Ёмкость: {}", vec.capacity());
    println!("Дан вектор: {vec:?}\n");

    // Метод: pop().
    let x = vec.pop();
    match x {
        Some(value) => println!("Последний удаленный элемент вектора: {value}."),
        None => println!("	(`o_o´)  "),
    }
    println!("{vec:?}\n");

    // Метод: remove().
    let y = vec.remove(0);
    println!("Удаленный элемент из вектора: {y}.");

    println!("{vec:?}\n");

    // Метод: get().
    let second: Option<&i32> = vec.get(1);
    match second {
        Some(value) => println!("Второе значение в векторе: {value}.\n"),
        None => println!("	(`0-0´)  "),
    }

    // Метод: resize().
    let mut vec = vec![4, 5, 18, 19, 100];

    println!("Дан вектор: {vec:?}");

    // 1. Если увеличиваем:
    vec.resize(7, 19);
    println!("Увеличенный вектор: {vec:?}");

    // 2. Если уменьшаем:
    vec.resize(2, 0);
    println!("Уменьшенный вектор: {vec:?}\n");

    // Метод with_capacity():

    let mut vec = Vec::with_capacity(2);

    println!("Ёмкость: {}", vec.capacity());

    vec.push(4);
    vec.push(5);

    println!("Вектор: {:?}, Его ёмкость: {}.", vec, vec.capacity());

    vec.push(6);

    println!("Вектор: {:?}, Его ёмкость: {}.", vec, vec.capacity());

    vec.push(18);
    vec.push(19);

    println!("Вектор: {:?}, Его ёмкость: {}.", vec, vec.capacity());
}
