// 1. Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: 
// - среднее значение; 
// - медиану (значение элемента из середины списка после его сортировки); 
// - моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз; 
// HashMap будет полезна в данном случае).

// *** Реализовать создание различных дополнительных заданий с помощью модулей, пакетов и прочего

// Документация API стандартной библиотеки описывает методы у векторов, строк и HashMap. Рекомендуем 
// воспользоваться ей при решении упражнений.

use std::io;

pub use app_modifier::{app_intro, the_beginning_of_app, the_end_of_app};

pub mod main_macrosses;

pub mod vec_functions;
pub use crate::vec_functions::{vec_medium, vec_median, vec_mode};

fn main() {
    'giga_loop: loop {
        app_modifier::app_intro!(
            "TRANSFORMATIONS OVER VECTOR", 
            "This is a program that commits some transformations over a vector of integers that you create."
        );
        
        app_modifier::the_beginning_of_app!('giga_loop, 'mega_loop_1);
        
        let mut vityok: Vec<i32> = Vec::new();
        // ** добавить проверку ошибок и прочее;
        vector_pushing_init!(&mut vityok, 'mega_loop_2 );

        vectors_transformations_results!(&mut vityok);

        app_modifier::the_end_of_app!('giga_loop);
    }
}
