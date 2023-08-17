// Пример 1

// Объявление cons list, который содержит RefCell<T>, чтобы мы могли изменять то, на что 
// ссылается экземпляр Cons

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {}


// Пример 2

// Создание ссылочного цикла из двух значений List, указывающих друг на друга

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// // интересует эта часть
// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
// }


// Пример 3

// Создание узла leaf без дочерних элементов и узла branch с leaf в качестве 
// одного из дочерних элементов

// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         children: RefCell::new(vec![]),
//     });

//     let branch = Rc::new(Node {
//         value: 5,
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
// }


// Пример 4

// Узел leaf со слабой ссылкой на его родительский узел branch

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// // эта часть интересует.
// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });

//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// }


// Пример 5

// Создание branch во внутренней области видимости и подсчёт сильных и слабых ссылок

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Эта часть интересует
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

