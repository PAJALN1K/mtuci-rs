// Пример 2

// Затем поместим код, который был в фигурных скобках, в новый файл с именем src/front_of_house.rs, как показано в 
// листинге 7-22. Компилятор знает, что нужно искать в этом файле, потому что он наткнулся в корневом модуле крейта на 
// объявление модуля с именем front_of_house.

// pub mod hosting {
//     pub fn add_to_waitlist() {}
// }


// Пример 3

// Чтобы начать перенос hosting, мы меняем src/front_of_house.rs так, чтобы он содержал только объявление модуля hosting:

pub mod hosting;

// [продолжение в hostings.rs]