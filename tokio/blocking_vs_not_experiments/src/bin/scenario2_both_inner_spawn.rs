use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: this time fns inside blocks use tokio::spawn

LEARNING: the blocks are synchronous to each other, but fns within blocks are async

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
    let mut handles = vec![];

    for i in 1..10 {
        let h = tokio::spawn(async move {
            println!("Starting func #{}", i);
            i_take_random_time().await;
            println!("Ending func #{}", i);
        });
        handles.push(h);
    }

    //todo important - w/o this it starts all 20 tasks and never waits for them to finish
    for h in handles {
        h.await.unwrap();
    }
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

    //todo also important - if we join the first block of handles, but not the second block, the second block never gets to even run
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
