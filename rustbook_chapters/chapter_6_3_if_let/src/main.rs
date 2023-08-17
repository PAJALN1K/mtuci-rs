// Синтаксис if let позволяет скомбинировать if и let в менее многословную конструкцию, и затем
// обработать значения соответствующе только одному шаблону, одновременно игнорируя все остальные.


// Пример 1

// Два одинаковых кода

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
//
// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }
// }

// Используя if let мы меньше печатаем, меньше делаем отступов и меньше получаем шаблонного кода.
// Тем не менее, мы теряем полную проверку всех вариантов, предоставляемую выражением match.
// Выбор между match и if let зависит от того, что вы делаете в вашем конкретном случае и является
// ли получение краткости при потере полноты проверки подходящим компромиссом.

// Другими словами, вы можете думать о конструкции if let как о синтаксическом сахаре для match,
// который выполнит код если входное значение будет соответствовать единственному шаблону, и
// проигнорирует все остальные значения.


// Пример 2

// if let и else

// Два одинаковых кода

// fn main() {
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
// }

// fn main() {
//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// }