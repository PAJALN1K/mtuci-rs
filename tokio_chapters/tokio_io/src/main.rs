// Пример 1

// AsyncReadExt::read предоставляет асинхронный метод для чтения данных в буфер, возвращающий 
// количество прочитанных байтов.

// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = [0; 10];

//     // read up to 10 bytes
//     let n = f.read(&mut buffer[..]).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }


// Пример 2

// AsyncReadExt::read_to_end считывает все байты из потока до EOF.

// use tokio::io::{self, AsyncReadExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = Vec::new();

//     // read the whole file
//     f.read_to_end(&mut buffer).await?;
//     Ok(())
// }


// Пример 3

// AsyncWriteExt::write записывает буфер в программу записи, возвращая, сколько байтов было записано.

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     // Writes some prefix of the byte string, but not necessarily all of it.
//     let n = file.write(b"some bytes").await?;

//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }


// Пример 4

// AsyncWriteExt::write_all записывает весь буфер в устройство записи .

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     file.write_all(b"some bytes").await?;
//     Ok(())
// }


// Пример 5

// tokio::io::copy асинхронно копирует все содержимое программы чтения в программу записи.

// use tokio::fs::File;
// use tokio::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut reader: &[u8] = b"hello";
//     let mut file = File::create("foo.txt").await?;

//     io::copy(&mut reader, &mut file).await?;
//     Ok(())
// }


// Пример 6

// io::copy()

// use tokio::io;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let (mut rd, mut wr) = socket.split();
            
//             if io::copy(&mut rd, &mut wr).await.is_err() {
//                 eprintln!("failed to copy");
//             }
//         });
//     }
// }

// Пример 7

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}