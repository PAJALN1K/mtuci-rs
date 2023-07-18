// Пример 1

// use std::time::Duration;

// fn main() {
//     let s = "thread done".to_string();
//     // let scopy = s.clone();
//     let h = std::thread::spawn(|| {
//         for i in 0..=10 {
//             println!("{}", i);
//             std::thread::sleep(Duration::from_millis(1000));
//         }
//         &s
//     });
//     for i in 0..=5 {
//         println!("{}", i);
//         std::thread::sleep(Duration::from_millis(1000));
//     }
//     let res = h.join().unwrap();
//     println!("{}", res);
// }


// Пример 2

// Два стула с x

// use std::thread;
// use std::time::Duration;
// use std::sync::{Arc, Mutex};

// fn main() {
//     // let s = "thread done".to_string();
//     // let scopy = s.clone();
//     let mut x = Arc::new(Mutex::new(0));
//     let xx = x.clone();
//     thread::Builder::spawn(move || {
//         for i in 0..10 {
//             {
//                 let mut q = x.lock().unwrap();
//                 *q += 2;
//                 println!("+2: {}", *q);
//             }
//             thread::sleep(Duration::from_millis(100));
//         }
//     });

//     for i in 0..5 {
//         {
//             let mut q = x.lock().unwrap();
//             *q += 2;
//         }
//         std::thread::sleep(Duration::from_millis(100));
//     }
// }


// Пример 3

// use std::time::Duration;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc::channel;

// fn main() {
//     let (tx, rx) = channel::<i32>();
//     let h = thread::spawn(move  || {
//         for i in 0..10 {
//             std::thread::sleep(Duration::from_millis(400));
//             tx.send(i * i).unwrap();
//         }
//     });

//     for i in 0..10 {
//         let j = rx.recv().unwrap();
//         std::thread::sleep(Duration::from_millis(400));
//         println!("{}", j);
//     }
// }


// // Пример 4

// use std::time::Duration;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc::{channel, sync_channel};

// fn main() {
//     let (tx, rx) = sync_channel(3);
//     let tx2 = tx.clone();
    
//     let hq = thread::spawn(move || {
//         for i in 0..10 {
//             tx.send(i * i).unwrap();
//             println!("sended {}", i * i);
//             thread::sleep(Duration::from_millis(400));
//         }
//     });

//     let h = thread::spawn(move || {
//         for i in 0..10 {
//             tx2.send(i * i * i).unwrap();
//             println!("sended {}", i * i * i);
//             thread::sleep(Duration::from_millis(400));
//         }
//     });

//     for i in 0..10 {
//         let j = rx.recv().unwrap();
//         println!("{}", j);
//     }
// }


// Пример 5

use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::sync::mpsc::{channel, sync_channel};
enum ThreadId {
    A,
    B,
}

fn main() {
    let (tx, rx) = channel::<(ThreadId, i32)>();
    let tx2 = tx.clone();
    
    let hq = thread::spawn(move || {
        for i in 0..10 {
            tx.send((ThreadId::A, i * i)).unwrap();
            println!("sended {}", i * i);
            thread::sleep(Duration::from_millis(400));
        }
    });

    let h = thread::spawn(move || {
        for i in 0..10 {
            tx2.send((ThreadId::B, i * i * i)).unwrap();
            println!("sended {}", i * i * i);
            thread::sleep(Duration::from_millis(400));
        }
    });

    for i in 0..10 {
        let j = rx.recv().unwrap();
        match j {
            (ThreadId::A, x) => println!("recieved from a {}", x),
            (ThreadId::B, x) => println!("recieved from a {}", x),
        }
    }
}
