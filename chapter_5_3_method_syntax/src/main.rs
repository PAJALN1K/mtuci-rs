// Методы похожи на функции: мы объявляем их с помощью ключевого слова fn и имени, они могут иметь
// параметры и возвращаемое значение, и они содержат код, запускающийся в случае вызова метода.
// В отличие от функций, методы определяются в контексте структуры
// (или перечисления, или типаж-объекта), и их первым параметром всегда является self,
// представляющий собой экземпляр структуры, на которой вызывается этот метод.


// Пример 1

// Определение метода area для структуры Rectangle

// Чтобы определить функцию в контексте Rectangle, мы создаём блок impl (implementation - реализация)
// для Rectangle. Всё в impl будет связано с типом Rectangle.
// В сигнатуре area мы используем &self вместо rectangle: &Rectangle.
// &self на самом деле является сокращением от self: &Self.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


// Пример 2

// Обратите внимание, что мы можем дать методу то же имя, что и одному из полей структуры.
// Например, для Rectangle мы можем определить метод, также названный width.

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }


// Пример 3

// Использование метода can_hold для Rectangle, принимающего другой экземпляр Rectangle в качестве
// параметра

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };
//
//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// Пример 4

// Все функции, определённые в блоке impl, называются ассоциированными функциями, потому что они
// ассоциированы с типом, указанным после ключевого слова impl. Мы можем определить ассоциированные
// функции, которые не имеют self в качестве первого параметра (и, следовательно, не являются
// методами), потому что им не нужен экземпляр типа для работы.

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }


// Пример 5

// Переписанный пример 3 с использованием нескольких impl

// Каждая структура может иметь несколько impl.

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
