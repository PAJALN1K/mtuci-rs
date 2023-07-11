// Пример 1

// Открытие файла

// File::open возвращает значения типа Result<T, E>. Универсальный тип T в реализации File::open 
// соответствует типу успешно полученного значения, std::fs::File, а именно дескриптору файла. Тип E, 
// используемый для значения в случае возникновения ошибки, - std::io::Error.

// В случае успеха File::open значением переменной greeting_file_result будет экземпляр Ok, содержащий 
// дескриптор файла. В случае неудачи значение в переменной greeting_file_result будет экземпляром Err, 
// содержащим дополнительную информацию о том, какая именно ошибка произошла.

// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");
// }


// Пример 2

// Использование выражения match для обработки возвращаемых вариантов типа Result

// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }


// Пример 3

// Обработка различных ошибок разными способами

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }


// Пример 4

// Функция, которая возвращает ошибки в вызывающий код, используя оператор match

// Метод unwrap - это метод быстрого доступа к значениям, реализованный так же, как и выражение match. 
// Если значение Result является вариантом Ok, unwrap возвращает значение внутри Ok. Если Result - 
// вариант Err, то unwrap вызовет для нас макрос panic!. 

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }

// Если мы запустим этот код при отсутствии файла hello.txt, то увидим сообщение об ошибке из вызова 
// panic! метода unwrap.

// Другой метод, похожий на unwrap, это expect, позволяющий указать сообщение об ошибке для макроса panic!. 

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");
// }


// Пример 5

// Функция, которая возвращает ошибки в вызывающий код, используя оператор match

// Если эта функция выполнится без проблем, то код, вызывающий эту функцию, получит значение Ok, 
// содержащее String - имя пользователя, которое эта функция прочитала из файла. Если функция 
// столкнётся с какими-либо проблемами, вызывающий код получит значение Err, содержащее экземпляр 
// io::Error, который включает дополнительную информацию о том, какие проблемы возникли.

// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// }

// Затем код, вызывающий этот, будет обрабатывать получение либо значения Ok, содержащего имя 
// пользователя, либо значения Err, содержащего io::Error. Вызывающий код должен решить, что делать 
// с этими значениями. Если вызывающий код получает значение Err, он может вызвать panic! и завершить 
// работу программы, использовать имя пользователя по умолчанию или найти имя пользователя, например, 
// не в файле. У нас недостаточно информации о том, что на самом деле пытается сделать вызывающий код, 
// поэтому мы распространяем всю информацию об успехах или ошибках вверх, чтобы она могла обрабатываться 
// соответствующим образом.

// Эта схема передачи ошибок настолько распространена в Rust, что Rust предоставляет оператор 
// вопросительного знака ?, чтобы облегчить эту задачу.


// Пример 6

// Функция, возвращающая ошибки в вызывающий код с помощью оператора ?

// В примере 6 показана реализация read_username_from_file, которая имеет ту же функциональность, 
// что и в примере 5, но в этой реализации используется оператор ?.

// Выражение ?, расположенное после Result, работает почти так же, как и те выражения match, которые 
// мы использовали для обработки значений Result в примере 5. Если в качестве значения Result будет 
// Ok, то значение внутри Ok будет возвращено из этого выражения, и программа продолжит работу. Если же 
// значение представляет собой Err, то Err будет возвращено из всей функции, как если бы мы использовали 
// ключевое слово return, так что значение ошибки будет передано в вызывающий код.

// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
// }


// Пример 7

// Цепочка вызовов методов после оператора ?

// Оператор ? позволяет избавиться от большого количества шаблонного кода и упростить реализацию этой 
// функции. Мы могли бы даже ещё больше сократить этот код, если бы использовали цепочку вызовов методов 
// сразу после ?

// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }
// }


// Пример 8

// Использование fs::read_to_string вместо открытия и последующего чтения файла

// Чтение файла в строку довольно распространённая операция, так что стандартная библиотека предоставляет 
// удобную функцию fs::read_to_string, которая открывает файл, создаёт новую String, читает содержимое 
// файла, размещает его в String и возвращает её. Конечно, использование функции fs::read_to_string не 
// даёт возможности объяснить обработку всех ошибок, поэтому мы сначала изучили длинный способ.

// #![allow(unused)]
// fn main() {
// use std::fs;
// use std::io;

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
// }


// Пример 9

// Попытка использовать ? в main функции, которая возвращает () , не будет компилироваться

// Оператор ? может использоваться только в функциях, тип возвращаемого значения которых совместим со 
// значением, для которого используется ?.

// #![allow(unused)]
// fn main() {
// use std::fs;
// use std::io;

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }

// Для исправления ошибки есть два варианта. Первый - изменить возвращаемый тип вашей функции так, 
// чтобы он был совместим со значением, для которого вы используете оператор ?, если у вас нет 
// ограничений, препятствующих этому. Другой способ - использовать match или один из методов 
// Result<T, E> для обработки Result<T, E> любым подходящим способом.


// Пример 10

// Использование оператора ? для значения Option<T>

// ? можно использовать и со значениями Option<T>.

// Обратите внимание, что вы можете использовать оператор ? Result в функции, которая возвращает Result, 
// и вы можете использовать оператор ? для Option в функции, которая возвращает Option , но вы не можете 
// смешивать и сопоставлять Option и Result.

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}


// Пример 11

// Замена main на return Result<(), E> позволяет использовать оператор ? оператор над значениями Result

// Тип Box<dyn Error> является трейт-объектом. Пока что вы можете считать, что Box<dyn Error> означает 
// "любой вид ошибки".

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }
