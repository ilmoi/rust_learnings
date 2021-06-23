use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: trying spawn_blocking for BOTH

LEARNING: acts exactly like scenario 4 - all 20 start together > print delay > end together

VS EXPECTATION: makes sense given spawn_blocking offloads to a diff thread and that's all
 */

#[tokio::main]
pub async fn main() {
    let h = tokio::task::spawn_blocking(move || {
        block_one();
    });
    println!("-------------");
    let h2 = tokio::task::spawn_blocking(move || {
        block_two();
    });

    h.await.unwrap();
    h2.await.unwrap();
}

// equivalent to "pull"
#[tokio::main]
pub async fn block_one() {
    // equivalent to "process_user_timeline"
    let mut handles = vec![];

    for i in 1..10 {
        let h = tokio::spawn(async move {
            println!("Starting func #{}", i);
            i_take_random_time().await;
            println!("Ending func #{}", i);
        });
        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }
}

// equivalent to "backfill"
#[tokio::main]
pub async fn block_two() {
    let mut handles = vec![];

    // equivalent to "process_rt_original_tweet"
    for i in 10001..10010 {
        let h = tokio::spawn(async move {
            println!("Starting func #{}", i);
            i_take_random_time().await;
            println!("Ending func #{}", i);
        });
        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }
}

pub async fn i_take_random_time() {
    let mut delay;
    {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen(); // generates a float between 0 and 1
        delay = (y * 10.0) as u64;
        println!("Delay is: {}", delay);
    }
    tokio::time::sleep(Duration::from_secs(delay)).await;
}
