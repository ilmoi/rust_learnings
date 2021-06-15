use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // // create a socket
    // let socket = TcpStream::connect("127.0.0.1:6142").await?;
    // // split into listener and receiver, since we need to pass them to diff parts of code below
    // let (mut rd, mut wr) = io::split(socket);

    // // Write data in the background
    // let write_task = tokio::spawn(async move {
    //     wr.write_all(b"hello\r\n").await?;
    //     wr.write_all(b"world\r\n").await?;
    //
    //     // Sometimes, the rust type inferencer needs
    //     // a little help
    //     Ok::<_, io::Error>(())
    // });
    //
    // let mut buf = vec![0; 128];
    //
    // // standard loop for a TCP listener
    // loop {
    //     // read the input
    //     let n = rd.read(&mut buf).await?;
    //     if n == 0 {
    //         break;
    //     }
    //     println!("GOT {:?}", &buf[..n]);
    // }

    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }

    // Ok(())
}