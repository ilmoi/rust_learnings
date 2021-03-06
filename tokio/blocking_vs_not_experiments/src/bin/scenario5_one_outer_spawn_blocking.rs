use rand::prelude::*;
use tokio::time::Duration;

/*
WHAT CHANGED: trying spawn_blocking

LEARNING: order of operations now:
1)---- line is printed first
2)second block runs
3)first block runs

todo this is weird - why does the 2nd block run before the 1st block?
 - https://stackoverflow.com/questions/67989412/unexpected-tokiotaskspawn-blocking-behavior
 - ok, so it's the other way around spawn_blocking takes something that would OTHERWISE block and offloads to a diff thread. Now makes sense.

VS EXPECTATION: makes sense given spawn_blocking offloads to a diff thread and that's all
 */

#[tokio::main]
pub async fn main() {
    let h = tokio::task::spawn_blocking(move || {
        block_one();
    });
    println!("-------------");
    let h2 = tokio::spawn(async move {
        block_two().await;
    });

    h.await.unwrap();
    h2.await.unwrap();
}

// equivalent to "pull"
#[tokio::main] //todo note that w/o this the below block wouldn't run - spawn_blocking takes a sync closure, so we can't .await on block_one
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




