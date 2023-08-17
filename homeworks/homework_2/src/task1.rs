// Слишком мудреный вариант решения задачи

/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    // println!("{}", find_term(SEARCH_TERM, QUOTE));   // convenience for code testing;
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut string_start = 0usize;
    let mut quote_num = 1usize;
    let length = quote.chars().count() - search_term.chars().count();
    let mut final_string = String::new();

    for i  in 0usize..length {
        if &quote[i..=i] == "\n" {
            quote_num += 1;
            string_start = i+1;

        }

        if search_term == &quote[i..i+search_term.chars().count()] {
            let quote_num = &quote_num.to_string()[..];

            final_string.push_str(quote_num);
            final_string.push_str(": ");

            for j in i..length {
                if &quote[j..=j] == "\n" {
                    final_string.push_str(&quote[string_start..j]);
                    break
                }
            }

            break;
        }
    }
    final_string
}

// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
