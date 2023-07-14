// Пример 1

// Сохранение значения i32 в куче с использованием box
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}


// Пример 2

// Первая попытка определить перечисление в качестве структуры данных cons list, состоящей 
// из i32 значений. Не скомпилируется

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {}


// Пример 3

// Использование перечисления List для хранения списка 1, 2, 3. Не скомпилируется

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}


// Пример 4

// Определение List, которое использует Box<T> для того, чтобы иметь вычисляемый размер

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
