// Боксы

// fn main() {
//     let a = Box::new(5);
//     let a1 = A::new();
// }
// // Бокс гарантирует, что данные, лежащие в нем, находятся в куче

// // Рекурсивный тип А хранит бесконечный размер.
// // struct A {
// //     x: A,
// // }


// // Это уже ссылка на структуру, запустится
// // Бокс - посредник, указатель. А хранит число, и указатель на другое А
// struct A {
//     x: Box<A>,
//     y: i32
// }



// Rc

// Вариант 1


// fn main() {
//     let v = vec![1i32, 2, 3, 4];
//     let a = A {vec: v};
//     let d = A {vec: v};
// }

// struct A {
//     vec: <Vec<i32>>
// }


// Вариант 2

// use std::rc::Rc;

// fn main() {
//     let v = Rc::new(vec![1i32, 2, 3, 4]);
//     let a = A {vec: v.clone()};
//     let d = A {vec: v.clone()};
// }

// struct A {
//     vec: Rc<Vec<i32>>,
// }

// struct MyRc {
//     counter: usize,
//     pointer: *const i32,
// }

// Данные внутри Rc всегда одинаковые. Это сделано с целью того, чтобы избежать 
// гонок типов.

// use std::rc::Rc;
// use std::cell::RefCell;

// fn main() {
//     let v = Rc::new(vec![1i32, 2, 3, 4]);
//     let i = RefCell::new(0);
//     let bm = i.borrow_mut();
//     i.borrow();
    
//     let a = A {vec: v.clone()};
//     let b = A {vec: v.clone()};
// }

// struct A {
//     vec: Rc<Vec<i32>>,
// }

// struct MyRc {
//     counter: usize,
//     pointer: *const i32,
// }


use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut vec = vec![1i32, 2, 3, 4];
    let v = Rc::new(RefCell::new(vec![1,2,3,4]));
    let mut a = A { vec: v.clone() };
}

struct A {
    vec: Rc<Vec<i32>>,
}

struct MyRc {
    counter: usize,
    pointer: *const i32,
}

// до 15 включительно