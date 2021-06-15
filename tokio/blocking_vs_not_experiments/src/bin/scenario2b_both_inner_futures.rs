use rand::prelude::*;
use tokio::time::Duration;
use std::future::Future;

/*
WHAT CHANGED: this time using futures instead of tokio::spawn

LEARNING: similar behavior to sc2, but slightly different:
 - similar: entire first block runs and finishes before second block begins (sync)
 - fns within each block still run async, but this time they're started in order (1>2>3>4) - whereas with tokio::spawn they were started in random order

VS EXPECTATION: didn't realize the slight diff between futures and tokio::spawn existed
 */

#[tokio::main]
pub async fn main() {
    block_one().await;
    println!("-------------");
    block_two().await;
}

// equivalent to "pull"
pub async fn block_one() {
    // equivalent to "process_user_timeline"
    let mut futs = vec![];

    for i in 1..10 {
        futs.push(async move {
            println!("Starting func #{}", i);
            i_take_random_time().await;
            println!("Ending func #{}", i);
        });
    }

    futures::future::join_all(futs).await;
}

// equivalent to "backfill"
pub async fn block_two() {
    let mut futs = vec![];

    // equivalent to "process_rt_original_tweet"
    for i in 10001..10010 {
        futs.push(async move {
            println!("Starting func #{}", i);
            i_take_random_time().await;
            println!("Ending func #{}", i);
        });
    }

    futures::future::join_all(futs).await;
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
