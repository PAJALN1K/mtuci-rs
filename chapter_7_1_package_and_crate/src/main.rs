// По мере роста кодовой базы ваших программ, организация проекта будет иметь большое значение, 
// ведь отслеживание всей программы в голове будет становиться всё более сложным. Группируя связанные 
// функции и разделяя код по основным функциональностям (фичам, feature), вы делаете более прозрачным 
// понимание о том, где искать код реализующий определённую функцию и где стоит вносить изменения 
// для того чтобы изменить её поведение.

// Программы, которые мы писали до сих пор, были в одном файле одного модуля. По мере роста проекта, 
// мы можем организовывать код иначе, разделив его на несколько модулей и несколько файлов. Пакет может 
// содержать несколько бинарных крейтов и опционально один крейт библиотеки.

// Мы также обсудим инкапсуляцию деталей, которая позволяет использовать код снова на более высоком 
// уровне: единожды реализовав какую-то операцию, другой код может вызывать этот код через публичный 
// интерфейс, не зная как работает реализация. То, как вы пишете код, определяет какие части общедоступны 
// для использования другим кодом и какие части являются закрытыми деталями реализации для которых вы 
// оставляете право на изменения только за собой.

// Rust имеет ряд функций, которые позволяют управлять организацией кода, в том числе управлять тем 
// какие детали открыты, какие детали являются частными, какие имена есть в каждой области вашей программы. 
// Эти функции иногда вместе именуемые модульной системой включают в себя:

// Пакеты: Функционал Cargo позволяющий собирать, тестировать и делиться крейтами
// Крейты: Дерево модулей, которое создаёт библиотечный или исполняемый файл
// Модули и use: Позволяют вместе контролировать организацию, область видимости и скрытие путей
// Пути: способ именования элемента, такого как структура, функция или модуль


// Дз до hasmaps (3) или traits (4)



// Первые части модульной системы, которые мы рассмотрим — это пакеты и крейты.

// Крейт — это наименьший объем кода, который компилятор Rust рассматривает за раз. Даже если 
// вы запустите rustc вместо cargo и передадите один файл с исходным кодом, компилятор считает 
// этот файл крейтом. 

// Крейт может быть одним из двух видов: бинарный крейт или библиотечный крейт. Бинарные крейты — 
// это программы, которые вы можете скомпилировать в исполняемые файлы, которые вы можете запускать, 
// например программу командной строки или сервер. У каждого бинарного крейта должна быть функция с 
// именем main, которая определяет, что происходит при запуске исполняемого файла. Все крейты, 
// которые мы создали до сих пор, были бинарными крейтами.

// Библиотечные крейты не имеют функции main и не компилируются в исполняемый файл. Вместо этого они 
// определяют функциональность, предназначенную для совместного использования другими проектами. Например, 
// крейт rand обеспечивает функциональность, которая генерирует случайные числа. В большинстве случаев, 
// когда Rustaceans говорят «крейт», они имеют в виду библиотечный крейт, и они используют «крейт» 
// взаимозаменяемо с общей концепцией программирования «библиотека».

// Корневой модуль крейта — это исходный файл, из которого компилятор Rust начинает собирать корневой модуль 
// вашего крейта.

// Пакет — это набор из одного или нескольких крейтов, предоставляющий набор функциональности. Пакет содержит файл 
// Cargo.toml, в котором описывается, как собирать эти крейты. 
// Пакет может содержать сколько угодно бинарных крейтов, но не более одного библиотечного крейта. Пакет должен 
// содержать хотя бы один крейт, библиотечный или бинарный.


// Пример 1

// (в терминале)

// $ cargo new my-project
//      Created binary (application) `my-project` package
// $ ls my-project
// Cargo.toml
// src
// $ ls my-project/src
// main.rs

// Cargo следует 
// соглашению о том, что src/main.rs — это корневой модуль бинарного крейта с тем же именем, что и у пакета. 
// Точно так же Cargo знает, что если каталог пакета содержит src/lib.rs, пакет содержит библиотечный крейт с 
// тем же именем, что и пакет, а src/lib.rs является корневым модулем этого крейта. 

fn main() {
    println!("Hello, world!");
}