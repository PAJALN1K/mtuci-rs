// Последняя коллекция, которую мы рассмотрим, будет hash map (хеш-карта). Тип HashMap<K, V> хранит ключи 
// типа K на значения типа V. Данная структура организует и хранит данные с помощью функции хеширования. 
// Во множестве языков программирования реализована данная структура, но часто с разными наименованиями: 
// такими как hash, map, object, hash table, dictionary или ассоциативный массив.

// Хеш-карты полезны, когда нужно искать данные не используя индекс, как это например делается в векторах, 
// а с помощью ключа, который может быть любого типа. Например, в игре вы можете отслеживать счёт каждой 
// команды в хеш-карте, в которой каждый ключ - это название команды, а значение - счёт команды.


// Пример 1

// Создание новой хеш-карты и вставка в неё пары ключей и значений

// Из трёх коллекций данная является наименее используемой, поэтому она не подключается в область видимости 
// функцией автоматического импорта (prelude).

// Подобно векторам, хеш-карты хранят свои данные в куче. Как и векторы, HashMap однородны: все ключи 
// должны иметь одинаковый тип и все значения должны иметь тоже одинаковый тип.

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
};


// Пример 2

// Доступ к очкам команды "Blue", которые хранятся в хеш-карте

// Метод get возвращает Option<&V>; если для какого-то ключа нет значения в HashMap, get вернёт None.

// Этот код будет печатать каждую пару в произвольном порядке:
// Yellow: 50
// Blue: 10

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
};


// Пример 3

// Мы можем перебирать каждую пару ключ/значение в HashMap таким же образом, как мы делали с векторами, 
// используя цикл for:

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
};


// Пример 4

// Для типов, которые реализуют типаж Copy, например i32, значения копируются в HashMap. Для значений со 
// владением, таких как String, значения будут перемещены в хеш-карту и она станет владельцем этих значений

// Мы не можем использовать переменные field_name и field_value после того, как их значения были 
// перемещены в HashMap вызовом метода insert.

fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
};


// Обновление данных в HashMap

// Хотя количество ключей и значений может увеличиваться в HashMap, каждый ключ может иметь только одно 
// значение, связанное с ним в один момент времени (обратное утверждение неверно: команды "Blue" и "Yellow" 
// могут хранить в хеш-карте scores одинаковое количество очков, например 10).

// Пример 5

// Замена значения, хранимого в конкретном ключе

// Даже несмотря на то, что код вызывает insert дважды, хеш-карта будет содержать только 
// одну пару ключ/значение, потому что мы вставляем значения для одного и того же ключа - ключа команды 
// "Blue".

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
};


// Пример 6

// Обычно проверяют, существует ли конкретный ключ в хеш-карте со значением, а затем предпринимаются 
// следующие действия: если ключ существует в хеш-карте, существующее значение должно оставаться таким, 
// какое оно есть. Если ключ не существует, то вставляют его и значение для него.

// Хеш-карты имеют для этого специальный API, называемый entry , который принимает ключ для проверки в 
// качестве входного параметра. Возвращаемое значение метода entry - это перечисление Entry, с двумя 
// вариантами: первый представляет значение, которое может существовать, а второй говорит о том, что 
// значение отсутствует. Допустим, мы хотим проверить, имеется ли ключ и связанное с ним значение для 
// команды "Yellow". Если хеш-карта не имеет значения для такого ключа, то мы хотим вставить значение 
// 50. То же самое мы хотим проделать и для команды "Blue".

// Метод or_insert определён в Entry так, чтобы возвращать изменяемую ссылку на соответствующее значение 
// ключа внутри варианта перечисления Entry, когда этот ключ существует, а если его нет, то вставлять 
// параметр в качестве нового значения этого ключа и возвращать изменяемую ссылку на новое значение. 

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
};


// Пример 7

// Подсчёт количества вхождений слов с использованием хеш-карты, которая хранит слова и счётчики

// Метод split_whitespace возвращает итератор по срезам строки, разделённых пробелами, для строки text.

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
};
