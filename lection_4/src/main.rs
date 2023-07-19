// Каждый должен реализовать собственный вектор, не используя обычный 

// Строки хранятся в UTF-8, поэтому к ним почти ни за что нельзя 
// обращаться по индексу или срезу


// Тема 1

// *Что???*

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<u32, String> = HashMap::new();

//     let res = map.insert("1", "one".to_string());
//     println!("{:?}", res);

//     let res = map.insert("1", "two".to_string());
//     println!("{:?}", res);
// }


// Тема 2

// Паника

// fn main() {
//     // panic!("ETO PANIC") // вызов макроса паники самостоятельно

//     let x: Option<u32> = None;
//     x.unwrap();
    
//     // println!("AHAHAHAHHAHAH you're too late, Sonic!\nNow I'm a cringe string slice!");
// }


// Тема 3

// Обработка неисправимых ошибок

// fn main() {
//     let x: Option<u32> = None;
//     match x {
//         Some(q) => todo!();
//         None => {
//             println!("ЭТО КОНЕЦ");
//             panic!("qweqwe");
//         }
//     }
// }


// Тема 4

// Обработка исправимых ошибок

// use std::io;

// fn main() {
//     match get_input() {
//         Ok(_) => todo!(),
//         Err(_) => todo!(),
//     }
// }


// // ? - перекладывание ответственности на другого.
// // Если что-то произошло неправильно, ошибки отправляются в верх по стеку.
// fn get_input() -> Result<i32, Box<dyn Error>> {
//     let mut read: String = String::new();
//     io::stdin().read_line(&mut read).unwrap()?;
//     let read: i32 = read.trim().parse().unwrap()?;
//     let x = read.parse::<i32>()?;
//     Ok(x)
// }


// Тема 5

// Пример 1

// generic

// generic - generate. Что-то, что во время компиляции будет
// генерировать нам код. generic - обобщенный тип.

// fn main() {
//     let i = vec![1, 2, 3, 4, 5, 6, 7];
//     let u: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];

//     println!("{}", largest_number(i));
//     println!("{}", largest_number(u));
// }


// fn largest_number(nums: Vec<T>) -> &T {
//     nums.iter().max().unwrap().clone()
// }


// Пример 2

// use std::ops::MulAssign;

// f64 - по умолчанию для дробных
// i32 - по умолчанию для целых
// fn main() {
//     let p: Point { x: 10, y: 15 };
//     let p2: Point { x: 10, y: 15. };
//     let p3: Point { 
//         x: String::from("10"), 
//         y: String::from("15"). 
//     };
//     p2.clone();
//     p3.clone();
// }


// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<T> 
// where 
//     T: Copy,
// {
//     fn clone(&self) -> T
//     where
//         T: Clone,
//     {
//         self.x.copy()
//     }
// }

// Это тоже работает
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// enum qwer<T, U> {
//     A(T),
//     B(U),
//     C(T, U),
//     D
// }


// Тема 6

// lifetimes, времена жизни

fn main() {
    let mut s = String::new();
}

// Написать собственную реализацию вектора (add, pop)
// Вектор - это единый блок, последовательность адресов
// Связный список удобен, когда гарантируется, что нет единого массива
// данных
// * Написать собственную реализацию hash map
// до вторника

// hash - группа данных, которым дают данные, и они по ним выдают 
// специальный номер

