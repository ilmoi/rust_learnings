use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: trying block_on - have to use it on both since can no longer use #[tokio::main]

LEARNING: behaves exactly like 2 - blocks are async within, but sync between themselves
- another interesting learning is that in this case we didn't have to collect / wait for handles

VS EXPECTATION: as expected
 */

// #[tokio::main] todo can no longer use this otherwise get a "runtime inside a runtime" error
pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        block_one().await;
    });

    println!("-------------");

    rt.block_on(async move {
        block_two().await;
    });
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
