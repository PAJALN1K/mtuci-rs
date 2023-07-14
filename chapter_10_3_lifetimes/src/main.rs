// Пример 1

// Попытка использования ссылки, значение которой вышло из области видимости

// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }


// Пример 2

// Аннотация времён жизни переменных r и x, с помощью идентификаторов времени 
// жизни 'a и 'b, соответственно. Не компилируется

// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+


// Пример 3

// Ссылка корректна, так как данные имеют более продолжительное время жизни, 
// чем ссылка на эти данные

// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+


// Пример 4

// Функция main вызывает функцию longest для поиска наибольшего из двух срезов строки

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }


// Пример 5

// Реализация функции longest, которая возвращает наибольший срез строки, 
// но пока не компилируется

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// Пример 6

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


// Пример 7

// В определении функции longest указано, что все ссылки должны иметь 
// одинаковое время жизни, обозначенное как 'a

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// Пример 8

// Использование функции longest со ссылками на значения типа String, 
// имеющими разное время жизни

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }


// Пример 9

// Попытка использования result, после того как string2 вышла из области видимости

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }


// Пример 10

// При возврате ссылки из функции, параметр времени жизни для возвращаемого типа должен соответствовать 
// параметру времени жизни одного из аргументов. Если возвращаемая ссылка не ссылается на один из 
// параметров, она должна ссылаться на значение, созданное внутри функции. Однако, это приведёт к 
// недействительной ссылке, поскольку значение, на которое она ссылается, выйдет из области 
// видимости в конце функции. Посмотрите на попытку реализации функции longest, которая не 
// скомпилируется:

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }


// Пример 11

// Структура, содержащая ссылку, требует аннотации времени жизни

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}


// Пример 12

// Функция, которую мы определили в листинге 4-9 компилируется без аннотаций времени жизни, 
// несмотря на то, что входной и возвращаемый тип параметров являются ссылками

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


// Пример 13

// Структура, содержащая ссылку, требует аннотации времени жизни

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }