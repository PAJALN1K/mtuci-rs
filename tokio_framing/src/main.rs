// Обрамление - это процесс получения потока байтов и преобразования его в поток кадров. 
// Фрейм - это единица данных, передаваемая между двумя одноранговыми узлами.

// Пример 1

// Фрейм протокола Redis определяется следующим образом:

use bytes::Bytes;

enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}


// Пример 2

// Для HTTP фрейм может выглядеть следующим образом:

// enum HttpFrame {
//     RequestHead {
//         method: Method,
//         uri: Uri,
//         version: Version,
//         headers: HeaderMap,
//     },
//     ResponseHead {
//         status: StatusCode,
//         version: Version,
//         headers: HeaderMap,
//     },
//     BodyChunk {
//         chunk: Bytes,
//     },
// }

fn main() {
    println!("AHAHAHAHHAHAH you're too late, Sonic!\nNow I'm a cringe string slice!");
}
