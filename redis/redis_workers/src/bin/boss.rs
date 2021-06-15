use redis::Connection;
use redis_workers::redis_tut::{connect, Processing, Task};
use std::{thread, time};

/// start 1 of these
fn main() {
    println!("ima boss");
    let mut conn = connect();

    let mut i = 1;
    loop {
        create_task(&mut conn, i);
        i += 1;
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn create_task(conn: &mut Connection, i: u32) {
    let task = Task {
        id: i,
        desc: "lets do this".into(),
        processing: Processing::Type1,
    };

    // pick either a high priority q or a low priority one
    let q = if i % 2 == 0 { "high" } else { "low" };

    // serialize
    let t = serde_json::to_string(&task).unwrap();

    // send to q
    let _: () = redis::cmd("RPUSH").arg(q).arg(t).query(conn).unwrap();

    println!("Task {} queued for execution", task.id);
}
