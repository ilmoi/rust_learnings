use redis::{Commands, Connection};
use redis_workers::redis_tut::{connect, Task};
use serde_redis::RedisDeserialize;
use std::time::Duration;

/// you can start many of these, they will all be feeding off the same queue
fn main() {
    println!("ima worker");
    let mut conn = connect();
    let mut pubsub = conn.as_pubsub();
    pubsub.subscribe("channel_1").unwrap();

    let time_delay = 1;

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);

        execute_task(payload);
        std::thread::sleep(std::time::Duration::from_secs(time_delay));
    }
}

pub fn execute_task(task: String) {
    let t: Task = serde_json::from_str(&task).unwrap();
    println!("... STARTED executing task {} ...", t.id);
    std::thread::sleep(Duration::from_secs(5));
    let processor = t.processing.get_processor();
    processor(t.desc);
    println!("... FINISHED executing task {} ...", t.id);
}
