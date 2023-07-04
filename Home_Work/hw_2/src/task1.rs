/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    find_term(SEARCH_TERM, QUOTE);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut enter = 0;
    let mut position_of_enter1 = 0;
    let mut position_of_enter2 = 0;
    let mut truth = 0;
    let mut last_i = 0;
    let mut final_itog: String = String::from("");

    for i in 0..quote.len() {
        if quote.chars().nth(i) == Some('\n') {
            if enter == 0 {
                position_of_enter1 += i+1;
                enter += 1;
            }
        }
        if quote.chars().nth(i) == search_term.chars().nth(0) {
            let x = search_term.len();
            let length = quote.len();
            if &quote[i..length].len() > &(i+x) {
                if &quote[i..i+x] == search_term {
                    truth += 1;
                    last_i += i;
                }
            }
        }
    }
    if truth == 1 {
        for x in last_i+1..quote.len() {
            if quote.chars().nth(x) == Some('\n') {
                if enter == 1 {
                    position_of_enter2 += x;
                    enter += 1;
                    let itog = &quote[position_of_enter1..position_of_enter2];
                    final_itog = enter.to_string() + ": " + itog;
                }
            }
        }
    }
    final_itog
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}