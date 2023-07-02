fn main() {
    for i in 0..=100 {
        if i == 0 {
            println!("{i}");
        }
        else {
            if i % 3 == 0 {
                if i % 15 == 0 {
                    println!("FizzBuzz");
                }
                else {
                    println!("Fizz");
                }
            } else if i % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{i}");
            }
        }
    }
}
