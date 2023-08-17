// Пример 1

// Тестирование приватных функций

pub fn add_twoo(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}


// Пример 2

// Интеграционная тест функция 

use chapter_11_2_control_of_how_tests_are_run::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

fn main() {
    println!("AHAHAHAHHAHAH you're too late, Sonic!\nNow I'm a cringe string slice!");
}
