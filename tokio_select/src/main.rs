// Пример 1

// tokio::select! Макрос позволяет ожидать выполнения нескольких асинхронных вычислений 
// и возвращается, когда завершается одно вычисление.

// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     tokio::spawn(async {
//         let _ = tx1.send("one");
//     });

//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });

//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 completed first with {:?}", val);
//         }
//         val = rx2 => {
//             println!("rx2 completed first with {:?}", val);
//         }
//     }
// }


// Пример 2

// Фьючерсы или другие типы могут быть реализованы Drop для очистки фоновых ресурсов. 
// Tokio oneshot::Receiver реализует Drop отправку закрытого уведомления Sender половине. 
// Отправитель half может получить это уведомление и прервать текущую операцию, отбросив его.

// use tokio::sync::oneshot;

// async fn some_operation() -> String {
//     // Compute value here
// }

// #[tokio::main]
// async fn main() {
//     let (mut tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     tokio::spawn(async {
//         // Select on the operation and the oneshot's
//         // `closed()` notification.
//         tokio::select! {
//             val = some_operation() => {
//                 let _ = tx1.send(val);
//             }
//             _ = tx1.closed() => {
//                 // `some_operation()` is canceled, the
//                 // task completes and `tx1` is dropped.
//             }
//         }
//     });

//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });

//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 completed first with {:?}", val);
//         }
//         val = rx2 => {
//             println!("rx2 completed first with {:?}", val);
//         }
//     }
// }


// Пример 3

// Чтобы помочь лучше понять, как select! работает, давайте посмотрим, как будет 
// выглядеть гипотетическая Future реализация. Это упрощенная версия. На практике select! 
// включает дополнительные функциональные возможности, такие как случайный выбор ветки 
// для опроса первой.

// use tokio::sync::oneshot;
// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};

// struct MySelect {
//     rx1: oneshot::Receiver<&'static str>,
//     rx2: oneshot::Receiver<&'static str>,
// }

// impl Future for MySelect {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
//         if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
//             println!("rx1 completed first with {:?}", val);
//             return Poll::Ready(());
//         }

//         if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
//             println!("rx2 completed first with {:?}", val);
//             return Poll::Ready(());
//         }

//         Poll::Pending
//     }
// }

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     // use tx1 and tx2

//     MySelect {
//         rx1,
//         rx2,
//     }.await;
// }



// select! Макрос может обрабатывать более двух ветвей. Текущее ограничение составляет 
// 64 ветви. Каждая ветвь структурирована как:
// <pattern> = <async expression> => <handler>,

// При вычислении select макроса все <async expression> макро-файлы агрегируются и 
// выполняются одновременно. Когда выражение завершается, результат сопоставляется с 
// <pattern>. Если результат соответствует шаблону, то все оставшиеся асинхронные 
// выражения удаляются и <handler> выполняются. <handler> Выражение имеет доступ ко 
// всем привязкам, установленным <pattern>.

// Основным случаем для <pattern> является имя переменной, результат асинхронного 
// выражения привязан к имени переменной и <handler> имеет доступ к этой переменной. 
// Вот почему в исходном примере val использовался для <pattern> и <handler> был 
// доступен val.

// Если <pattern> не совпадает с результатом асинхронного вычисления, то остальные 
// асинхронные выражения продолжают выполняться одновременно до завершения следующего. 
// В это время к этому результату применяется та же логика.

// Поскольку select! принимает любое асинхронное выражение, можно определить более 
// сложные вычисления для выбора.

// Пример 4

// Здесь мы выбираем на выходе oneshot канала и TCP-соединения.

// use tokio::net::TcpStream;
// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();

//     // Spawn a task that sends a message over the oneshot
//     tokio::spawn(async move {
//         tx.send("done").unwrap();
//     });

//     tokio::select! {
//         socket = TcpStream::connect("localhost:3465") => {
//             println!("Socket connected {:?}", socket);
//         }
//         msg = rx => {
//             println!("received message first {:?}", msg);
//         }
//     }
// }

// Пример 5

// Здесь мы выбираем на oneshot и принимаем сокеты от TcpListener.

// use tokio::net::TcpListener;
// use tokio::sync::oneshot;
// use std::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let (tx, rx) = oneshot::channel();

//     tokio::spawn(async move {
//         tx.send(()).unwrap();
//     });

//     let mut listener = TcpListener::bind("localhost:3465").await?;

//     tokio::select! {
//         _ = async {
//             loop {
//                 let (socket, _) = listener.accept().await?;
//                 tokio::spawn(async move { process(socket) });
//             }

//             // Help the rust type inferencer out
//             Ok::<_, io::Error>(())
//         } => {}
//         _ = rx => {
//             println!("terminating accept loop");
//         }
//     }

//     Ok(())
// }


// Пример 6

// tokio::select! Макрос возвращает результат вычисленного <handler> выражения.

// async fn computation1() -> String {
//     // .. computation
// }

// async fn computation2() -> String {
//     // .. computation
// }

// #[tokio::main]
// async fn main() {
//     let out = tokio::select! {
//         res1 = computation1() => res1,
//         res2 = computation2() => res2,
//     };

//     println!("Got = {}", out);
// }


// Пример 7

// Использование ? оператора приводит к распространению ошибки из выражения. 
// Как это работает, зависит от того, ? используется из асинхронного выражения или 
// из обработчика. Использование ? в асинхронном выражении приводит к распространению 
// ошибки из асинхронного выражения. Это делает вывод асинхронного выражения Result. 
// Использование ? из обработчика немедленно выводит ошибку из select! выражения.

// use tokio::net::TcpListener;
// use tokio::sync::oneshot;
// use std::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // [setup `rx` oneshot channel]

//     let listener = TcpListener::bind("localhost:3465").await?;

//     tokio::select! {
//         res = async {
//             loop {
//                 let (socket, _) = listener.accept().await?;
//                 tokio::spawn(async move { process(socket) });
//             }

//             // Help the rust type inferencer out
//             Ok::<_, io::Error>(())
//         } => {
//             res?;
//         }
//         _ = rx => {
//             println!("terminating accept loop");
//         }
//     }

//     Ok(())
// }


// Пример 8

// Напомним, что select! синтаксис ветвления макроса был определен как:
// <pattern> = <async expression> => <handler>,

// До сих пор мы использовали привязки переменных только для <pattern>. Однако можно 
// использовать любой шаблон Rust . Например, предположим, что мы получаем от 
// нескольких MPSC каналов, мы могли бы сделать что-то вроде этого:

// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (mut tx1, mut rx1) = mpsc::channel(128);
//     let (mut tx2, mut rx2) = mpsc::channel(128);

//     tokio::spawn(async move {
//         // Do something w/ `tx1` and `tx2`
//     });

//     tokio::select! {
//         Some(v) = rx1.recv() => {
//             println!("Got {:?} from rx1", v);
//         }
//         Some(v) = rx2.recv() => {
//             println!("Got {:?} from rx2", v);
//         }
//         else => {
//             println!("Both channels closed");
//         }
//     }
// }



// При создании задач созданному асинхронному выражению должны принадлежать все его 
// данные. select! Макрос не имеет этого ограничения. Асинхронное выражение каждой 
// ветви может заимствовать данные и работать одновременно. Следуя правилам 
// заимствования Rust, несколько асинхронных выражений могут неизменно заимствовать 
// одну часть данных или одно асинхронное выражение может изменчиво заимствовать 
// часть данных.

// Пример 9

// Здесь мы одновременно отправляем одни и те же данные двум разным адресатам TCP.

// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpStream;
// use std::io;
// use std::net::SocketAddr;

// async fn race(
//     data: &[u8],
//     addr1: SocketAddr,
//     addr2: SocketAddr
// ) -> io::Result<()> {
//     tokio::select! {
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr1).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr2).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         else => {}
//     };

//     Ok(())
// }

// Пример 10

// Когда дело доходит до каждой ветки <handler>, select! гарантируется, что выполняется 
// только одна <handler>. Из-за этого каждая <handler> ветка может изменчиво заимствовать 
// одни и те же данные.

// Например, это изменяет out в обоих обработчиках:

// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     let mut out = String::new();

//     tokio::spawn(async move {
//         // Send values on `tx1` and `tx2`.
//     });

//     tokio::select! {
//         _ = rx1 => {
//             out.push_str("rx1 completed");
//         }
//         _ = rx2 => {
//             out.push_str("rx2 completed");
//         }
//     }

//     println!("{}", out);
// }


// Пример 11

// select! Макрос часто используется в циклах. В этом разделе будут рассмотрены некоторые 
// примеры, показывающие распространенные способы использования select! макроса в цикле. 
// Мы начнем с выбора по нескольким каналам:

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}


// Пример 12

// Теперь мы покажем, как выполнить асинхронную операцию при нескольких вызовах select!. 
// В этом примере у нас есть канал MPSC с типом элемента i32 и асинхронная функция. Мы хотим 
// запускать асинхронную функцию до тех пор, пока она не завершится или на канале не будет 
// получено четное целое число.

// async fn action() {
//     // Some asynchronous logic
// }

// #[tokio::main]
// async fn main() {
//     let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);    
    
//     let operation = action();
//     tokio::pin!(operation);
    
//     loop {
//         tokio::select! {
//             _ = &mut operation => break,
//             Some(v) = rx.recv() => {
//                 if v % 2 == 0 {
//                     break;
//                 }
//             }
//         }
//     }
// }


// Пример 13

// Изменение ветки

// Давайте рассмотрим несколько более сложный цикл. У нас есть:
// 1. Канал i32 значений.
// 2. Асинхронная операция для выполнения над i32 значениями.

// Логика, которую мы хотим реализовать, такова:

// 1. Дождитесь четного числа на канале.
// 2. Запустите асинхронную операцию, используя четное число в качестве входных данных.
// 3. Дождитесь завершения операции, но в то же время прослушайте больше четных чисел на канале.
// 4. Если новое четное число получено до завершения существующей операции, прервите существующую 
// операцию и начните ее заново с новым четным числом.

// async fn action(input: Option<i32>) -> Option<String> {
//     // If the input is `None`, return `None`.
//     // This could also be written as `let i = input?;`
//     let i = match input {
//         Some(input) => input,
//         None => return None,
//     };
//     // async logic here
// }

// #[tokio::main]
// async fn main() {
//     let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);
    
//     let mut done = false;
//     let operation = action(None);
//     tokio::pin!(operation);
    
//     tokio::spawn(async move {
//         let _ = tx.send(1).await;
//         let _ = tx.send(3).await;
//         let _ = tx.send(2).await;
//     });
    
//     loop {
//         tokio::select! {
//             res = &mut operation, if !done => {
//                 done = true;

//                 if let Some(v) = res {
//                     println!("GOT = {}", v);
//                     return;
//                 }
//             }
//             Some(v) = rx.recv() => {
//                 if v % 2 == 0 {
//                     // `.set` is a method on `Pin`.
//                     operation.set(action(Some(v)));
//                     done = false;
//                 }
//             }
//         }
//     }
// }