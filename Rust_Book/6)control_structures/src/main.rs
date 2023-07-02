// Выражения if:

fn main() {
    let number = 4;

    if number < 5 {
        println!("True :))");
    } else {
        println!("False :[");
    }
    
    main1();
}

fn main1() {
    let number = 5;

    if number % 7 == 0 {
        println!("Число делится на 7.");
    } else if number % 6 == 0 {
        println!("Число делится на 6.");
    } else if number % 5 == 0 {
        println!("Число делится на 5. {} (O__O) {}", number / 5, number % 5);
    }

    main2();
}

// Повторение выполнения кода с помощью циклов:

fn main2() {
    loop {
        println!("Опять :O");
        break
    }

    main3();
}

// Возвращение значений из циклов:

fn main3() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Это странная часть кода, тут можно обойтись без точки с запятой,
            // а также можно поставить выражение под break, 
            // главное, чтобы выражение было после break !!!!!
            break counter * 5 + 5;
        }
    };

    println!("Значение result: {result}.");

    main4();
}

// Метки циклов для устранения неоднозначности между несколькими циклами:

fn main4() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    main5();
}

// Циклы с условием while:

fn main5() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("Конец подсчёта :3");

    main6();
}

// Цикл по элементам коллекции с помощью for:

fn main6() {
    let x: [u32; 5] = [5, 10, 15, 20, 25];
    let mut index = 0; 

    while index < 5 {
        println!("Значение {index} ячейки массива: {}", x[index]);

        index += 1;
    }

    main7();
}

// Модернизация кода выше (повышение безопасности для того, чтобы код не вылез за пределы 
// массива, и не началась паника.)

fn main7() {
    let a: [i32; 5] = [-5, 5, -15, 18, 19];

    for x in a {
        println!("Значение массива: {x}.");
    // Здесь 'x' -- это значение, с помощью которого мы можем достать из массива
    // все числа, находящиеся в самом массиве. Можно подумать вы сами не поняли ,':)
    // Вот я не понял, ща еще раз перечитаю)
    }

    main8();
}

// Использование цикла for с методом rev:

fn main8() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("КОНЕЦ, пойду пушить код и в кassочку))");
}







