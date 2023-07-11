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



