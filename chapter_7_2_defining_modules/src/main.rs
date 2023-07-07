// Шпаргалка по модулям
// Здесь мы даём краткий обзор того, как модули, пути, ключевое слово use и ключевое слово pub 
// работают в компиляторе и как большинство разработчиков организуют свой код. 

// Начнём с корня крейта: при компиляции компилятор сначала ищет корневой модуль крейта 
// (обычно это src/lib.rs для библиотечного крейта или src/main.rs для бинарного крейта) 
// для компиляции кода.

// Объявление модулей: В файле корневого модуля крейта вы можете объявить новые модули; скажем, 
// вы объявляете модуль “garden” с помощью mod garden;. Компилятор будет искать код модуля 
// в следующих местах:
// - в этом же файле, между фигурных скобок, которые заменяют точку с запятой после mod garden
// - в файле src/garden.rs
// - в файле src/garden/mod.rs

// Объявление подмодулей: В любом файле, кроме корневого модуля крейта, вы можете объявить подмодули. 
// К примеру, вы можете объявить mod vegetables; в src/garden.rs. Компилятор будет искать код подмодуля 
// в каталоге с именем родительского модуля в следующих местах:
// - в этом же файле, сразу после mod vegetables, между фигурных скобок, которые заменяют точку с запятой
// - в файле src/garden/vegetables.rs
// - в файле src/garden/vegetables/mod.rs

// Пути к коду в модулях: После того, как модуль станет частью вашего крейта и если допускают правила 
// приватности, вы можете ссылаться на код в этом модуле из любого места вашего крейта, используя путь к коду. 
// Например, тип Asparagus, в подмодуле vegetables модуля garden, будет найден по пути 
// crate::garden::vegetables::Asparagus.

// Скрытие или общедоступность: Код в модуле по умолчанию скрыт от родительского модуля. Чтобы сделать модуль 
// общедоступным, объявите его как pub mod вместо mod. Чтобы сделать элементы общедоступного модуля тоже 
// общедоступными, используйте pub перед их объявлением.

// Ключевое слово use: Внутри области видимости использование ключевого слова use создаёт псевдонимы для элементов, 
// чтобы уменьшить повторение длинных путей. В любой области видимости, в которой может обращаться к 
// crate::garden::vegetables::Asparagus, вы можете создать псевдоним use crate::garden::vegetables::Asparagus; и после 
// этого вам нужно просто писать Asparagus, чтобы использовать этот тип в этой области видимости.


// Мы создали бинарный крейт backyard, который иллюстрирует эти правила. Директория крейта, также названная как backyard, содержит следующие файлы и директории:

// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs
// Файл корневого модуля крейта в нашем случае src/main.rs, и его содержимое:


// Пример 1

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// Строка pub mod garden; говорит компилятору о подключении кода, найденном в src/garden.rs:

// Файл: src/garden.rs
// pub mod vegetables;

// А здесь pub mod vegetables; указывает на подключаемый код в src/garden/vegetables.rs. Этот код:

// #[derive(Debug)]
// pub struct Asparagus {}




// Группировка связанного кода в модулях
// Модули позволяют упорядочивать код внутри крейта для удобочитаемости и лёгкого повторного 
// использования. Модули также позволяют нам управлять приватностью элементов, поскольку код 
// внутри модуля по умолчанию является закрытым.


// Пример 2

// Создадим новую библиотеку (библиотечный крейт) с именем restaurant выполнив команду 
// cargo new restaurant --lib; затем вставим код из листинга 7-11 в src/lib.rs для определения 
// некоторых модулей и сигнатур функций. 

// Файл: src/lib.rs

// Модуль front_of_house , содержащий другие модули, которые в свою очередь содержат функции

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// Мы определяем модуль, начиная с ключевого слова mod, затем определяем название модуля 
// (в данном случае front_of_house) и размещаем фигурные скобки вокруг тела модуля. 
// Внутри модулей, можно иметь другие модули, как в случае с модулями hosting и serving. 
// Модули также могут содержать определения для других элементов, таких как структуры, 
// перечисления, константы, типажи или — как в примере 2 — функции.

// Программистам будет легче найти необходимую функциональность в сгруппированном 
// коде, вместо того чтобы искать её в одном общем списке. 

// Как мы упоминали ранее, файлы src/main.rs и src/lib.rs называются корневыми модулями крейта. 
// Причина такого именования в том, что содержимое любого из этих двух файлов образует модуль с 
// именем crate в корне структуры модулей крейта, известной как дерево модулей.


// дерево модулей для структуры модулей, приведённой в коде примера 2

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// Это дерево показывает, как некоторые из модулей вкладываются друг в друга; например, 
// hosting находится внутри front_of_house. Дерево также показывает, что некоторые модули являются 
// братьями (siblings) друг для друга, то есть они определены в одном модуле; hosting и serving 
// это братья которые определены внутри front_of_house. Если модуль A содержится внутри модуля B, 
// мы говорим, что модуль A является *потомком * (child) модуля B, а модуль B является *родителем* 
// (parent) модуля A. Обратите внимание, что родителем всего дерева модулей является неявный модуль с именем crate.