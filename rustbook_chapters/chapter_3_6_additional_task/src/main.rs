// Конвертация температур между значениями по Фаренгейту к Цельсию.

use::std::io;

fn main() {
    'temperature_convert: loop {
        println!("\n\nTemperature convert");

        println!("\nPlease, type a number to choose an operation:");
        println!("1 - convert Celsius to Fahrenheit");
        println!("2 - convert Fahrenheit to Celsius");

        let mut operation_choice: String = String::new();
        io::stdin()
            .read_line(&mut operation_choice)
            .expect("\nError! Line has not been read! \nPlease, try again.\n");

        let operation_choice: u32 = match operation_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if operation_choice == 1 {
            c_to_f();
        }
        else if operation_choice == 2 {
            f_to_c();
        }
        else {
            println!("\nWrong number! \nPlease, type a correct number.");
            continue;
        }

        loop {
            println!("\n\nWould you like to continue using this program?");
            println!("\nPlease, type a letter to choose an operation:");
            println!("[Y] - Yes, I would.");
            println!("[N] - No, I'd rather quit.");

            let mut operation_choice: String = String::new();
            io::stdin()
                .read_line(&mut operation_choice)
                .expect("\nError! \nLine has not been read! \nPlease, try again.\n");

            let operation_choice = operation_choice.trim();

            if operation_choice == "Y" || operation_choice == "y" {
                continue 'temperature_convert;
            }
            else if operation_choice == "N" || operation_choice == "n" {
                break 'temperature_convert;
            }
            println!("\nWrong input! \nPlease, type a correct letter.");
        }
    }
}

fn c_to_f() {
    loop {
        println!("\nInput the temperature in Celsius degree:");

        let mut temp_c = String::new();

        io::stdin()
                .read_line(&mut temp_c)
                .expect("Error! \nLine has not been read! \nPlease, try again.");

        let temp_c: f64 = match temp_c.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_f: f64 = temp_c * 9.0 / 5.0 + 32.0;

        println!("\nThis temperature in Fahrenheit degree: {}", temp_f);
        break;
    }
}

fn f_to_c() {
    loop {
        println!("\nInput the temperature in Fahrenheit degree:");

        let mut temp_f = String::new();

        io::stdin()
                .read_line(&mut temp_f)
                .expect("Error! \nLine has not been read! \nPlease, try again.");

        let temp_f: f64 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_c: f64 = (temp_f - 32.0) * 5.0 / 9.0 ;

        println!("\nThis temperature in Celsius degree: {}", temp_c);
        break;
    }
}



// Генерирование n-го числа Фибоначчи (до 47).

// use std::io;
//
// fn main() {
//     'fib_num: loop {
//         println!("\n\nFibonacci number");
//
//         println!("\nPlease, type the index of Fibonacci number (it has to be less than 47)");
//
//         let mut fib_index: String = String::new();
//         io::stdin()
//             .read_line(&mut fib_index)
//             .expect("\nError! \nLine has not been read! \nPlease, try again.\n");
//
//         let fib_index: u32 = match fib_index.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         if fib_index >= 47u32 {
//             println!("\nThis index is too big! Please, enter a new one.");
//             continue;
//         }
//
//         println!("The Fibonacci number with index {} is {}", fib_index, fib_number(fib_index));
//
//         loop {
//             println!("\n\nWould you like to continue using this program?");
//             println!("\nPlease, type a letter to choose an operation:");
//             println!("[Y] - Yes, I would.");
//             println!("[N] - No, I'd rather quit.");
//
//             let mut operation_choice: String = String::new();
//             io::stdin()
//                 .read_line(&mut operation_choice)
//                 .expect("\nError! \nLine has not been read! \nPlease, try again.\n");
//
//             let operation_choice = operation_choice.trim();
//
//             if operation_choice == "Y" || operation_choice == "y" {
//                 continue 'fib_num;
//             }
//             else if operation_choice == "N" || operation_choice == "n" {
//                 break 'fib_num;
//             }
//             println!("\nWrong input! \nPlease, type a correct letter.");
//         }
//     }
// }
//
//
// fn fib_number(index: u32) -> u32 {
//     if index == 0 { 0u32 }
//     else if index == 1 { 1u32 }
//     else { fib_number(index-1)  +  fib_number(index-2) }
// }



// Распечатайте текст рождественской песни "Двенадцать дней Рождества", воспользовавшись повторами в песне.

// fn main() {
//     let array1 = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
//     let array2 = [
//         "And a partridge in a pear tree\n",
//         "Two turtle doves",
//         "Three French hens",
//         "Four calling birds",
//         "Five golden rings",
//         "Six geese-a-laying",
//         "Seven swans-a-swimming!",
//         "Eight maids-a-milking!",
//         "Nine drummers drumming",
//         "Ten pipers piping",
//         "Eleven ladies dancing",
//         "Twelve lords-a-leaping"
//     ];
//
//     for i in 0..=11 {
//         println!("On the {} day of Christmas", array1[i]);
//         println!("My true love gave to me");
//
//         if i == 0 {
//             println!("A partridge in a pear tree\n");
//             continue;
//         }
//
//         for y in (0..=i).rev() {
//             println!("{}", array2[y])
//         }
//     }
// }
