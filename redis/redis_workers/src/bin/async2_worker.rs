use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use redis::{Commands, Connection, RedisResult};
use serde_redis::RedisDeserialize;

use redis_workers::redis_tut::{connect, Task};

/*
todo: now the measure_task_q fn is also async.. makes no diff
 */

/// you can start many of these, they will all be feeding off the same queue
#[tokio::main]
pub async fn main() {
    println!("ima worker [{:?}]", thread::current().id());
    let time_delay = 1;

    let mut conn = connect();
    let arc_conn = Arc::new(Mutex::new(conn));

    loop {
        let conn_clone = arc_conn.clone();

        let high = measure_task_q(arc_conn.clone(), "high").await;
        measure_task_q(arc_conn.clone(), "low").await; //just for printing purposes

        // if there are high priority tasks do them first, then fetch again
        if high > 0 {
            let handle = tokio::spawn(async move {
                execute_task(conn_clone, "high").await;
            });
            std::thread::sleep(std::time::Duration::from_secs(time_delay));
            continue;
        }

        // else do low priority
        let handle = tokio::spawn(async move {
            execute_task(conn_clone, "low").await;
        });
        std::thread::sleep(std::time::Duration::from_secs(time_delay));
    }
}

pub async fn measure_task_q(conn_clone: Arc<Mutex<Connection>>, q_name: &str) -> isize {
    let mut conn = conn_clone.lock().unwrap();

    let size: isize = conn.llen(q_name).unwrap();

    println!(
        "Task q >>{}<< has {} tasks. [{:?}]",
        q_name,
        size,
        thread::current().id()
    );

    size
}

pub async fn execute_task(conn_clone: Arc<Mutex<Connection>>, q_name: &str) {
    let mut task: RedisResult<String>;
    // need a block here so that mutex is dropped before a call .await is done below - other we're carrying Mutex over .await which is not allowed
    {
        let mut conn = conn_clone.lock().unwrap();
        task = conn.lpop::<&str, String>(q_name);
    }

    if let Ok(task) = task {
        let t: Task = serde_json::from_str(&task).unwrap();
        println!(
            "... STARTED executing task {} ... [{:?}]",
            t.id,
            thread::current().id()
        );
        tokio::time::sleep(Duration::from_secs(10)).await;
        let processor = t.processing.get_processor();
        processor(t.desc);
        println!(
            "... FINISHED executing task {} ... [{:?}]",
            t.id,
            thread::current().id()
        );
    }
}
