// Панику можно создать или, собственно, каким-либо действием,
// или с помощью макроса panic!


// Пример 1
fn main() {
    panic!("crash and burn");
}


// По умолчанию, когда происходит паника, программа начинает процесс раскрутки стека, означающий в Rust
// проход обратно по стеку вызовов и очистку данных для каждой обнаруженной функции. Тем не менее, этот 
// обратный проход по стеку и очистка генерируют много работы. Rust как альтернативу предоставляет вам 
// возможность немедленного прерывания (aborting), которое завершает работу программы без очистки.

// Память, которую использовала программа, должна быть очищена операционной системой. Если в вашем 
// проекте нужно насколько это возможно сделать маленьким исполняемый файл, вы можете переключиться с 
// варианта раскрутки стека на вариант прерывания при панике, добавьте panic = 'abort' в раздел 
// [profile] вашего Cargo.toml файла. Например, если вы хотите прервать панику в режиме релиза, 
// добавьте это:
// [profile.release]
// panic = 'abort'


// Пример 2
// Попытка доступа к элементу за пределами вектора, которая вызовет panic!
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

// мы можем установить переменную среды RUST_BACKTRACE, чтобы получить обратную трассировку того, что 
// именно стало причиной ошибки. Обратная трассировка создаёт список всех функций, которые были вызваны 
// до какой-то определённой точки выполнения программы.

// $ RUST_BACKTRACE=1 cargo run
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
//    1: core::panicking::panic_fmt
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
//    2: core::panicking::panic_bounds_check
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
//    3: <usize as core::slice::index::SliceIndex<[T]>>::index
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
//    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
//    5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
//    6: panic::main
//              at ./src/main.rs:4
//    7: core::ops::function::FnOnce::call_once
//              at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

// строка #6 указывает на строку в нашем проекте, которая вызывала проблему: строка 4 из файла 
// src/main.rs. Если мы не хотим, чтобы наша программа запаниковала, мы должны начать исследование с 
// места, на которое указывает первая строка с упоминанием нашего файла. 
