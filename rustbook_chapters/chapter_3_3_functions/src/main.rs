// Пример 1

// fn - объявление функции
// main() - название и параметры функции
// {} - тело функции
// Где функция находится - выше или ниже main - не важно, главное, чтобы они были определены
// где-то в той области видимости, которую может видеть вызывающий их код.

// fn main() {
//     println!("Hello, world!");
//
//     another_function();
// }
//
// fn another_function() {
//     println!("Another function.");
// }


// Пример 2

// Передача параметров функции another_function).
// Чисто технически правильнее говорить аргумент, но в неформальной беседе эти два понятия, аргумент и параметр, становятся взаимозаменяемыми.
// Объявить тип каждого параметра функции обязательно.

// fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }


// Пример 3

// При определении нескольких параметров, нужно разделять объявления параметров запятыми

// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


// Пример 4

// Операторы - это инструкции, которые выполняют какое-либо действие и не возвращают значение.
// Выражения вычисляют результирующее значение.
// Создание переменной и присвоение ей значения с помощью let - это оператор.
// Определения функций также являются операторами.

// fn main() {
//     let y = 6; // Оператор let y = 6 не возвращает значение, поэтому не с чем связать переменную x.
// }

// Выражения вычисляют значение и составляют большую часть остального кода, который вы напишете на Rust.
// Выражения могут быть частью операторов: 6 в операторе let y = 6;
// Вызов функции - это выражение. Вызов макроса - это выражение.


// Пример 5

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {y}");
// }

// {let x = 3; x + 1} - выражение.
// Следует обратить внимание: выражения не имеют в конце знака ";"
// Если этот знак будет - это уже будет оператор, и значение не вернется.


// Пример 6

// Функции могут возвращать значения коду, который их вызывает.
// Мы не называем возвращаемые значения, но мы должны объявить их тип после стрелки ( -> ).

// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {x}");
// }


// Пример 7

// (правильный)

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// (неправильный): попросит убрать точку с запятой.

// fn main() {
//     let x = plus_one(5);
//
//     println!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }
