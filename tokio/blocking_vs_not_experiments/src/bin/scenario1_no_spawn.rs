use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: this is the first run - not using any tokio::spawn blocks

LEARNING: the below runs 100% synchronously, ie blocks are sync in relation to each other, and all the fns inside each block are sync too

VS EXPECTATION: as expected
 */

#[tokio::main]
pub async fn main() {
    block_one().await;
    block_two().await;
}

// equivalent to "pull"
pub async fn block_one() {
    // equivalent to "process_user_timeline"
    for i in 1..10 {
        println!("Starting func #{}", i);
        i_take_random_time().await;
        println!("Ending func #{}", i);
    }
}

// equivalent to "backfill"
pub async fn block_two() {
    // equivalent to "process_rt_original_tweet"
    for i in 10001..10010 {
        println!("Starting func #{}", i);
        i_take_random_time().await;
        println!("Ending func #{}", i);
    }
}

pub async fn i_take_random_time() {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    let delay: u64 = (y * 10.0) as u64;
    println!("Delay is: {}", delay);

    tokio::time::sleep(Duration::from_secs(delay)).await;
}
