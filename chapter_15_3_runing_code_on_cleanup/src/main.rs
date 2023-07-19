// Пример 1

// Структура CustomSmartPointer, реализующая типаж Drop, куда мы поместим наш код очистки

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     println!("CustomSmartPointers created.");
// }


// Пример 2

// Попытка вызвать метод drop из трейта Drop вручную для досрочной очистки. Не скомпилируется.

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("some data"),
//     };
//     println!("CustomSmartPointer created.");
//     c.drop();
//     println!("CustomSmartPointer dropped before the end of main.");
// }


// Пример 3

// Вызов std::mem::drop для принудительного удаления значения до того, как оно выйдет из 
// области видимости

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}


