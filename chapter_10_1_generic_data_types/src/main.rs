// Каждый язык программирования имеет в своём арсенале эффективные средства борьбы 
// с дублированием кода. В Rust одним из таких инструментов являются обобщённые 
// типы данных - generics. Это абстрактные подставные типы на место которых возможно 
// поставить какой-либо конкретный тип или другое свойство.

// На самом деле мы уже использовали обобщённые типы данных в Главе 6 (Option<T>), 
// в Главе 8 (Vec<T> и HashMap<K, V>) и в Главе 9 (Result<T, E>).


// Пример 1

// Вариант 1

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// Вариант 2

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }


// как удалить дублирование путём извлечения функции, которая заменяет определённые значения 
// заполнителем, представляющим несколько значений.

// 1. Определить дублирующийся код.
// 2. Извлечь дублирующийся код и поместить его в тело функции, определив входные и выходные 
// значения этого кода в сигнатуре функции.
// 3. Обновить и заменить два участка дублирующегося кода вызовом одной функции.


// Пример 3

// две функции, отличающиеся только именем и типом обрабатываемых данных

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }


// Пример 4

// функция largest, использующая параметры обобщённого типа; пока ещё не компилируется

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }


// Пример 5

// структура Point, содержащая поля x и y типа T

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }


// Пример 6

// поля x и y должны быть одного типа, так как они имеют один и тот же 
// обобщённый тип T, не скомпилируется

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }


// Пример 7

// структура Point<T, U> обобщена для двух типов, так что x и y могут быть 
// значениями разных типов

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }


// Пример 8

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// enum Option<T> {
//     Some(T),
//     None,
// }


// Пример 9

// Реализация метода с именем x у структуры Point<T>, которая будет 
// возвращать ссылку на поле x типа T

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }


// Пример 10

// блок impl, который применяется только к структуре, имеющей 
// конкретный тип для параметра обобщённого типа T

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


// Пример 11

// метод, использующий обобщённые типы, отличающиеся от типов, 
// используемых в определении структуры

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


// Пример 12

// Два одинаковых кода

// 1

// fn main() {
//     let integer = Some(5);
//     let float = Some(5.0);
// }

// 2

// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

// fn main() {
//     let integer = Option_i32::Some(5);
//     let float = Option_f64::Some(5.0);
// }
