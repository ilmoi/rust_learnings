// An async client using Channels

use tokio::sync::{mpsc, oneshot};
use bytes::Bytes;
use mini_redis::client;

// address where to respond
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

// this is the message we're passing around
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // todo
    //  problem: we have a single Client, but we want to use it simultaneously across multiple tasks
    //  solution: use channels. A single task will be handling the Client - and all other tasks will send msgs to that one task

    // ----------------------------------------------------------------------------- create channel

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // ----------------------------------------------------------------------------- spawn manager task

    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    // ----------------------------------------------------------------------------- send msgs from multiple tasks

    //spawn async task
    let t1 = tokio::spawn(async move {
        // spawn oweshot channel
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx, //pass down the Sender, keep the Receiver
        };

        // Send the GET request
        tx.send(cmd).await.unwrap();

        // Await the response on the Receiver
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: b"bar".to_vec(),
            resp: resp_tx,
        };

        // Send the SET request
        tx2.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    // ----------------------------------------------------------------------------- await join handles
    // prevents the process from closing until these 3 are done

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}

