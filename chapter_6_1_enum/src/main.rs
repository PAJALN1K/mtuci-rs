// мы рассмотрим перечисления (enumerations), также называемые enums.
// Перечисления позволяют определить тип путём перечисления его возможных вариантов.

// Там, где структуры дают вам возможность группировать связанные поля и данные, например Rectangle
// с его width и height, перечисления дают вам способ сказать, что значение является одним из
// возможных наборов значений. Например, мы можем захотеть сказать, что Rectangle — это одна из
// множества возможных фигур, в которую также входят Circle и Triangle. Для этого Rust позволяет
// нам закодировать эти возможности в виде перечисления.

// Пример 1

// Можно выразить эту концепцию в коде, определив перечисление IpAddrKind и составив список
// возможных видов IP-адресов, V4 и V6.

// enum IpAddrKind {
//     V4,
//     V6,
// }
//
//
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }
//
// fn route(ip_kind: IpAddrKind) {}
//


// Пример 2

// Сохранение данных и IpAddrKind IP-адреса с использованием struct

// enum IpAddrKind {
//         V4,
//         V6,
//     }
//
// struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }


// Сохранение данных и IpAddrKind IP-адреса с использованием enum

#[derive(Debug)]
enum IpAddr {
        V4(String),
        V6(String),
    }

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}


// Пример 4

// При необходимости сохранить адреса типа V4 как четыре значения типа u8, а также описать адреса
// типа V6 как единственное значение типа String, мы не смогли бы с помощью структуры.
// Перечисления решают эту задачу легко:

// enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//
// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//
//     let loopback = IpAddr::V6(String::from("::1"));
// }


// Пример 5

// Однако, как оказалось, желание хранить IP-адреса и указывать их тип настолько распространено,
// что в стандартной библиотеке есть определение, которое мы можем использовать!

// struct Ipv4Addr {
//     // --snip--
// }
//
// struct Ipv6Addr {
//     // --snip--
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }


// Пример 6

//  Перечисление Message, в каждом из вариантов которого хранятся разные количества и типы значений.

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

//  То же самое, но через структуры

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Но когда мы использовали различные структуры, которые имеют свои собственные типы, мы не могли
// легко определять функции, которые принимают любые типы сообщений, как это можно сделать с
// помощью перечисления типа Message, объявленного в листинге 6-2, который является единым типом.


// Пример 7

// Есть ещё одно сходство между перечислениями и структурами: так же, как мы можем определять
// методы для структур с помощью impl блока, мы можем определять и методы для перечисления.

// impl Message {
//         fn call(&self) {
//             // method body would be defined here
//         }
//     }
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// Option

// Перечисление Option<T> настолько полезно, что оно даже включено в прелюдию; вам не нужно явно
// вводить его в область видимости. Его варианты также включены в прелюдию: вы можете использовать
// Some и None напрямую, без префикса Option::. При всём при этом, Option<T> является обычным
// перечислением, а Some(T) и None представляют собой его варианты.

// Пример 8

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');
//
//     let absent_number: Option<i32> = None;
// }

// Пример 9

// Вкратце, поскольку Option<T> и T (где T может быть любым типом) относятся к разным типам,
// компилятор не позволит нам использовать значение Option<T> даже если бы оно было определённо
// допустимым вариантом Some.

// Запуск данного кода даст ошибку.

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//
//     let sum = x + y;
// }

// Сильно! Фактически, выведенное сообщение об ошибке будет означать, что Rust не понимает, как
// сложить i8 и Option<i8>, потому что это разные типы.

// В общем случае, чтобы использовать значение Option<T>, нужен код, который будет обрабатывать все
// варианты перечисления Option<T>.