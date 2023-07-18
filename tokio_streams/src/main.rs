// Пример 1

// В настоящее время язык программирования Rust не поддерживает асинхронные for циклы. 
// Вместо этого итерация потоков выполняется с помощью while let цикла, сопряженного с 
// StreamExt::next().

// use tokio_stream::StreamExt;

// #[tokio::main]
// async fn main() {
//     let mut stream = tokio_stream::iter(&[1, 2, 3]);

//     while let Some(v) = stream.next().await {
//         println!("GOT = {:?}", v);
//     }
// }


// Пример 2

// Создается задача публиковать сообщения на сервере Mini-Redis на канале "numbers" . 
// Затем, в основной задаче, мы подписываемся на канал "numbers" и показываем 
// полученные сообщения.

// После подписки into_stream() вызывается на возвращенном подписчике. Это потребляет 
// Subscriber, возвращая поток, который выдает сообщения по мере их поступления. Прежде 
// чем мы начнем повторять сообщения, обратите внимание, что поток закрепляется в 
// стеке с помощью tokio::pin!. Для вызова next() потока требуется, чтобы поток был 
// закреплен. into_stream() Функция возвращает поток, который не закреплен, мы должны 
// явно закрепить его, чтобы повторить его.

// вывод
// got = Ok(Message { channel: "numbers", content: b"1" })
// got = Ok(Message { channel: "numbers", content: b"two" })
// got = Ok(Message { channel: "numbers", content: b"3" })
// got = Ok(Message { channel: "numbers", content: b"four" })
// got = Ok(Message { channel: "numbers", content: b"five" })
// got = Ok(Message { channel: "numbers", content: b"6" })
// Программа никогда не завершается.

// use tokio_stream::StreamExt;
// use mini_redis::client;

// async fn publish() -> mini_redis::Result<()> {
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Publish some data
//     client.publish("numbers", "1".into()).await?;
//     client.publish("numbers", "two".into()).await?;
//     client.publish("numbers", "3".into()).await?;
//     client.publish("numbers", "four".into()).await?;
//     client.publish("numbers", "five".into()).await?;
//     client.publish("numbers", "6".into()).await?;
//     Ok(())
// }

// async fn subscribe() -> mini_redis::Result<()> {
//     let client = client::connect("127.0.0.1:6379").await?;
//     let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
//     let messages = subscriber.into_stream();

//     tokio::pin!(messages);

//     while let Some(msg) = messages.next().await {
//         println!("got = {:?}", msg);
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> mini_redis::Result<()> {
//     tokio::spawn(async {
//         publish().await
//     });

//     subscribe().await?;

//     println!("DONE");

//     Ok(())
// }


// Пример 3

// Тот же код, что и в примере 2, только заменена переменная messages

// вывод
// got = Ok(Message { channel: "numbers", content: b"1" })
// got = Ok(Message { channel: "numbers", content: b"two" })
// got = Ok(Message { channel: "numbers", content: b"3" })
// На этот раз программа завершается.

// let messages = subscriber
//     .into_stream()
//     .take(3);


// Пример 4

// Тот же код, что и в примере 2, только заменена переменная messages

// Теперь давайте ограничим поток однозначными числами. Мы проверим это, 
// проверив длину сообщения. Мы используем filter адаптер для удаления любого 
// сообщения, которое не соответствует предикату.

// let messages = subscriber
//     .into_stream()
//     .filter(|msg| match msg {
//         Ok(msg) if msg.content.len() == 1 => true,
//         _ => false,
//     })
//     .take(3);


// Пример 5

// Тот же код, что и в примере 2, только заменена переменная messages

// Наконец, мы приведем в порядок выходные данные, удалив Ok(Message { ... }) часть 
// выходных данных. Это делается с помощью map. Поскольку это применяется после filter, 
// мы знаем, что сообщение является Ok, поэтому мы можем использовать unwrap().

// Теперь на выходе:
// got = b"1"
// got = b"3"
// got = b"6"

// let messages = subscriber
//     .into_stream()
//     .filter(|msg| match msg {
//         Ok(msg) if msg.content.len() == 1 => true,
//         _ => false,
//     })
//     .map(|msg| msg.unwrap().content)
//     .take(3);


// Пример 6

// Трейт Stream очень похож на трейт Future.

// use std::pin::Pin;
// use std::task::{Context, Poll};

// pub trait Stream {
//     type Item;

//     fn poll_next(
//         self: Pin<&mut Self>, 
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>>;

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, None)
//     }
// }

// Stream::poll_next()Функция во многом похожа на Future::poll, за 
// исключением того, что ее можно вызывать повторно для получения большого 
// количества значений из потока. Точно так же, как мы видели в Асинхронность 
// в глубине (async_in_depth), когда поток не готов вернуть значение, 
// Poll::Pending . Зарегистрирован инициатор проверки задачи. Как только 
// поток должен быть опрошен снова, инициатор проверки получает уведомление.

// size_hint() Метод используется так же, как и с итераторами.


// Пример 7

// Обычно при ручной реализации Stream это делается путем составления фьючерсов 
// и других потоков. В качестве примера давайте подробно рассмотрим Delay future, 
// которое мы реализовали в Async. Мы преобразуем ее в поток, который выдает 
// результаты () три раза с интервалом в 10 мс.

// use tokio_stream::Stream;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// use std::time::Duration;

// struct Interval {
//     rem: usize,
//     delay: Delay,
// }

// impl Interval {
//     fn new() -> Self {
//         Self {
//             rem: 3,
//             delay: Delay { when: Instant::now() }
//         }
//     }
// }

// impl Stream for Interval {
//     type Item = ();

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
//         -> Poll<Option<()>>
//     {
//         if self.rem == 0 {
//             // No more delays
//             return Poll::Ready(None);
//         }

//         match Pin::new(&mut self.delay).poll(cx) {
//             Poll::Ready(_) => {
//                 let when = self.delay.when + Duration::from_millis(10);
//                 self.delay = Delay { when };
//                 self.rem -= 1;
//                 Poll::Ready(Some(()))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }


// Пример 8

// Ручная реализация потоков с использованием Stream признака может быть утомительной. 
// К сожалению, язык программирования Rust пока не поддерживает async/await синтаксис для 
// определения потоков. Это находится в разработке, но еще не готово.

// Крейт async-stream  доступен как временное решение. Этоткрейт предоставляет stream! макрос, 
// который преобразует входные данные в поток. Используя этот крейт, вышеуказанный интервал 
// может быть реализован следующим образом:

use async_stream::stream;
use std::time::{Duration, Instant};

stream! {
    let mut when = Instant::now();
    for _ in 0..3 {
        let delay = Delay { when };
        delay.await;
        yield ();
        when += Duration::from_millis(10);
    }
}