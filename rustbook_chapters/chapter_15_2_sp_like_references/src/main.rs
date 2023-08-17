// Пример 1

// Использование оператора разыменования для следования по ссылке к значению i32

// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// Пример 2

// Использование оператора разыменования с типом Box<i32>

// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// Пример 3

// Определение типа MyBox<T>

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// fn main() {}


// Пример 4

// Попытка использовать MyBox<T> таким же образом, как мы использовали ссылки и Box<T>. 
// Не скомпилируется

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// Пример 5

// Реализация Deref для типа MyBox<T>

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// Пример 6

// Функция hello имеющая параметр name типа &str

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {}


// Пример 7

// Вызов hello со ссылкой на значение MyBox<String>, которое работает из-за разыменованного приведения

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }


// Пример 8

// Код, который нам пришлось бы написать, если бы в Rust не было разыменованного приведения ссылок

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&(*m)[..]);
// }
