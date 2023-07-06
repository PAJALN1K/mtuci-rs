// Пример 1

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


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

// У нас также не может быть изменяемой ссылки, пока у нас есть неизменяемая ссылка на то же значение.
// Однако стоит обратите внимание, что область действия ссылки начинается с того места,
// где она была введена, и продолжается до последнего использования этой ссылки.
// Этот код скомпилируется без ошибки (если только не раскомментировать строку после объявления
// переменной r3).

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//     //println!("{} and {}", r1, r2); // если ввести эту строку, то возникнет ошибка.
//     println!("{}", r3);
// }


// Пример 7

// В языках с указателями весьма легко ошибочно создать недействительную, висячую (dangling) ссылку.

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

// Поскольку s создаётся внутри dangle, когда код dangle будет завершён, s будет освобождена.
// Но мы попытались вернуть ссылку на неё. Это означает, что эта ссылка будет указывать
// на недопустимую String. Это нехорошо! Rust не позволит нам сделать это.

// Код без ошибок

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");
//
//     s
// }