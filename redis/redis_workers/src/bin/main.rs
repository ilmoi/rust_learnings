use redis::Connection;
use redis_workers::redis_play::test_pub_sub;
use redis_workers::redis_tut::{connect, sorted_set};

fn main() {
    println!("Hello, world!");
    let mut conn = connect();
    // basics(&mut conn);
    // hash(&mut conn);
    // list(&mut conn);
    // set(&mut conn);
    // sorted_set(&mut conn);
    // test_scan(&mut conn);
    // test_pipe(&mut conn);
    test_pub_sub(conn);
}
