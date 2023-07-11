// Реализовать собственный vector. Ваш вектор должен иметь методы new, with_capacity (создаёт вектор 
// с заранее заданым размером), push, pop (возвращает последний элемент вектора, удаляя его из вектора),
// remove (работает как pop, но принимает индекс элемента, который нужно вернуть), get, resize (изменяет 
// размер вектора)

#[derive(Debug)]
struct Victor<T> {
    core: T,
    len: u32,
    capacity: u32,
}

fn main() {
    // let vitya = Victor{ core: Box::new([1u32]), len: 3, capacity: 3 };
    // println!("{:?}", vitya);
    // println!("{:?}, {:?}, {:?}", vitya.core, vitya.len, vitya.capacity);
    
    let vityok = Victor::new();
    println!("{:?}", vityok);
    println!("{:?}, {:?}, {:?}", vityok.core, vityok.len, vityok.capacity);
}

impl<T> Victor<T> {
    fn new() -> Victor<T> {
        Victor {
            core: (), 
            len: 0, 
            capacity: 0,
        }
    }
    // // создаёт вектор с заранее заданым размером
    // fn with_capacity() {

    // }
    // fn push() {

    // }
    // // возвращает последний элемент вектора, удаляя его из вектора
    // fn pop() {

    // }
    // // работает как pop, но принимает индекс элемента, который нужно вернуть(?)
    // fn remove() {

    // }
    // fn get() {

    // }
    // // изменяет размер вектора
    // fn resize() {

    // }
}
