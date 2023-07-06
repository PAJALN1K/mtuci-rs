// В Rust есть чрезвычайно мощный механизм управления потоком, именуемый match, который позволяет
// сравнивать значение с различными шаблонами и затем выполнять код в зависимости от того, какой из
// шаблонов совпал. Сила match заключается в выразительности шаблонов и в том, что компилятор
// проверяет, что все возможные случаи обработаны.


// Пример 1

// Перечисление и выражение match, использующее в качестве шаблонов его варианты

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// Обычно фигурные скобки не используются, если код совпадающей ветви невелик, как в листинге 6-3,
// где каждая ветвь просто возвращает значение. Если вы хотите выполнить несколько строк кода в
// одной ветви, вы должны использовать фигурные скобки, а запятая после этой ветви необязательна.

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }



// Есть ещё одно полезное качество у веток в выражении match: они могут привязываться к частям тех
// значений, которые совпали с шаблоном. Благодаря этому можно извлекать значения из вариантов перечисления.

// Пример 2

// Перечисление Coin, в котором вариант Quarter также сохраняет значение UsState

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// В выражении match для этого кода мы добавляем переменную с именем state в шаблон, который
// соответствует значениям варианта Coin::Quarter. Когда Coin::Quarter совпадёт с шаблоном,
// переменная state будет привязана к значению штата этого четвертака.

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }


// Пример 3

// Допустим, мы хотим написать функцию, которая принимает Option<i32> и если есть значение внутри,
// то добавляет 1 к существующему значению. Если значения нет, то функция должна возвращать
// значение None и не пытаться выполнить какие-либо операции.

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


// Пример 4

// Есть ещё один аспект match, который мы должны обсудить: шаблоны должны покрывать все возможные
// варианты. Этот код не компилируется.
// Сопоставления в Rust являются исчерпывающими: мы должны покрыть все возможные варианты,
// чтобы код был корректным.

// fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             Some(i) => Some(i + 1),
//         }
//     }


// Пример 5

// Эти два кода одинаковы

// let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => reroll(),
//     }
//
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}

// let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
//
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}



// let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }
//
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
