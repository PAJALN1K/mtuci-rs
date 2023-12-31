// Пример 1

// Если вы попытаетесь скомпилировать этот код, вы получите следующую ошибку:

// fn main() {
//     let x = 5;
//     let y = &mut x;
// }


// Пример 2

// Библиотека для отслеживания степени приближения того или иного значения к максимально 
// допустимой величине и предупреждения, в случае если значение достигает определённого уровня

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }


// Пример 3

// Попытка реализовать MockMessenger, которая не была принята механизмом проверки заимствований.
// Не скомпилируется.

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: vec![],
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }

// Пример 4

// Использование RefCell<T> для изменения внутреннего значения, в то время как внешнее значение 
// считается неизменяемым

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }

// // вот эта часть интересует
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.borrow_mut().push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         // --snip--
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }


// Пример 5

// Создание двух изменяемых ссылок в одной области видимости, чтобы убедиться, что RefCell<T> 
// вызовет панику

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }
//     // вот этот импл - и есть виновник паники.
//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             let mut one_borrow = self.sent_messages.borrow_mut();
//             let mut two_borrow = self.sent_messages.borrow_mut();

//             one_borrow.push(String::from(message));
//             two_borrow.push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }

// Пример 6

// Использование Rc<RefCell<i32>> для создания List, который мы можем изменять

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

