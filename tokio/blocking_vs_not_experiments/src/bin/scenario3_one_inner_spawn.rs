use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: running first block without tokio::spawn, but second block with

LEARNING: the first block runs completely syncronously (both within the block, and blocking the next block) - the second one has all its functions running async

VS EXPECTATION: as expected
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
    // let mut handles = vec![];

    for i in 1..10 {
        // let h = tokio::spawn(async move {
        println!("Starting func #{}", i);
        i_take_random_time().await;
        println!("Ending func #{}", i);
        // });
        // handles.push(h);
    }

    // for h in handles {
    //     h.await.unwrap();
    // }
}

// equivalent to "backfill"
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
