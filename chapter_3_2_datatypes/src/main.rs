// Пример 1
//
// С помощью .parse() мы можем переделать строку в число, например, u32
//
// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("{}", guess)
// }


// Пример 2

// В целом, классические операции с числами на Rust не сильно отличаются от остальных языков. Однако вызывает интерес операция деления.

// fn main() {
//     // addition
//     let sum = 5 + 10;
//
//     // subtraction
//     let difference = 95.5 - 4.3;
//
//     // multiplication
//     let product = 4 * 30;
//
//     // division
//     let quotient = 56.7 / 32.2;                  // Операнд "/" работает как обычное деление, если числа представлены в дробном виде
//     let truncated = -5 / 3; // Results in -1     // Операнд "/" работает как целочисленное деление, если числа представлены в целом виде
//
//     // remainder
//     let remainder = 43 % 5; // Деление с остатком
//
//     println!("sum is {}", sum);
//     println!("difference is {}", difference);
//     println!("product is {}", product);
//     println!("quotient is {}", quotient);
//     println!("truncated is {}", truncated);
//     println!("remainder is {}", remainder);
// }


// Пример 3

// Дробные числа - f64 и f32. f64 более точный.

// fn main() {
//     let x = 2.0; // f64
//
//     let y: f32 = 3.0; // f32
// }


// Пример 4

// bool - логический тип. Занимает один байт.

// fn main() {
//     let t = true;
//
//     let f: bool = false; // with explicit type annotation
// }


// Пример 5

// char - "самый примитивный алфавитный тип языка". Пишется через одинарные кавычки.

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }


// Пример 6

// Кортеж - способ хранить значения различных типов данных в одном месте. Тип данных. Его длина неизменна.

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1); // Аннотация необязательна
// }


// Пример 7

// Присваивание переменным значений с помощью кортежа.

// fn main() {
//     let tup = (500, 6.4, 1);
//
//     let (x, y, z) = tup;
//
//     println!("The value of y is: {y}");
// }


// Пример 8

// Обращение к элементам кортежа

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//
//     let five_hundred = x.0;
//
//     let six_point_four = x.1;
//
//     let one = x.2;
// }


// Пример 9

// Массив - способ хранить значения одного типа данных в одном месте. Тип данных. Его длина неизменна.

// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }


// Пример 10

// Массив будет идеальным способом для, например, хранения названий месяцев (их число неизменно).

// fn main() {
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
// }


// Пример 11

// Аннотация массива выглядит следующим образом:

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
// }


// Пример 12

// В этом примере создастся массив a = [3, 3, 3, 3, 3]
// fn main() {
//     let a = [3; 5];
// }


// Пример 13

// Обращение к элементам массива. В этом примере first - 1, second - 2.

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     let first = a[0];
//     let second = a[1];
// }


// Пример 14

// В этом примере пользователь должен ввести индекс элемента массива, а программа вернет элемент с этим индексом.
// В случае, если значение индекса будет больше длины массива или равно ей, то программа вызовет Панику.

// use std::io;
//
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//
//     let element = a[index];
//
//     println!("The value of the element at index {index} is: {element}");
// }
