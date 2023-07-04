/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

fn main() {}

struct Rect {
    x: f32,
    y: f32,
    width: f32
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Self {
            x: top_left.0,
            y: top_left.1,
            width: width
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let x_right = &self.x + &self.width;
        let y_right = &self.y - &self.width;
        (x_right, y_right)
    }

    fn area(&self) -> f32 {
        &self.width * &self.width
    }

    fn perimeter(&self) -> f32 {
        let four: f32 = 4.0;
        &self.width * four
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}
