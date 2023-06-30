// Пример 1

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// Пример 2

// Изменение ссылки изменяет начальный объект

// fn main() {
//     let s = String::from("hello");
//
//     change(&s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// Пример 3

// Изменяемые ссылки имеют одно большое ограничение:
// если у вас есть изменяемая ссылка на значение, у вас не может быть других ссылок на это же значение.

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     println!("{}, {}", r1, r2);
// }


// Пример 4

// fn main() {
//     let mut s = String::from("hello");
//
//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.
//
//     let r2 = &mut s;
//
//     println!("{}, {}", r1, r2);
// }


// Пример 5

// У нас также не может быть изменяемой ссылки, пока у нас есть неизменяемая ссылка на то же значение.

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//
//     println!("{}, {}, and {}", r1, r2, r3);
// }


// Пример 6

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }


// Пример 7

// Код с ошибкой

// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!


// Код без ошибок

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");
//
//     s
// }