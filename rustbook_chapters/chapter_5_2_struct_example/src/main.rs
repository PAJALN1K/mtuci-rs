// Чтобы понять, когда нам может понадобиться использование структур, давайте напишем программу,
// которая вычисляет площадь прямоугольника.


// Пример 1

// Вычисление площади прямоугольника, заданного отдельными переменными ширины и высоты

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Проблема данного метода очевидна из сигнатуры area
// Функция area должна вычислять площадь одного прямоугольника, но функция, которую мы написали,
// имеет два параметра, и нигде в нашей программе не ясно, что эти параметры взаимосвязаны.


// Пример 2

// Определение ширины и высоты прямоугольника с помощью кортежа

// Кортежи позволяют добавить немного структуры, и теперь мы передаём только один аргумент.
// Но эта версия менее понятна: кортежи не называют свои элементы, поэтому нам приходится
// индексировать части кортежа, что делает наше вычисление менее очевидным.

// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// Пример 3

// Определение структуры Rectangle

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// Добавление полезной функциональности при помощи выводимых типажей

// Было бы полезно иметь возможность печатать экземпляр Rectangle во время отладки программы и
// видеть значения всех полей.


// Пример 1

// это не работает.

// Примитивные типы, изученные ранее, по умолчанию реализуют типаж Display, потому что есть только
// один способ отобразить число 1 или любой другой примитивный тип. Но для структур форматирование
// println! менее очевидно, потому что есть гораздо больше способов отображения: Вы хотите запятые
// или нет? Вы хотите печатать фигурные скобки? Должны ли отображаться все поля?

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is {}", rect1);
// }


// Пример 2

// Это работает

// Rust реализует функциональность для печати отладочной информации, но не включает
// (не выводит) её по умолчанию. Мы должны явно включить эту функциональность для нашей структуры.
// Чтобы это сделать, добавляем внешний атрибут #[derive(Debug)] сразу перед определением структуры

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!("rect1 is {:?}", rect1);
// }


// Пример 3

// Другой способ распечатать значение в формате Debug — использовать макрос dbg!, который становится
// владельцем выражения (в отличие от println!, принимающего ссылку), печатает номер файла и строки,
// где происходит вызов макроса dbg!, вместе с результирующим значением этого выражения и возвращает
// владение на значение.

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//
//     dbg!(&rect1);
// }