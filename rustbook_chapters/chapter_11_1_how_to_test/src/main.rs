// Пример 1

// Тестовый модуль и функция, сгенерированные автоматически с помощью cargo new

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }


// Пример 2

// Добавление второго теста, который завершится ошибкой, потому что мы вызываем panic! макрос

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }


// Пример 3

// Использование структуры Rectangle и её метода can_hold из главы 5

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// Тест для метода can_hold, который проверяет что больший 
// прямоугольник действительно может содержать меньший

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }


// Пример 4

// Два теста работают. Теперь проверим, как отреагируют тесты, если мы добавим ошибку в код.
// Давайте изменим реализацию метода can_hold заменив одно из логических выражений знак 
// сравнения с "больше чем" на противоположный "меньше чем" при сравнении ширины:

// --snip--
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }


// Пример 5

// Тестирование функции add_two с помощью макроса assert_eq!

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }


// Пример 6

// Проверка того, что условие вызовет макрос panic!

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }


// Пример 7

// Проверка panic! на наличие в его сообщении указанной подстроки

// --snip--

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

fn main() {
    println!("AHAHAHAHHAHAH you're too late, Sonic!\nNow I'm a cringe string slice!");
}