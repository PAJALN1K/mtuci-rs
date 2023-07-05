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
    println!("{}", find_term(SEARCH_TERM, QUOTE));
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut final_string = String::new();
    let quote = quote.split("\n");
    let mut line_count = 1;

    for q in quote {
       let term_index = match q.find(search_term) {
           Some(i) => Some(i),
           None => None,
        };

        if term_index != None {
            let line_count = &line_count.to_string()[..];

            final_string.push_str(line_count);
            final_string.push_str(": ");
            final_string.push_str(q);

            break
        }

        line_count += 1;
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
