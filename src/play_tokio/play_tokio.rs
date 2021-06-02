// https://www.youtube.com/watch?v=4DqP57BHaXI

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};

pub async fn main() {
    let listener = TcpListener::bind("localhost:8081").await.unwrap();
    // let (mut socket, _addr) = listener.accept().await.unwrap();

    // -----------------------------------------------------------------------------
    // naive version (single msg, single client)
    // let mut buffer = [0u8; 1024];
    // let bytes_read = socket.read(&mut buffer).await.unwrap();
    //write_all takes all the bytes inside the buffer and writes them to the socket
    // socket.write_all(&buffer[..bytes_read]).await.unwrap();

    // -----------------------------------------------------------------------------
    // more idiomatic version (many msgs, single client)
    // tokio has a split method to separate reading from writing part
    // let (read, mut write) = socket.split();
    //
    // let mut reader = BufReader::new(read);
    // let mut line = String::new();
    //
    // loop {
    //     let bytes_read = reader.read_line(&mut line).await.unwrap();
    //
    //     //if no more bytes, means the client has disconnected
    //     if bytes_read == 0 {
    //         break;
    //     }
    //
    //     write.write_all(line.as_bytes()).await.unwrap();
    //     line.clear(); //read_line doesn't clean the buffer so we need to do it manually
    // }

    // -----------------------------------------------------------------------------
    // proper async (multiple msgs, multiple clients)
    // the above code is not blocking at thread level, but it is blocking at task level
    // we use tokio::spawn to get around that

    // when to use tokio::spawn vs tokio::select? []
    // - so spawn creates 2 tasks, select does everything inside of 1 task
    // - use select when you have things that need to operate on the same shared state and you have a FINITE nr of things
    // - if you wanted shared state split between diff tokio::spawn instances you'd have to put it behind atomics / mutex
    // - select also provides out of the box locking, which you don't need to manually implement - one select block is auto blocking another
    //

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                //if no more bytes, means the client has disconnected
                if bytes_read == 0 {
                    break;
                }

                write.write_all(line.as_bytes()).await.unwrap();
                line.clear(); //read_line doesn't clean the buffer so we need to do it manually
            }
        });
    }

    //todo stopped writing code at around 34 min
}