// Пример 1

// Создание итератора

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();
// }

// Пример 2

// Использование итератора в цикле for

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("Got: {}", val);
//     }
// }


// Пример 3

// Все итераторы реализуют типаж Iterator, который определён в стандартной библиотеке. Его определение выглядит так:

// fn main() {
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }
// }


// Пример 4

// Вызов метода next итератора

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn iterator_demonstration() {
//         let v1 = vec![1, 2, 3];

//         let mut v1_iter = v1.iter();

//         assert_eq!(v1_iter.next(), Some(&1));
//         assert_eq!(v1_iter.next(), Some(&2));
//         assert_eq!(v1_iter.next(), Some(&3));
//         assert_eq!(v1_iter.next(), None);
//     }
// }


// Пример 5

// Вызов метода sum для получения суммы всех элементов в итераторе

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn iterator_sum() {
//         let v1 = vec![1, 2, 3];

//         let v1_iter = v1.iter();

//         let total: i32 = v1_iter.sum();

//         assert_eq!(total, 6);
//     }
// }


// Пример 6

// Вызов адаптера итератора map для создания нового итератора.
// Этот код не приводит к желаемому поведению.

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
}


// Пример 7

// Вызов метода map для создания нового итератора, а затем вызов метода collect для 
// потребления нового итератора и создания вектора

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}


// Пример 8

// Использование метода filter с замыканием, фиксирующим shoe_size

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}