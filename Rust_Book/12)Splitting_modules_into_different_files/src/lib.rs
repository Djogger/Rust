// Проверка

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Разделение модулей на разные файлы:
// (Как же приятно вернуться в UTF-8)))))))

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
