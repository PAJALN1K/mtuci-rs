// Пример 1

// Ситуация с раздачей рубашек компанией

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }


// Пример 2

// Добавление необязательных аннотаций типов параметров и возвращаемых значений в замыкании

// use std::thread;
// use std::time::Duration;

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }


// Пример 3

// Попытка вызова замыкания, типы которого выводятся из двух разных типов. Не скомпилируется.

// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));
//     let n = example_closure(5);
// }


// Пример 4

// Определение и вызов замыкания, которое захватывает неизменяемую ссылку

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let only_borrows = || println!("From closure: {:?}", list);

//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }


// Пример 5

// Определение и вызов замыкания, захватывающего изменяемую ссылку

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let mut borrows_mutably = || list.push(7);

//     borrows_mutably();
//     println!("After calling closure: {:?}", list);
// }


// Пример 6

// Использование move для принуждения замыкания потока принять на себя владение list

// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     thread::spawn(move || println!("From thread: {:?}", list))
//         .join()
//         .unwrap();
// }


// Пример 7

// Использование sort_by_key для сортировки прямоугольников по ширине

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }


// Пример 8

// Попытка использовать замыкание FnOnce с sort_by_key

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("by key called");

//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{:#?}", list);
// }


// Пример 9

// Использование замыкания FnMut с sort_by_key разрешено

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}