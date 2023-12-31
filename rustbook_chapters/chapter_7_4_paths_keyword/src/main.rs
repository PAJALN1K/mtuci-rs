// Пример 1

// мы можем один раз создать псевдоним на путь при помощи ключевого слова use, а затем использовать более 
// короткое имя везде в области видимости.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


// Пример 2

// Не компилируется.

// Обратите внимание, что use создаёт псевдоним только для той конкретной области, в которой это объявление 
// use и находится. В листинге 7-12 функция eat_at_restaurant перемещается в новый дочерний модуль с именем 
// customer, область действия которого отличается от области действия оператора use, поэтому тело функции 
// не будет компилироваться

// Чтобы решить эту проблему, можно переместить use в модуль customer, или же можно сослаться на псевдоним 
// в родительском модуле с помощью super::hosting в дочернем модуле customer.

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }


// Пример 3

// В примере 2 вы могли бы задаться вопросом, почему мы указали use crate::front_of_house::hosting, а 
// затем вызвали hosting::add_to_waitlist внутри eat_at_restaurant вместо указания в use полного пути прямо 
// до функции add_to_waitlist для получения того же результата, что в примере 3.

// Неидиоматический способ

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// Хотя примеры 2 и 3 выполняют одну и ту же задачу, пример 2 является идиоматическим способом 
// подключения функции в область видимости с помощью use. Подключение родительского модуля функции в область 
// видимости при помощи use означает, что мы должны указывать родительский модуль при вызове функции. 
// Указание родительского модуля при вызове функции даёт понять, что функция не определена локально, но в то 
// же время сводя к минимуму повторение полного пути. В коде примера 3 не ясно, где именно определена 
// add_to_waitlist.


// Пример 4

// С другой стороны, при подключении структур, перечислений и других элементов используя use, идиоматически 
// правильным будет указывать полный путь. Листинг 7-14 показывает идиоматический способ подключения структуры 
// стандартной библиотеки HashMap в область видимости бинарного крейта.

// За этой идиомой нет веской причины: это просто соглашение, которое появилось само собой. Люди привыкли 
// читать и писать код на Rust таким образом.

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// Пример 5

// Исключением из этой идиомы является случай, когда мы подключаем два элемента с одинаковыми именами в область 
// видимости используя оператор use - Rust просто не позволяет этого сделать. Пример 5 показывает, как подключить 
// в область действия два типа с одинаковыми именами Result, но из разных родительских модулей и как на них ссылаться.

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Как видите, использование имени родительских модулей позволяет различать два типа Result. Если бы вместо этого мы 
// указали use std::fmt::Result и use std::io::Result, мы бы имели два типа Result в одной области видимости, и Rust не 
// смог бы понять какой из двух Result мы имели в виду, когда нашёл бы их употребление в коде.


// Пример 6

// Есть другое решение проблемы добавления двух типов с одинаковыми именами в одну и ту же область видимости используя use: 
// после пути можно указать as и новое локальное имя (псевдоним) для типа. Листинг 7-16 показывает как по-другому написать 
// код из листинга 7-15, путём переименования одного из двух типов Result используя as.

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }


// Пример 7

// Когда мы подключаем имя в область видимости, используя ключевое слово use, то имя, доступное в новой области видимости, 
// является приватным. Чтобы позволить коду, который вызывает наш код, ссылаться на это имя, как если бы оно было определено 
// в области видимости данного кода, можно объединить pub и use. Этот метод называется реэкспортом (re-exporting), потому 
// что мы подключаем элемент в область видимости, но также делаем этот элемент доступным для подключения в других 
// областях видимости.

// Пример 7 показывает код из примера 1, где use в корневом модуле заменено на pub use.

// Файл: src/lib.rs

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// До этого изменения внешний код должен был вызывать функцию add_to_waitlist , используя путь 
// restaurant::front_of_house::hosting::add_to_waitlist() . Теперь, когда это объявление pub use повторно экспортировало 
// модуль hosting из корневого модуля, внешний код теперь может использовать вместо него путь 
// restaurant::hosting::add_to_waitlist() .

// Реэкспорт полезен, когда внутренняя структура вашего кода отличается от того, как программисты, вызывающие ваш код, думают 
// о предметной области. Например, по аналогии с рестораном люди, управляющие им, думают о «передней части дома» и «задней части 
// дома». Но клиенты, посещающие ресторан, вероятно, не будут думать о частях ресторана в таких терминах. Используя pub use , 
// мы можем написать наш код с одной структурой, но сделать общедоступной другую структуру. Благодаря этому наша библиотека 
// хорошо организована для программистов, работающих над библиотекой, и для программистов, вызывающих библиотеку.





// Члены сообщества Rust сделали много пакетов доступными на ресурсе crates.io, и добавление любого из них в ваш пакет включает 
// в себя одни и те же шаги: добавить внешние пакеты в файл Cargo.toml вашего пакета, использовать use для подключения элементов 
// внешних пакетов в область видимости.

// Обратите внимание, что стандартная библиотека std также является крейтом, внешним по отношению к нашему пакету. Поскольку 
// стандартная библиотека поставляется с языком Rust, нам не нужно изменять Cargo.toml для подключения std.



// Пример 8

// Два одинаковых кода

// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};


// Пример 9

// Два одинаковых кода

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};


// Пример 10

// Оператор * (glob)

// Если мы хотим включить в область видимости все общедоступные элементы, определённые в пути, мы можем указать этот путь, 
// за которым следует оператор *:

// use std::collections::*;

