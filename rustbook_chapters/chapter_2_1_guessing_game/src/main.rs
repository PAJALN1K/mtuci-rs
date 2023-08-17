// Импорт библиотек и методов
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // Генерация произвольного числа в районе от 1 до 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Создание изменяемой (mut) новой и пустой (new - ассоциированная функция,
        // т.е. функция, привязанная к типу данных) переменной guess типа данных String.
        let mut guess = String::new();

        // stdin - тип, представляющий дескриптор стандартного ввода для вашего терминала.
        // read_line - ввод с командной строки.
        // & - ссылка, по умолчанию неизменяемая. Используется, чтобы не засорять память.
        // Кроме непосредственно ввода c командной строки read_line возвращает Result.
        // Он имеет два значение: Err и Ok. В случае с Err, при возникновении ошибки,
        // метод .except возвратит сообщение об ошибке,
        // указанное в его скобках.
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");




        // match, обладая ветвями с различными исходами событий, сделает так,
        // чтобы выбралась одна из них в зависимости от условий
        // метод .trim() очистит входное число от пробелов и прочих знаков (вместо "5/n" "5"),
        // а метод .parse преобразует string в u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Если число - продолжит дальше
            Err(_) => continue, // Если нет (ошибка) - то вновь попросит ввести число
        };

        println!("You guessed: {guess}");

        // Метод .cmp - сравнение исходной переменной с той, которая указана в скобках.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
