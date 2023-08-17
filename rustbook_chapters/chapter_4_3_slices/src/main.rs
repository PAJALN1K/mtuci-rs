// Пример 1

// Вывод первого слова (без срезов)

// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s); // word will get the value 5
//
//     s.clear(); // this empties the String, making it equal to ""
//
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }

// fn first_word(s: &String) -> usize {
//     iter — это метод, который возвращает каждый элемент в коллекции, а enumerate оборачивает
//     результат iter и вместо этого возвращает каждый элемент как часть кортежа. Первый элемент
//     кортежа, возвращаемый из enumerate, является индексом, а второй элемент — ссылкой на элемент.
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }


// Пример 2

// Строковый срез - это ссылка на часть строки String

// fn main() {
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     let world = &s[6..11];
// }


// Пример 3

// Вывод первого слова (срезы)

// fn main() {
//     let s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     // s.clear(); // error!
//
//     println!("the first word is: {}", word);
// }
//
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }

// Пример 4

// fn main() {
//     let my_string = String::from("hello world");
//
//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);
//
//     let my_string_literal = "hello world";
//
//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);
//
//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }


// Примеры 5

// Эти примеры показывают, что два варианта написания кода дают о.и.т.ж. результат

// fn main() {
//     let s = String::from("hello");
//
//     let slice = &s[0..2];
//     let slice = &s[..2];
// }

// fn main() {
//     let s = String::from("hello");
//
//     let len = s.len();
//
//     let slice = &s[3..len];
//     let slice = &s[3..];
// }

// fn main() {
//     let s = String::from("hello");

//     let len = s.len();

//     let slice = &s[0..len];
//     let slice = &s[..];
// }


// Пример 6

// Срезы для массивов

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
// let slice = &a[1..3];
//
// assert_eq!(slice, &[2, 3]);
// }

fn main() {
    println!("AHAHAHAHHAHAH you're too late, Sonic!\nNow I'm a cringe string slice!");
}
