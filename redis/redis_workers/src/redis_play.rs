use crate::redis_tut::{connect, sorted_set};
use redis::Connection;
use std::sync::{Arc, Mutex};

pub fn test_scan(conn: &mut Connection) {
    // populate the list
    for i in 1..100 {
        let _: () = redis::cmd("SADD")
            .arg("simple_set")
            .arg(i)
            .query(conn)
            .unwrap();
    }
    //scan through it
    let mut iter: redis::Iter<isize> = redis::cmd("SSCAN")
        .arg("simple_set")
        .cursor_arg(0)
        .clone()
        .iter(conn)
        .unwrap();
    for x in iter {
        println!("{}", x);
    }
}

pub fn test_pipe(conn: &mut Connection) {
    let (k1, k2): (i32, i32) = redis::pipe()
        .atomic()
        .cmd("SET")
        .arg("key_1")
        .arg(42)
        .ignore()
        .cmd("SET")
        .arg("key_2")
        .arg(43)
        .ignore()
        .cmd("GET")
        .arg("key_1")
        .cmd("GET")
        .arg("key_2")
        .query(conn)
        .unwrap();
}

pub fn test_pub_sub(mut conn: Connection) {
    let mut conn = Arc::new(Mutex::new(conn));
    // clone BEFORE sending to the thread
    let mut conn_clone = conn.clone();

    let handle = std::thread::spawn(move || {
        //unlock BEFORE we can do anything
        let mut conn = conn_clone.lock().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe("channel_1").unwrap();

        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            println!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    });

    // todo this won't work coz first thread is holding the mutex - but then to be fair I don't even need threading here, worker = separate process
    // let mut conn_clone2 = conn.clone();
    // let mut conn2 = conn_clone2.lock().unwrap();
    //
    // let _: () = redis::cmd("publish")
    //     .arg("channel_1")
    //     .arg(42)
    //     .query(&mut *conn2)
    //     .unwrap();

    let mut conn2 = connect();

    let _: () = redis::cmd("publish")
        .arg("channel_1")
        .arg(42)
        .query(&mut conn2)
        .unwrap();

    handle.join();
}
