// Условные конструкции if

// Пример 1
// Блоки кода, связанные с условиями в выражениях if, иногда называются ответвлениями (вспомни match)
//
// fn main() {
//     let number = 3;
//
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


// Пример 2

// Также стоит отметить, что условие в этом коде должно быть логического типа bool.
// В отличии от других языков вроде Ruby и JavaScript,
// Rust не будет пытаться автоматически конвертировать нелогические типы в логические.
//
// fn main() {
//     let number = 3;
//
//     // Если вписать просто number, то код не сработает. Зато выражение вроде number != 0 подойдет
//     if number {
//         println!("number was three");
//     }
// }


// Пример 3

// В этом примере будет выведено "number is divisible by 3". Третье условие не сработает
// Rust выполняет блок только для первого истинного условия, а обнаружив его, даже не проверяет остальные.
//
// Использование множества выражений else if приводит к загромождению кода, поэтому
// при наличии более чем одного выражения, возможно, стоит провести рефакторинг кода.
// В главе 6 описана мощная конструкция ветвления Rust для таких случаев, называемая match.
//
// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// Пример 4

// Поскольку if является выражением, его можно использовать в правой части оператора let для присвоения результата переменной
//
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("The value of number is: {number}");
// }


// Пример 5

// В этом случае выведет ошибку, так как 5 и "six" не являются одним и тем же типом данных.
// Так работает Rust. Компилятор усложнился бы и давал бы меньше гарантий в отношении кода,
// если бы ему приходилось отслеживать несколько гипотетических типов для любой переменной.
//
// fn main() {
//     let condition = true;
//
//     let number = if condition { 5 } else { "six" };
//
//     println!("The value of number is: {number}");
// }



// Циклы

// Пример 1

// Ключевое слово loop говорит Rust выполнять блок кода снова и снова до бесконечности или пока не будет явно приказано остановиться.
// Большинство терминалов поддерживают комбинацию клавиш ctrl-c для прерывания программы, которая застряла в непрерывном цикле.

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// break - завершить цикл.
// continue - пропустить продолжение цикла и перейти к следующей итерации.


// Пример 2

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Пример x

// fn main() {
//     let x = 10;
//
//     let y = {
//         if x < 100 {
//             x + 100
//         } else {
//             x - 100
//         }
//     };
//
//     println!("x is {}, y is {}", x, y)
// }

// Пример 3

// Если у вас есть циклы внутри циклов, break и continue применяются к самому внутреннему
// циклу в этой цепочке. При желании вы можете создать метку цикла, которую вы затем сможете
// использовать с break или continue для указания, что эти ключевые слова применяются к помеченному
// циклу, а не к самому внутреннему циклу. Метки цикла должны начинаться с одинарной кавычки.

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {     // 'counting_up - и есть метка
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;      // после этой строчки закончится внешний цикл.
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
// }


// Пример 4

// Цикл while - он и в Rust цикл while

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }


// Пример 5

// Цикл for. Безопасность и компактность циклов for делают их наиболее часто используемой
// конструкцией цикла в Rust.

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a {
//         println!("the value is: {element}");
//     }
// }


// Пример 6

// (1..4) - 1, 2, 3
// (1..=4) - 1, 2, 3, 4

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }