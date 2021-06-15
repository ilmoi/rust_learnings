use redis::{Commands, Connection};
use redis_workers::redis_tut::{connect, Task};
use serde_redis::RedisDeserialize;
use std::time::Duration;

/// you can start many of these, they will all be feeding off the same queue
fn main() {
    println!("ima worker");
    let mut conn = connect();
    let time_delay = 1;

    loop {
        let high = measure_task_q(&mut conn, "high");
        measure_task_q(&mut conn, "low"); //just for printing purposes

        // if there are high priority tasks do them first, then fetch again
        if high > 0 {
            execute_task(&mut conn, "high");
            std::thread::sleep(std::time::Duration::from_secs(time_delay));
            continue;
        }

        // else do low priority
        execute_task(&mut conn, "low");
        std::thread::sleep(std::time::Duration::from_secs(time_delay));
    }
}

pub fn measure_task_q(conn: &mut Connection, q_name: &str) -> isize {
    let size: isize = conn.llen(q_name).unwrap();

    println!("Task q >>{}<< has {} tasks.", q_name, size);

    size
}

pub fn execute_task(conn: &mut Connection, q_name: &str) {
    let task = conn.lpop::<&str, String>(q_name);

    if let Ok(task) = task {
        let t: Task = serde_json::from_str(&task).unwrap();
        println!("... STARTED executing task {} ...", t.id);
        std::thread::sleep(Duration::from_secs(5));
        let processor = t.processing.get_processor();
        processor(t.desc);
        println!("... FINISHED executing task {} ...", t.id);
    }
}

// BLPOP (blocks the connection when no tasks to execute)
// pub fn execute_task(conn: &mut Connection) {
//     // BLPOP comes back as Bulk datatype, which is actually Vec<String> - https://docs.rs/redis/0.20.1/redis/enum.Value.html#variant.Bulk
//     let task = conn.blpop::<&str, Vec<String>>("task_q", 120).unwrap();
//
//     // the 1st item in the Vec<> is the name of the list ("task_q") and the 2nd item is the actual item...
//     let t: Task = serde_json::from_str(&task[1]).unwrap();
//     println!("... executing task {:?} ...", t.id);
// }

// -----------------------------------------------------------------------------
// todo using serde-redis library

// pub fn execute_task(conn: &mut Connection) {
//     let task: Task = redis::cmd("LPOP")
//         .arg("task_q")
//         .query::<Task>(conn)
//         .unwrap()
//         .deserialize()
//         .unwrap();
//
//     println!("... executing task {} ...", task.id);
// }

// pub fn get_simple(conn: &mut Connection) {
//     let s: Simple = conn
//         .hgetall::<String, Simple>("simple_hash".into())
//         .unwrap()
//         .deserialize()
//         .unwrap();
//
//     println!("{:?}", s);
// }
//
// #[derive(Debug, serde::Deserialize, PartialEq)]
// struct Simple {
//     a: String,
//     b: String,
// }
