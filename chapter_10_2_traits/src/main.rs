// Пример 1

// Определение типажа Summary, который содержит поведение 
// предоставленное методом summarize

// pub trait Summary {
//     fn summarize(&self) -> String;
// }


// Пример 2

// Реализация типажа Summary для структур NewsArticle и Tweet

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


// Пример 3

//

// вывод:
// 1 new tweet: horse_ebooks: of course, as you probably already know, people.
mod aggregator;
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}


// Пример 4

// Определение типажа Summary с реализацией метода summarize по умолчанию

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }


// Пример 5

// вывод
// New article available! (Read more...)

// use aggregator::{self, NewsArticle, Summary};

// fn main() {
//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     println!("New article available! {}", article.summarize());
// }


// Пример 6

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }


// Пример 7

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }


// Пример 8

// вывод
// 1 new tweet: (Read more from @horse_ebooks...)

// use aggregator::{self, Summary, Tweet};

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }


// Пример 10

// Вместо этого:

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// ..лучше делать так

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}


// Пример 11

// Также можно использовать синтаксис impl Trait в возвращаемой позиции, чтобы 
// вернуть значение некоторого типа реализующего типаж, как показано здесь:

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }


// Пример 12

// Однако, impl Trait возможно использовать, если возвращаете только один тип. 
// Например, данный код, который возвращает значения или типа NewsArticle или типа 
// Tweet, но в качестве возвращаемого типа объявляет impl Summary , не будет работать:

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


// Пример 13

// Условная реализация методов у обобщённых типов в зависимости от ограничений типажа

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }
