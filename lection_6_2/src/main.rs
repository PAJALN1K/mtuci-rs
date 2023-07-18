// Пример 1

use std::thread::yield_now;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let mut client = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    tokio::spawn(async move {
        loop {
            let (mut socket, _) = client.accept().await.unwrap();
            let buf = [0u16, 1024];
            let n = socket.read(&buf).await.unwrap();
            yield_now();
        }
    });
}


// Пример 2

// use tokio::task::yield_now;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;
// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel::<i32>(5);
//     let (tx2, mut rx2) = mpsc::channel::<String>(5);

//     // tokio::select! {
//     //     val1 = rx.recv() => println!("Recieved from 1st channel {}", val1.unwrap())
//     //     val2 = rx2.recv() => println!("Recieved from 2nd channel {}", val2.unwrap())
//     // }

//     let res = tokio::join!(rx.recv(), rx2.recv());
// }


// egui - простая вещь для графического интерфейса, не для заморочек
// iced - вещь для графического интерфейса, popOs собирается использовать именно его - он более красивый.
// tokio - штука для работы с вводом/выводом.
// на расте можно писать свои сайты - это leptos (молодой, но быстро развивающихся), yew (самый первый и самый популярный)
// tauri - "наш" (растовский) ответ электрону (инструмент, позволяющий веб приложение разворачивать в нормальные приложения)
// если захочется сделать сайт на лептосе или юе, то можно развернуть его на таури.
// туи.рс - для консольной графики.
// serde - 2-я по популярности. Для сериализации/десериализации данных
// diesel - ОЕМ библиотека. Для работы с другими SQL библиотеками (постгрес и прочие)
// macroquad, bevy - для игр
// macroquad - 2d
// teloxide - телеграм бот (потребуется день-два потратить на то, чтобы понять, как работает что-то там)
// areweguiyet - список всех gui библиотек для раста
// arewegameyet - ... для игр

// у тудушки должна быть графика (человеческая)
// необходимо, чтобы можно было добавлять и удалять туду, сортировать их между собой, отмечать, как выполненную и т.д.
