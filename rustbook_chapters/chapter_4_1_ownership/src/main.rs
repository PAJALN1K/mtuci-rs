// fn main() {
//     // Пример 1
//
//     //  {                      // s is not valid here, it’s not yet declared
//     //     let s = "hello";   // s is valid from this point forward
//     //
//     //     // do stuff with s
//     // }                      // this scope is now over, and s is no longer valid
//
//
//     // Пример 2
//
//     // Также можно создать String из строкового литерала, используя функцию from.
//     // Оператор "Двойное двоеточие" :: позволяет использовать пространство
//     // имён данной конкретной функции from с типом String.
//
//     // Строка такого типа может быть изменяема
//
//     // let mut s = String::from("hello");
//     //
//     // s.push_str(", world!"); // push_str() appends a literal to a String
//     //
//     // println!("{}", s); // This will print `hello, world!`
//
//     // Почему строку String можно изменить, а литералы — нельзя? Разница заключается в том, как эти два типа работают с памятью.
//
//
//     // Пример 3
//
//     // В случае с целыми числами будут созданы две переменные с одним и тем же значением 5.
//     // Память о них сохранится в стеке: у них определенный объем памяти.
//
//     // let x = 5;
//     // let y = x;
//
//     // Для типа данных string дело обстоит иначе: у них не определен объем памяти на каждое значение.
//     // При создании двух переменных, как указано ниже, первая переменная s1 исчезнет, для того
//     // чтобы не загружать память.
//
//     // При запуске кода выведет ошибку: s1 переместится в s2, и вызвать ее не получится.
//
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     //
//     // println!("{}, world!", s1);
//
//
//     // Пример 3
//
//     // Пример работы с clone - глубоким копированием данных.
//     // Когда вы видите вызов clone, вы знаете о выполнении некоторого кода, который может быть дорогим.
//
//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();
//     //
//     // println!("s1 = {}, s2 = {}", s1, s2);
// }


// Пример 4
//
// fn main() {
//     let s = String::from("hello");  // s comes into scope
//
//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//
//     let x = 5;                      // x comes into scope
//
//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward
//
//
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.
//
// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.
//
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// Пример 5

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
