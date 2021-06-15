use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: this time not only fns inside the blocks are spawned with tokio::spawn, but also the blocks themselves

LEARNING: 100% async - ie 20 fns start all together > all 20 print their delay > all 20 end

VS EXPECTATION: as expected
 */

#[tokio::main]
pub async fn main() {
    let h = tokio::spawn(async move {
        block_one().await;
    });
    println!("-------------");
    let h2 = tokio::spawn(async move {
        block_two().await;
    });

    // todo important or it exits before the fns finish
    h.await.unwrap();
    h2.await.unwrap();
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
